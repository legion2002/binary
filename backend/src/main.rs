mod bindings;
mod config;
mod contract;
mod db;
mod indexer;
mod routes;

use axum::{routing::get, Router};
use config::Config;
use contract::ContractClient;
use db::Database;
use indexer::EventIndexer;
use routes::{get_markets, AppState};
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

    // Initialize database
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:markets.db".to_string());
    let db = Arc::new(Database::new(&database_url).await?);
    tracing::info!("Database initialized at: {}", database_url);

    // Create contract client
    let contract_client = Arc::new(ContractClient::new(
        config.multiverse_address,
        config.provider,
    ));

    // Start event indexer in background task
    let ws_rpc_url = std::env::var("WS_RPC_URL")
        .expect("WS_RPC_URL must be set");
    let indexer = EventIndexer::new(
        config.multiverse_address,
        ws_rpc_url,
        db.clone(),
    );

    tokio::spawn(async move {
        if let Err(e) = indexer.start().await {
            tracing::error!("Event indexer error: {}", e);
        }
    });

    // Create app state
    let state = AppState {
        contract_client,
        db,
    };

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        .route("/markets", get(get_markets))
        .with_state(state)
        .layer(cors)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // Start server
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
