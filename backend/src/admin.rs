use crate::contract::ContractClient;
use crate::db::Database;
use crate::uniswap_v4::{V4PoolInfo, PoolConfig};
use alloy::primitives::{Address, FixedBytes};
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

    /// Optional V4 pool configuration (which pools to create)
    pub v4_pool_config: Option<PoolConfig>,
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
    pub v4_pools: Vec<V4PoolInfo>,
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

    // Create V4 pools if we have at least 2 assets
    let mut v4_pools = Vec::new();
    if assets.len() >= 2 {
        // Use the first two assets as token0 and token1
        let token0_address = assets[0];
        let token1_address = assets[1];

        // Try to create V4 pools
        match state
            .contract_client
            .initialize_v4_pools(
                market_hash,
                token0_address,
                token1_address,
                request.v4_pool_config,
            )
            .await
        {
            Ok(created_pools) => {
                // Store pool IDs in database
                if let Err(e) = state
                    .db
                    .insert_v4_pools(
                        market_hash,
                        created_pools.yes0_yes1,
                        created_pools.no0_no1,
                        created_pools.yes0_no0,
                        created_pools.yes1_no1,
                        token0_address,
                        token1_address,
                        Some(format!("{:?}", tx_hash)),
                    )
                    .await
                {
                    tracing::error!("Failed to store V4 pools in database: {}", e);
                }

                // Build pool info for response
                if let Some(pool_id) = created_pools.yes0_yes1 {
                    v4_pools.push(V4PoolInfo {
                        pool_type: "YES_TOKEN0/YES_TOKEN1".to_string(),
                        pool_id: format!("0x{}", hex::encode(pool_id)),
                        token0: format!("YES_{:?}", token0_address),
                        token1: format!("YES_{:?}", token1_address),
                        fee: 3000,
                        tick_spacing: 60,
                        liquidity: None,
                        sqrt_price_x96: None,
                        tick: None,
                    });
                }

                if let Some(pool_id) = created_pools.no0_no1 {
                    v4_pools.push(V4PoolInfo {
                        pool_type: "NO_TOKEN0/NO_TOKEN1".to_string(),
                        pool_id: format!("0x{}", hex::encode(pool_id)),
                        token0: format!("NO_{:?}", token0_address),
                        token1: format!("NO_{:?}", token1_address),
                        fee: 3000,
                        tick_spacing: 60,
                        liquidity: None,
                        sqrt_price_x96: None,
                        tick: None,
                    });
                }

                if let Some(pool_id) = created_pools.yes0_no0 {
                    v4_pools.push(V4PoolInfo {
                        pool_type: "YES_TOKEN0/NO_TOKEN0".to_string(),
                        pool_id: format!("0x{}", hex::encode(pool_id)),
                        token0: format!("YES_{:?}", token0_address),
                        token1: format!("NO_{:?}", token0_address),
                        fee: 3000,
                        tick_spacing: 60,
                        liquidity: None,
                        sqrt_price_x96: None,
                        tick: None,
                    });
                }

                if let Some(pool_id) = created_pools.yes1_no1 {
                    v4_pools.push(V4PoolInfo {
                        pool_type: "YES_TOKEN1/NO_TOKEN1".to_string(),
                        pool_id: format!("0x{}", hex::encode(pool_id)),
                        token0: format!("YES_{:?}", token1_address),
                        token1: format!("NO_{:?}", token1_address),
                        fee: 3000,
                        tick_spacing: 60,
                        liquidity: None,
                        sqrt_price_x96: None,
                        tick: None,
                    });
                }

                tracing::info!(
                    "Created {} V4 pools for market {}",
                    v4_pools.len(),
                    hex::encode(market_hash)
                );
            }
            Err(e) => {
                tracing::warn!("Failed to create V4 pools: {}. Market creation continues.", e);
                // Don't fail the entire market creation if V4 pools fail
            }
        }
    } else if !assets.is_empty() {
        tracing::info!("V4 pools require at least 2 assets, only {} provided", assets.len());
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
            v4_pools,
        }),
    )
        .into_response()
}
