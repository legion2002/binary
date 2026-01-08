use crate::contract::ContractClient;
use crate::db::Database;
use crate::tempo_orderbook::{OrderbookInfo, calculate_probability_from_orderbooks, PATH_USD_ADDRESS};
use alloy::primitives::{Address, FixedBytes};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub contract_client: Arc<ContractClient>,
    pub db: Arc<Database>,
    pub multiverse_address: Address,
    pub oracle_address: Address,
}

#[derive(Debug, Deserialize)]
pub struct GetMarketsQuery {
    /// Pagination: limit (default: 50, max: 100)
    #[serde(default = "default_limit")]
    pub limit: i64,

    /// Pagination: offset (default: 0)
    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 {
    50
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketResponse {
    pub market_hash: String,
    pub question_hash: String,
    pub question: Option<String>,
    pub resolution_deadline: i64,
    pub oracle: String,
    pub block_number: i64,
    // Probability derived from orderbook prices (0.0 - 1.0)
    pub yes_probability: Option<f64>,
    pub no_probability: Option<f64>,
    // Resolution status: "UNRESOLVED", "YES", "NO", "EVEN"
    pub resolution: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GetMarketsResponse {
    pub markets: Vec<MarketResponse>,
    pub count: usize,
    pub total: i64,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// GET /markets
/// Query parameters:
/// - limit: maximum number of markets to return (default: 50, max: 100)
/// - offset: number of markets to skip for pagination (default: 0)
pub async fn get_markets(
    State(state): State<AppState>,
    Query(params): Query<GetMarketsQuery>,
) -> impl IntoResponse {
    // Enforce max limit
    let limit = params.limit.min(100);

    // Fetch markets from database
    let markets = match state.db.get_markets_paginated(limit, params.offset).await {
        Ok(markets) => markets,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to fetch markets: {}", e),
                }),
            )
                .into_response();
        }
    };

    // Get total count
    let total = match state.db.get_markets_count().await {
        Ok(total) => total,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to get total count: {}", e),
                }),
            )
                .into_response();
        }
    };

    // Convert to response format
    // Note: probability and resolution are None in list view for performance
    // Use the detail endpoint (/markets/:marketHash) for full data
    let market_responses: Vec<MarketResponse> = markets
        .into_iter()
        .map(|m| MarketResponse {
            market_hash: m.market_hash,
            question_hash: m.question_hash,
            question: m.question_text,
            resolution_deadline: m.resolution_deadline,
            oracle: m.oracle,
            block_number: m.block_number,
            yes_probability: None,
            no_probability: None,
            resolution: None,
        })
        .collect();

    let count = market_responses.len();

    (
        StatusCode::OK,
        Json(GetMarketsResponse {
            markets: market_responses,
            count,
            total,
        }),
    )
        .into_response()
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerseTokenResponse {
    pub asset: String,
    pub yes_verse: String,
    pub no_verse: String,
    pub transaction_hash: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketDetailResponse {
    pub market_hash: String,
    pub question_hash: String,
    pub question: Option<String>,
    pub resolution_deadline: i64,
    pub oracle: String,
    pub block_number: i64,
    pub verse_tokens: Vec<VerseTokenResponse>,
    pub orderbooks: Vec<OrderbookInfo>,
    // Probability derived from orderbook prices (0.0 - 1.0)
    pub yes_probability: Option<f64>,
    pub no_probability: Option<f64>,
    // Resolution status: "UNRESOLVED", "YES", "NO", "EVEN"
    pub resolution: Option<String>,
}

/// GET /markets/:marketHash
/// Get a single market by its hash with live orderbook data
pub async fn get_market(
    State(state): State<AppState>,
    Path(market_hash): Path<String>,
) -> impl IntoResponse {
    // Fetch market from database
    let market = match state.db.get_market(&market_hash).await {
        Ok(Some(market)) => market,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: format!("Market not found: {}", market_hash),
                }),
            )
                .into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to fetch market: {}", e),
                }),
            )
                .into_response();
        }
    };

    // Fetch verse tokens for this market
    let verse_tokens_db = match state.db.get_verse_tokens_for_market(&market_hash).await {
        Ok(tokens) => tokens,
        Err(e) => {
            tracing::warn!("Failed to fetch verse tokens for market {}: {}", market_hash, e);
            Vec::new()
        }
    };

    let verse_tokens: Vec<VerseTokenResponse> = verse_tokens_db
        .iter()
        .map(|t| VerseTokenResponse {
            asset: t.asset.clone(),
            yes_verse: t.yes_verse.clone(),
            no_verse: t.no_verse.clone(),
            transaction_hash: t.transaction_hash.clone(),
        })
        .collect();

    // Parse market hash for contract calls
    let market_hash_bytes: Option<FixedBytes<32>> = market_hash
        .strip_prefix("0x")
        .and_then(|s| hex::decode(s).ok())
        .and_then(|bytes| <Vec<u8> as TryInto<[u8; 32]>>::try_into(bytes).ok())
        .map(FixedBytes::from);

    // Fetch live orderbook data and resolution
    let mut orderbooks: Vec<OrderbookInfo> = Vec::new();
    let mut resolution: Option<String> = None;

    if let Some(hash_bytes) = market_hash_bytes {
        // Fetch resolution from contract
        match state.contract_client.get_resolution(hash_bytes).await {
            Ok(res) => resolution = Some(res),
            Err(e) => tracing::warn!("Failed to fetch resolution: {}", e),
        }

        // Fetch live orderbook data for each verse token pair
        for verse in &verse_tokens_db {
            // Parse addresses
            let yes_addr: Option<Address> = verse.yes_verse
                .parse()
                .ok();
            let no_addr: Option<Address> = verse.no_verse
                .parse()
                .ok();
            let asset_addr: Option<Address> = verse.asset
                .parse()
                .ok();

            if let (Some(yes), Some(no), Some(_asset)) = (yes_addr, no_addr, asset_addr) {
                // Fetch live orderbook state from Tempo DEX
                match state.contract_client.get_market_orderbooks(hash_bytes, yes, PATH_USD_ADDRESS).await {
                    Ok(mut live_orderbooks) => {
                        // Update pair types
                        for ob in &mut live_orderbooks {
                            if ob.base_token.contains(&format!("{:?}", yes)) {
                                ob.pair_type = "YES/QUOTE".to_string();
                            }
                        }
                        orderbooks.extend(live_orderbooks);
                    }
                    Err(e) => tracing::debug!("No orderbook for YES token: {}", e),
                }

                match state.contract_client.get_market_orderbooks(hash_bytes, no, PATH_USD_ADDRESS).await {
                    Ok(mut live_orderbooks) => {
                        for ob in &mut live_orderbooks {
                            if ob.base_token.contains(&format!("{:?}", no)) {
                                ob.pair_type = "NO/QUOTE".to_string();
                            }
                        }
                        orderbooks.extend(live_orderbooks);
                    }
                    Err(e) => tracing::debug!("No orderbook for NO token: {}", e),
                }
            }
        }
    }

    // Fall back to database orderbook info if live fetch failed
    if orderbooks.is_empty() {
        orderbooks = match state.db.get_orderbooks_for_market(&market_hash).await {
            Ok(orderbook_data) => {
                orderbook_data
                    .into_iter()
                    .flat_map(|data| {
                        let mut infos = Vec::new();
                        if let Some(pair_key) = data.yes_pair_key {
                            infos.push(OrderbookInfo {
                                pair_type: "YES/QUOTE".to_string(),
                                pair_key,
                                base_token: format!("YES_{}", data.asset_address),
                                quote_token: data.quote_token_address.clone(),
                                best_bid_tick: None,
                                best_ask_tick: None,
                                best_bid_price: None,
                                best_ask_price: None,
                                mid_price: None,
                                spread_bps: None,
                            });
                        }
                        if let Some(pair_key) = data.no_pair_key {
                            infos.push(OrderbookInfo {
                                pair_type: "NO/QUOTE".to_string(),
                                pair_key,
                                base_token: format!("NO_{}", data.asset_address),
                                quote_token: data.quote_token_address,
                                best_bid_tick: None,
                                best_ask_tick: None,
                                best_bid_price: None,
                                best_ask_price: None,
                                mid_price: None,
                                spread_bps: None,
                            });
                        }
                        infos
                    })
                    .collect()
            }
            Err(e) => {
                tracing::warn!("Failed to fetch orderbooks: {}", e);
                Vec::new()
            }
        };
    }

    // Calculate probability from orderbook data
    let (yes_prob, no_prob) = calculate_probability_from_orderbooks(&orderbooks);
    let (yes_probability, no_probability) = if yes_prob == 0.5 && no_prob == 0.5 && orderbooks.iter().all(|o| o.mid_price.is_none()) {
        // No price data available
        (None, None)
    } else {
        (Some(yes_prob), Some(no_prob))
    };

    (
        StatusCode::OK,
        Json(MarketDetailResponse {
            market_hash: market.market_hash,
            question_hash: market.question_hash,
            question: market.question_text,
            resolution_deadline: market.resolution_deadline,
            oracle: market.oracle,
            block_number: market.block_number,
            verse_tokens,
            orderbooks,
            yes_probability,
            no_probability,
            resolution,
        }),
    )
        .into_response()
}

/// GET /markets/:marketHash/verse-tokens
/// Get all verse tokens for a specific market
pub async fn get_verse_tokens(
    State(state): State<AppState>,
    Path(market_hash): Path<String>,
) -> impl IntoResponse {
    // Verify market exists
    match state.db.get_market(&market_hash).await {
        Ok(Some(_)) => {}
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: format!("Market not found: {}", market_hash),
                }),
            )
                .into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to fetch market: {}", e),
                }),
            )
                .into_response();
        }
    }

    // Fetch verse tokens
    let verse_tokens: Vec<VerseTokenResponse> = match state.db.get_verse_tokens_for_market(&market_hash).await {
        Ok(tokens) => tokens
            .into_iter()
            .map(|t| VerseTokenResponse {
                asset: t.asset,
                yes_verse: t.yes_verse,
                no_verse: t.no_verse,
                transaction_hash: t.transaction_hash,
            })
            .collect(),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to fetch verse tokens: {}", e),
                }),
            )
                .into_response();
        }
    };

    (StatusCode::OK, Json(verse_tokens)).into_response()
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigResponse {
    pub multiverse_address: String,
    pub oracle_address: String,
}

/// GET /config
/// Returns contract addresses and other configuration needed by the frontend
pub async fn get_config(State(state): State<AppState>) -> impl IntoResponse {
    let config = ConfigResponse {
        multiverse_address: format!("{:?}", state.multiverse_address),
        oracle_address: format!("{:?}", state.oracle_address),
    };
    (StatusCode::OK, Json(config))
}
