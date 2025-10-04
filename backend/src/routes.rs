use crate::contract::{ContractClient, MarketInfo};
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
}

#[derive(Debug, Deserialize)]
pub struct GetMarketsQuery {
    /// Filter markets by status: "open", "closed", or "all" (default)
    #[serde(default = "default_status")]
    pub status: String,

    /// Comma-separated list of market hashes to query
    /// If not provided, returns empty list (requires event indexing for full list)
    pub market_hashes: Option<String>,
}

fn default_status() -> String {
    "all".to_string()
}

#[derive(Debug, Serialize)]
pub struct GetMarketsResponse {
    pub markets: Vec<MarketInfo>,
    pub count: usize,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// GET /markets
/// Query parameters:
/// - status: "open", "closed", or "all" (default: "all")
/// - market_hashes: comma-separated list of market hashes (hex strings with 0x prefix)
pub async fn get_markets(
    State(state): State<AppState>,
    Query(params): Query<GetMarketsQuery>,
) -> impl IntoResponse {
    // Parse market hashes from query parameter
    let market_hashes = match &params.market_hashes {
        Some(hashes_str) => {
            let mut hashes = Vec::new();
            for hash_str in hashes_str.split(',') {
                let hash_str = hash_str.trim();
                if hash_str.is_empty() {
                    continue;
                }

                // Remove 0x prefix if present
                let hash_str = hash_str.strip_prefix("0x").unwrap_or(hash_str);

                match hex::decode(hash_str) {
                    Ok(bytes) if bytes.len() == 32 => {
                        let mut array = [0u8; 32];
                        array.copy_from_slice(&bytes);
                        hashes.push(array.into());
                    }
                    _ => {
                        return (
                            StatusCode::BAD_REQUEST,
                            Json(ErrorResponse {
                                error: format!("Invalid market hash: {}", hash_str),
                            }),
                        )
                            .into_response();
                    }
                }
            }
            hashes
        }
        None => {
            // Return empty list if no market hashes provided
            return (
                StatusCode::OK,
                Json(GetMarketsResponse {
                    markets: vec![],
                    count: 0,
                }),
            )
                .into_response();
        }
    };

    // Fetch markets from contract
    let markets = match state.contract_client.get_markets(market_hashes).await {
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

    // Filter based on status
    let filtered_markets: Vec<MarketInfo> = match params.status.to_lowercase().as_str() {
        "open" => markets
            .into_iter()
            .filter(|m| m.resolution == "UNRESOLVED")
            .collect(),
        "closed" => markets
            .into_iter()
            .filter(|m| m.resolution != "UNRESOLVED" && m.resolution != "NULL")
            .collect(),
        "all" => markets,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Invalid status parameter. Use 'open', 'closed', or 'all'".to_string(),
                }),
            )
                .into_response();
        }
    };

    let count = filtered_markets.len();

    (
        StatusCode::OK,
        Json(GetMarketsResponse {
            markets: filtered_markets,
            count,
        }),
    )
        .into_response()
}
