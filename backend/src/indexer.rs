use crate::bindings::multi_verse::MultiVerse;
use crate::db::Database;
use alloy::primitives::Address;
use alloy::providers::{ProviderBuilder, WsConnect};
use futures_util::StreamExt;
use std::sync::Arc;

pub struct EventIndexer {
    multiverse_address: Address,
    ws_rpc_url: String,
    db: Arc<Database>,
}

impl EventIndexer {
    pub fn new(multiverse_address: Address, ws_rpc_url: String, db: Arc<Database>) -> Self {
        Self {
            multiverse_address,
            ws_rpc_url,
            db,
        }
    }

    /// Start listening to MarketOpened events
    pub async fn start(self) -> anyhow::Result<()> {
        tracing::info!(
            "Starting event indexer for MultiVerse at {:?}",
            self.multiverse_address
        );
        tracing::info!("Connecting to WebSocket RPC: {}", self.ws_rpc_url);

        // Create WebSocket provider
        let ws = WsConnect::new(&self.ws_rpc_url);
        let provider = ProviderBuilder::new().connect_ws(ws).await?;

        tracing::info!("Connected to WebSocket RPC");

        // Create contract instance
        let contract = MultiVerse::new(self.multiverse_address, provider);

        // Subscribe to MarketOpened events
        let filter = contract.MarketOpened_filter().watch().await?;

        tracing::info!("Subscribed to MarketOpened events, waiting for events...");

        // Stream events
        let mut stream = filter.into_stream();

        while let Some(event) = stream.next().await {
            match event {
                Ok((event_data, log)) => {
                    tracing::info!(
                        "Received MarketOpened event: market_hash={:?}, question_hash={:?}, deadline={}, oracle={:?}, block={}",
                        event_data.marketHash,
                        event_data.questionHash,
                        event_data.resolutionDeadline,
                        event_data.oracle,
                        log.block_number.unwrap_or(0)
                    );

                    // Insert into database
                    if let Err(e) = self
                        .db
                        .insert_market(
                            event_data.marketHash,
                            event_data.questionHash,
                            event_data.resolutionDeadline,
                            event_data.oracle,
                            log.block_number.unwrap_or(0),
                        )
                        .await
                    {
                        tracing::error!("Failed to insert market into database: {}", e);
                    } else {
                        tracing::info!("Market indexed successfully");
                    }
                }
                Err(e) => {
                    tracing::error!("Error receiving event: {}", e);
                }
            }
        }

        tracing::warn!("Event stream ended");
        Ok(())
    }
}
