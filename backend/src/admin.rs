use crate::contract::ContractClient;
use crate::council::{self, CouncilInfo, CouncilProof, CouncilSettings, Evidence, Outcome};
use crate::db::Database;
use crate::tempo_amm::{PairInfo, PATH_USD_ADDRESS};
use alloy::primitives::{Address, FixedBytes};
use axum::{
    extract::{Path, State},
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
    /// Set when markets are opened with, and resolved through, the council oracle
    pub council: Option<Arc<CouncilSettings>>,
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
    pub pairs: Vec<PairInfo>,
    /// Council oracle markets: the council and salted question the market is pinned to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub council: Option<CouncilInfo>,
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

    // Council oracle markets pin a salted question hash: question_hash =
    // sha256(JCS({"question","salt"})). The salt is kept and published so anyone can check it.
    let council_question = state.council.as_ref().map(|_| {
        let salt = council::new_question_salt();
        let hash = council::question_hash(&request.question, &salt);
        (salt, hash)
    });

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

    // Register the market with the council oracle
    let mut council_info = None;
    if let (Some(c), Some((salt, question_hash))) = (&state.council, &council_question) {
        let council_hash = council::hex32(c.council_hash);
        let question_hash_hex = council::hex32(*question_hash);
        if let Err(e) = state
            .db
            .set_council_question(market_hash, &council_hash, salt, &question_hash_hex)
            .await
        {
            tracing::error!("Failed to store council question in database: {}", e);
        }
        if let Err(e) = state
            .contract_client
            .register_council_market(c.oracle_address, market_hash, c.council_hash, *question_hash)
            .await
        {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Market opened but not registered with the council oracle: {}. Resolving the market registers it.", e),
                    "marketHash": format!("0x{}", hex::encode(market_hash)),
                    "council": {"councilHash": council_hash, "questionSalt": salt, "questionHash": question_hash_hex},
                })),
            )
                .into_response();
        }
        council_info = Some(CouncilInfo {
            council_hash,
            question_salt: salt.clone(),
            question_hash: question_hash_hex,
            query_id: None,
        });
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

    // Get AMM pair info for created verse tokens
    // Parse quote token address (defaults to pathUSD)
    let quote_token = request.quote_token
        .as_ref()
        .and_then(|addr| addr.parse::<Address>().ok())
        .unwrap_or(PATH_USD_ADDRESS);

    let mut pairs = Vec::new();
    for &asset in &assets {
        // Try to get pair info for each asset's verse tokens
        match state
            .contract_client
            .get_market_pairs(market_hash, asset, quote_token)
            .await
        {
            Ok(infos) => {
                // Store pair info in database
                if let Err(e) = state
                    .db
                    .insert_pair_market(
                        market_hash,
                        asset,
                        quote_token,
                        infos.iter().find(|i| i.pair_type == "YES/QUOTE").map(|i| i.pair_address.clone()),
                        infos.iter().find(|i| i.pair_type == "NO/QUOTE").map(|i| i.pair_address.clone()),
                    )
                    .await
                {
                    tracing::error!("Failed to store pair info in database: {}", e);
                }
                pairs.extend(infos);
            }
            Err(e) => {
                tracing::debug!("No pair info available for asset {:?}: {}", asset, e);
            }
        }
    }

    tracing::info!(
        "Market created with {} AMM pairs",
        pairs.len()
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
            pairs,
            council: council_info,
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
    pub pairs: Vec<PairInfo>,
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

    // Store verse tokens and fetch pair info
    let quote_token = request.quote_token
        .as_ref()
        .and_then(|addr| addr.parse::<Address>().ok())
        .unwrap_or(PATH_USD_ADDRESS);

    let mut verse_tokens = Vec::new();
    let mut pairs = Vec::new();

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

        // Fetch pair info for this asset
        match state
            .contract_client
            .get_market_pairs(market_hash, asset, quote_token)
            .await
        {
            Ok(infos) => {
                // Store pair info in database
                if let Err(e) = state
                    .db
                    .insert_pair_market(
                        market_hash,
                        asset,
                        quote_token,
                        infos.iter().find(|i| i.pair_type == "YES/QUOTE").map(|i| i.pair_address.clone()),
                        infos.iter().find(|i| i.pair_type == "NO/QUOTE").map(|i| i.pair_address.clone()),
                    )
                    .await
                {
                    tracing::error!("Failed to store pair info in database: {}", e);
                }
                pairs.extend(infos);
            }
            Err(e) => {
                tracing::debug!("No pair info available for asset {:?}: {}", asset, e);
            }
        }
    }

    tracing::info!(
        "Market added with {} verse tokens and {} AMM pairs",
        verse_tokens.len(),
        pairs.len()
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
            pairs,
        }),
    )
        .into_response()
}

// ============================================================================
// Resolve Market Endpoint - Resolve a council oracle market through the AI council
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct ResolveMarketRequest {
    /// Evidence for the council, one entry per party (party names: [a-z0-9_-]{1,32})
    pub evidence: Vec<Evidence>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolveMarketResponse {
    pub market_hash: String,
    pub query_id: String,
    /// keccak256(query_id), indexed in the oracle's ResolutionSet event
    pub query_id_hash: String,
    /// "YES" or "NO"
    pub outcome: String,
    /// sha256 of the signed record bytes
    pub digest: String,
    pub signer: String,
    pub oracle_transaction_hash: String,
    /// MultiVerse.resolve, sent after the oracle resolution
    pub multiverse_transaction_hash: Option<String>,
    pub multiverse_error: Option<String>,
    /// The signed decision record
    pub record: serde_json::Value,
}

fn error(status: StatusCode, msg: impl Into<String>) -> Response {
    (status, Json(ErrorResponse { error: msg.into() })).into_response()
}

/// POST /admin/markets/:marketHash/resolve
/// Asks the council to decide the market question from the given evidence (a paid query),
/// verifies the signed record and submits it to the CouncilOracle, then resolves the market.
pub async fn resolve_market(
    State(state): State<AdminState>,
    Path(market_hash): Path<String>,
    Json(request): Json<ResolveMarketRequest>,
) -> Response {
    let Some(settings) = state.council.clone() else {
        return error(StatusCode::BAD_REQUEST, "Council oracle is not configured (MARKET_ORACLE=council)");
    };
    let market_hash = market_hash.to_lowercase();
    let Some(market_hash_bytes) = council::parse_hash(&market_hash) else {
        return error(
            StatusCode::BAD_REQUEST,
            "Invalid market hash format. Expected 0x-prefixed 32-byte hex string",
        );
    };
    if request.evidence.is_empty() {
        return error(StatusCode::BAD_REQUEST, "evidence must list at least one party");
    }

    let market = match state.db.get_market(&market_hash).await {
        Ok(Some(m)) => m,
        Ok(None) => return error(StatusCode::NOT_FOUND, format!("Market not found: {}", market_hash)),
        Err(e) => return error(StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch market: {}", e)),
    };
    if market.oracle.parse::<Address>().ok() != Some(settings.oracle_address) {
        return error(StatusCode::BAD_REQUEST, "Market is not resolved by the council oracle");
    }
    let (Some(question), Some(salt), Some(council_hash), Some(question_hash)) = (
        market.question_text.clone(),
        market.council_question_salt.clone(),
        market.council_hash.as_deref().and_then(council::parse_hash),
        market.council_question_hash.as_deref().and_then(council::parse_hash),
    ) else {
        return error(StatusCode::CONFLICT, "Market has no stored council question (question, salt and hashes)");
    };
    if council::question_hash(&question, &salt) != question_hash {
        return error(StatusCode::INTERNAL_SERVER_ERROR, "Stored question does not match its question hash");
    }
    // The market stays pinned to the council it was created with.
    let mut settings = (*settings).clone();
    settings.council_hash = council_hash;

    // On chain: register if opening the market could not, refuse if already resolved.
    let on_chain = match state
        .contract_client
        .council_market(settings.oracle_address, market_hash_bytes)
        .await
    {
        Ok(m) => m,
        Err(e) => return error(StatusCode::BAD_GATEWAY, format!("Failed to read the council oracle: {}", e)),
    };
    if on_chain.resolved {
        return error(StatusCode::CONFLICT, "Market is already resolved by the council oracle");
    }
    if !on_chain.registered {
        if let Err(e) = state
            .contract_client
            .register_council_market(settings.oracle_address, market_hash_bytes, council_hash, question_hash)
            .await
        {
            return error(StatusCode::BAD_GATEWAY, format!("Failed to register the market: {}", e));
        }
    } else if on_chain.councilHash != council_hash || on_chain.questionHash != question_hash {
        return error(StatusCode::CONFLICT, "Market is registered on chain with another council or question");
    }

    // Ask the council (paid), then verify the record again through the council API.
    let stem = format!(
        "{}-{}",
        market_hash,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or_default()
    );
    let opened = match council::open_query(&settings, &stem, &question, &salt, &request.evidence).await {
        Ok(q) => q,
        Err(e) => return error(StatusCode::BAD_GATEWAY, format!("Council query failed: {}", e)),
    };
    if let Err(e) = council::verify_query(&settings, &opened.query_id).await {
        return error(StatusCode::BAD_GATEWAY, format!("Council record did not verify: {}", e));
    }
    let proof = match CouncilProof::from_record(&opened.record) {
        Ok(p) => p,
        Err(e) => return error(StatusCode::BAD_GATEWAY, format!("Malformed council record: {}", e)),
    };
    if proof.query_id != opened.query_id {
        return error(StatusCode::BAD_GATEWAY, "Council record is for another query");
    }
    if let Err(e) = council::check_record(&opened.record, &proof, council_hash, question_hash, &settings.signers) {
        return error(StatusCode::BAD_GATEWAY, format!("Council record rejected: {}", e));
    }
    tracing::info!(
        "Council decided market {}: query {}, digest {}",
        market_hash,
        proof.query_id,
        council::hex32(proof.digest)
    );
    let outcome = match &proof.outcome {
        Outcome::Yes => "YES",
        Outcome::No => "NO",
        Outcome::Other(o) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(serde_json::json!({
                    "error": "The council did not decide yes or no. The market stays open; resolve it again with a new query.",
                    "queryId": proof.query_id,
                    "outcome": o,
                })),
            )
                .into_response();
        }
    };

    let oracle_tx = match state
        .contract_client
        .resolve_with_council(settings.oracle_address, market_hash_bytes, &proof)
        .await
    {
        Ok(tx) => tx,
        Err(e) => return error(StatusCode::BAD_GATEWAY, format!("Council oracle refused the record: {}", e)),
    };
    if let Err(e) = state.db.set_council_query(&market_hash, &proof.query_id).await {
        tracing::error!("Failed to store council query id: {}", e);
    }
    let (multiverse_tx, multiverse_error) = match state.contract_client.resolve_market(market_hash_bytes).await {
        Ok(tx) => (Some(format!("{:?}", tx)), None),
        Err(e) => (None, Some(e)),
    };

    (
        StatusCode::OK,
        Json(ResolveMarketResponse {
            market_hash,
            query_id: proof.query_id.clone(),
            query_id_hash: council::hex32(proof.query_id_hash()),
            outcome: outcome.to_string(),
            digest: council::hex32(proof.digest),
            signer: format!("{:?}", proof.signer).to_lowercase(),
            oracle_transaction_hash: format!("{:?}", oracle_tx),
            multiverse_transaction_hash: multiverse_tx,
            multiverse_error,
            record: opened.record,
        }),
    )
        .into_response()
}
