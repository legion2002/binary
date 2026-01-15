mod admin;
mod auth;
mod bindings;
mod config;
mod contract;
mod db;
mod routes;
mod tempo_amm;

use admin::{add_market, open_market, AdminState};
use auth::require_admin_api_key;
use axum::{middleware, routing::{get, post}, Router};
use config::Config;
use contract::ContractClient;
use db::Database;
use routes::{get_config, get_market, get_markets, get_verse_tokens, AppState};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env().await?;
    let addr = config.server_address();

    tracing::info!("MultiVerse Address: {:?}", config.multiverse_address);
    tracing::info!("Oracle Address: {:?}", config.oracle_address);
    tracing::info!("UniV2 Factory Address: {:?}", config.univ2_factory_address);

    // Initialize database
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:markets.db".to_string());
    let db = Arc::new(Database::new(&database_url).await?);
    tracing::info!("Database initialized at: {}", database_url);

    // Create contract client
    let contract_client = Arc::new(ContractClient::new(
        config.multiverse_address,
        config.univ2_factory_address,
        config.provider.clone(),
        config.signer.clone(),
    ));

    // Create app state
    let state = AppState {
        contract_client: contract_client.clone(),
        db: db.clone(),
        multiverse_address: config.multiverse_address,
        oracle_address: config.oracle_address,
    };

    // Create admin state
    let admin_state = AdminState {
        contract_client,
        db,
        oracle_address: config.oracle_address,
    };

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build admin routes (protected by API key)
    let admin_routes = Router::new()
        .route("/markets/open", post(open_market))
        .route("/markets/add", post(add_market))
        .route_layer(middleware::from_fn_with_state(
            config.admin_api_key_hash.clone(),
            require_admin_api_key,
        ))
        .with_state(admin_state);

    // Build main router
    let app = Router::new()
        .route("/config", get(get_config))
        .route("/markets", get(get_markets))
        .route("/markets/:marketHash", get(get_market))
        .route("/markets/:marketHash/verse-tokens", get(get_verse_tokens))
        .with_state(state)
        .nest("/admin", admin_routes)
        .layer(cors)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // Start server
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
