use crate::contract::ContractClient;
use crate::db::Database;
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
    let market_responses: Vec<MarketResponse> = markets
        .into_iter()
        .map(|m| MarketResponse {
            market_hash: m.market_hash,
            question_hash: m.question_hash,
            question: m.question_text,
            resolution_deadline: m.resolution_deadline,
            oracle: m.oracle,
            block_number: m.block_number,
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
}

/// GET /markets/:marketHash
/// Get a single market by its hash
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
    let verse_tokens = match state.db.get_verse_tokens_for_market(&market_hash).await {
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
            tracing::warn!("Failed to fetch verse tokens for market {}: {}", market_hash, e);
            Vec::new()
        }
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
