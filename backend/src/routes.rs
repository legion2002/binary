use crate::contract::ContractClient;
use crate::db::Database;
use axum::{
    extract::{Query, State},
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
