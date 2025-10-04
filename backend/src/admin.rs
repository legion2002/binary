use crate::contract::ContractClient;
use crate::db::Database;
use alloy::primitives::Address;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct AdminState {
    pub contract_client: Arc<ContractClient>,
    pub db: Arc<Database>,
    pub oracle_address: Address,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenMarketRequest {
    /// The question text (e.g., "Will FOCIL be included in Ethereum by end of 2025?")
    pub question: String,

    /// Unix timestamp for when the market should resolve
    pub resolution_deadline: u32,

    /// Optional list of asset addresses to create verse tokens for
    pub assets: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerseTokenInfo {
    pub asset: String,
    pub yes_verse: String,
    pub no_verse: String,
    pub transaction_hash: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenMarketResponse {
    pub market_hash: String,
    pub question_hash: String,
    pub question: String,
    pub resolution_deadline: u32,
    pub oracle: String,
    pub transaction_hash: String,
    pub verse_tokens: Vec<VerseTokenInfo>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// POST /admin/markets/open
/// Opens a new prediction market
pub async fn open_market(
    State(state): State<AdminState>,
    Json(request): Json<OpenMarketRequest>,
) -> impl IntoResponse {
    // Validate question
    if request.question.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Question cannot be empty".to_string(),
            }),
        )
            .into_response();
    }

    if request.question.len() < 10 {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Question must be at least 10 characters long".to_string(),
            }),
        )
            .into_response();
    }

    // Validate deadline is in the future
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;

    if request.resolution_deadline <= now {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Resolution deadline must be in the future".to_string(),
            }),
        )
            .into_response();
    }

    // Parse asset addresses if provided
    let assets: Vec<Address> = if let Some(assets_str) = &request.assets {
        let mut parsed_assets = Vec::new();
        for asset_str in assets_str {
            match asset_str.parse::<Address>() {
                Ok(addr) => parsed_assets.push(addr),
                Err(_) => {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(ErrorResponse {
                            error: format!("Invalid asset address: {}", asset_str),
                        }),
                    )
                        .into_response();
                }
            }
        }
        parsed_assets
    } else {
        Vec::new()
    };

    // Calculate hashes
    let question_hash = ContractClient::calculate_question_hash(&request.question);
    let market_hash = ContractClient::calculate_market_hash(
        question_hash,
        request.resolution_deadline,
        state.oracle_address,
    );

    tracing::info!(
        "Opening market: question='{}', question_hash=0x{}, market_hash=0x{}",
        request.question,
        hex::encode(question_hash),
        hex::encode(market_hash)
    );

    // Open the market on-chain
    let tx_hash = match state
        .contract_client
        .open_market(question_hash, request.resolution_deadline, state.oracle_address)
        .await
    {
        Ok(hash) => hash,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to open market on-chain: {}", e),
                }),
            )
                .into_response();
        }
    };

    // Store market in database
    if let Err(e) = state
        .db
        .insert_market_with_question(
            market_hash,
            question_hash,
            request.question.clone(),
            request.resolution_deadline,
            state.oracle_address,
        )
        .await
    {
        tracing::error!("Failed to store market in database: {}", e);
        // Continue anyway - the market is already on-chain
    }

    // Create verse tokens for each asset
    let mut verse_tokens = Vec::new();
    for asset in assets {
        match state
            .contract_client
            .create_verse_tokens(asset, market_hash)
            .await
        {
            Ok((yes_verse, no_verse, verse_tx_hash)) => {
                let verse_info = VerseTokenInfo {
                    asset: format!("{:?}", asset),
                    yes_verse: format!("{:?}", yes_verse),
                    no_verse: format!("{:?}", no_verse),
                    transaction_hash: format!("{:?}", verse_tx_hash),
                };

                // Store verse tokens in database
                if let Err(e) = state
                    .db
                    .insert_verse_tokens(
                        market_hash,
                        asset,
                        yes_verse,
                        no_verse,
                        Some(format!("{:?}", verse_tx_hash)),
                    )
                    .await
                {
                    tracing::error!("Failed to store verse tokens in database: {}", e);
                }

                verse_tokens.push(verse_info);
            }
            Err(e) => {
                tracing::error!("Failed to create verse tokens for asset {:?}: {}", asset, e);
                // Return partial success - market was opened but some verse token creations failed
                return (
                    StatusCode::PARTIAL_CONTENT,
                    Json(ErrorResponse {
                        error: format!(
                            "Market opened successfully but failed to create verse tokens for asset {:?}: {}",
                            asset, e
                        ),
                    }),
                )
                    .into_response();
            }
        }
    }

    // Return success response
    (
        StatusCode::OK,
        Json(OpenMarketResponse {
            market_hash: format!("0x{}", hex::encode(market_hash)),
            question_hash: format!("0x{}", hex::encode(question_hash)),
            question: request.question,
            resolution_deadline: request.resolution_deadline,
            oracle: format!("{:?}", state.oracle_address),
            transaction_hash: format!("{:?}", tx_hash),
            verse_tokens,
        }),
    )
        .into_response()
}
