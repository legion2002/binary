use crate::contract::ContractClient;
use crate::db::Database;
use crate::tempo_orderbook::{OrderbookInfo, PATH_USD_ADDRESS};
use alloy::primitives::{Address, FixedBytes};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json, Response},
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

    /// Optional quote token address for orderbook trading (defaults to pathUSD)
    pub quote_token: Option<String>,
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
    pub orderbooks: Vec<OrderbookInfo>,
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
) -> Response {
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
    for &asset in &assets {
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

    // Get orderbook info for created verse tokens
    // Parse quote token address (defaults to pathUSD)
    let quote_token = request.quote_token
        .as_ref()
        .and_then(|addr| addr.parse::<Address>().ok())
        .unwrap_or(PATH_USD_ADDRESS);

    let mut orderbooks = Vec::new();
    for &asset in &assets {
        // Try to get orderbook info for each asset's verse tokens
        match state
            .contract_client
            .get_market_orderbooks(market_hash, asset, quote_token)
            .await
        {
            Ok(infos) => {
                // Store orderbook info in database
                if let Err(e) = state
                    .db
                    .insert_orderbook_market(
                        market_hash,
                        asset,
                        quote_token,
                        infos.iter().find(|i| i.pair_type == "YES/QUOTE").map(|i| i.pair_key.clone()),
                        infos.iter().find(|i| i.pair_type == "NO/QUOTE").map(|i| i.pair_key.clone()),
                    )
                    .await
                {
                    tracing::error!("Failed to store orderbook info in database: {}", e);
                }
                orderbooks.extend(infos);
            }
            Err(e) => {
                tracing::debug!("No orderbook info available for asset {:?}: {}", asset, e);
            }
        }
    }

    tracing::info!(
        "Market created with {} orderbook pairs",
        orderbooks.len()
    );

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
            orderbooks,
        }),
    )
        .into_response()
}

// ============================================================================
// Add Market Endpoint - Add an existing on-chain market to the database
// ============================================================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMarketRequest {
    /// The market hash (0x-prefixed hex string)
    pub market_hash: String,

    /// The question text - must match the on-chain questionHash
    pub question: String,

    /// Optional quote token address for orderbook trading (defaults to pathUSD)
    pub quote_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMarketVerseToken {
    pub asset: String,
    pub yes_verse: String,
    pub no_verse: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMarketResponse {
    pub market_hash: String,
    pub question_hash: String,
    pub question: String,
    pub resolution_deadline: u32,
    pub oracle: String,
    pub resolution: String,
    pub verse_tokens: Vec<AddMarketVerseToken>,
    pub orderbooks: Vec<OrderbookInfo>,
}

/// POST /admin/markets/add
/// Adds an existing on-chain market to the database
pub async fn add_market(
    State(state): State<AdminState>,
    Json(request): Json<AddMarketRequest>,
) -> Response {
    // Parse market hash
    let market_hash_str = request.market_hash.strip_prefix("0x").unwrap_or(&request.market_hash);
    let market_hash_bytes = match hex::decode(market_hash_str) {
        Ok(bytes) if bytes.len() == 32 => bytes,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Invalid market hash format. Expected 0x-prefixed 32-byte hex string".to_string(),
                }),
            )
                .into_response();
        }
    };
    let market_hash: FixedBytes<32> = FixedBytes::from_slice(&market_hash_bytes);

    // Validate question is not empty
    if request.question.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Question cannot be empty".to_string(),
            }),
        )
            .into_response();
    }

    tracing::info!(
        "Adding existing market: market_hash=0x{}, question='{}'",
        hex::encode(market_hash),
        request.question
    );

    // Fetch market data from chain
    let market_info = match state.contract_client.get_market(market_hash).await {
        Ok(info) => info,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to fetch market from chain: {}", e),
                }),
            )
                .into_response();
        }
    };

    // Check if market exists (resolution != NULL)
    if market_info.resolution == "NULL" {
        return (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Market does not exist on-chain".to_string(),
            }),
        )
            .into_response();
    }

    // Validate question text matches on-chain questionHash
    let calculated_question_hash = ContractClient::calculate_question_hash(&request.question);
    let expected_question_hash = format!("0x{}", hex::encode(calculated_question_hash));
    if expected_question_hash != market_info.question_hash {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!(
                    "Question text does not match on-chain questionHash. Expected: {}, Got: {}",
                    market_info.question_hash, expected_question_hash
                ),
            }),
        )
            .into_response();
    }

    // Parse oracle address from market info
    let oracle_address = match market_info.oracle.parse::<Address>() {
        Ok(addr) => addr,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to parse oracle address from chain".to_string(),
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
            calculated_question_hash,
            request.question.clone(),
            market_info.resolution_deadline,
            oracle_address,
        )
        .await
    {
        tracing::error!("Failed to store market in database: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to store market in database: {}", e),
            }),
        )
            .into_response();
    }

    // Query VerseTokensCreated events to discover existing verse tokens
    let verse_events = match state
        .contract_client
        .get_verse_tokens_created_events(market_hash)
        .await
    {
        Ok(events) => events,
        Err(e) => {
            tracing::warn!("Failed to query verse token events: {}", e);
            Vec::new()
        }
    };

    // Store verse tokens and fetch orderbook info
    let quote_token = request.quote_token
        .as_ref()
        .and_then(|addr| addr.parse::<Address>().ok())
        .unwrap_or(PATH_USD_ADDRESS);

    let mut verse_tokens = Vec::new();
    let mut orderbooks = Vec::new();

    for (asset, yes_verse, no_verse) in verse_events {
        // Store verse tokens in database
        if let Err(e) = state
            .db
            .insert_verse_tokens(market_hash, asset, yes_verse, no_verse, None)
            .await
        {
            tracing::error!("Failed to store verse tokens in database: {}", e);
        }

        verse_tokens.push(AddMarketVerseToken {
            asset: format!("{:?}", asset),
            yes_verse: format!("{:?}", yes_verse),
            no_verse: format!("{:?}", no_verse),
        });

        // Fetch orderbook info for this asset
        match state
            .contract_client
            .get_market_orderbooks(market_hash, asset, quote_token)
            .await
        {
            Ok(infos) => {
                // Store orderbook info in database
                if let Err(e) = state
                    .db
                    .insert_orderbook_market(
                        market_hash,
                        asset,
                        quote_token,
                        infos.iter().find(|i| i.pair_type == "YES/QUOTE").map(|i| i.pair_key.clone()),
                        infos.iter().find(|i| i.pair_type == "NO/QUOTE").map(|i| i.pair_key.clone()),
                    )
                    .await
                {
                    tracing::error!("Failed to store orderbook info in database: {}", e);
                }
                orderbooks.extend(infos);
            }
            Err(e) => {
                tracing::debug!("No orderbook info available for asset {:?}: {}", asset, e);
            }
        }
    }

    tracing::info!(
        "Market added with {} verse tokens and {} orderbook pairs",
        verse_tokens.len(),
        orderbooks.len()
    );

    // Return success response
    (
        StatusCode::OK,
        Json(AddMarketResponse {
            market_hash: market_info.market_hash,
            question_hash: market_info.question_hash,
            question: request.question,
            resolution_deadline: market_info.resolution_deadline,
            oracle: market_info.oracle,
            resolution: market_info.resolution,
            verse_tokens,
            orderbooks,
        }),
    )
        .into_response()
}
