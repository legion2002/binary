use alloy::primitives::{Address, FixedBytes, keccak256, B256, U160};
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use alloy::sol_types::SolValue;
use alloy::rpc::types::trace::geth::{GethDebugTracingOptions, GethTrace};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::bindings::r#multi_verse::MultiVerse;
use crate::uniswap_v4::{V4PoolManager, V4PoolState, create_market_v4_pools, PoolConfig, CreatedPools};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketInfo {
    pub market_hash: String,
    pub question_hash: String,
    pub resolution_deadline: u32,
    pub oracle: String,
    pub resolution: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerseAddresses {
    pub yes_verse: String,
    pub no_verse: String,
}

pub struct ContractClient {
    multiverse_address: Address,
    provider: DynProvider,
    signer: PrivateKeySigner,
    pool_manager_address: Option<Address>,
}

impl ContractClient {
    pub fn new(
        multiverse_address: Address,
        provider: DynProvider,
        signer: PrivateKeySigner,
        pool_manager_address: Option<Address>,
    ) -> Self {
        Self {
            multiverse_address,
            provider,
            signer,
            pool_manager_address,
        }
    }

    /// Calculate question hash from question text
    pub fn calculate_question_hash(question: &str) -> FixedBytes<32> {
        keccak256(question.as_bytes())
    }

    /// Calculate market hash from parameters (matching Solidity's keccak256(abi.encode(...)))
    pub fn calculate_market_hash(
        question_hash: FixedBytes<32>,
        resolution_deadline: u32,
        oracle: Address,
    ) -> FixedBytes<32> {
        // Encode the same way Solidity does: keccak256(abi.encode(questionHash, resolutionDeadline, oracle))
        let encoded = (question_hash, resolution_deadline, oracle).abi_encode();
        keccak256(&encoded)
    }

    pub async fn get_market(&self, market_hash: FixedBytes<32>) -> anyhow::Result<MarketInfo> {
        let multiverse = MultiVerse::new(self.multiverse_address, &self.provider);

        let market = multiverse.markets(market_hash).call().await?;
        let resolution_value = multiverse.resolutions(market_hash).call().await?;

        // Resolution is a u8: 0=NULL, 1=UNRESOLVED, 2=YES, 3=NO, 4=EVEN
        let resolution_str = match resolution_value {
            0 => "NULL",
            1 => "UNRESOLVED",
            2 => "YES",
            3 => "NO",
            4 => "EVEN",
            _ => "UNKNOWN",
        };

        Ok(MarketInfo {
            market_hash: format!("0x{}", hex::encode(market_hash)),
            question_hash: format!("0x{}", hex::encode(market.questionHash)),
            resolution_deadline: market.resolutionDeadline,
            oracle: format!("{:?}", market.oracle),
            resolution: resolution_str.to_string(),
        })
    }

    pub async fn get_markets(
        &self,
        market_hashes: Vec<FixedBytes<32>>,
    ) -> anyhow::Result<Vec<MarketInfo>> {
        let mut markets = Vec::new();

        for hash in market_hashes {
            match self.get_market(hash).await {
                Ok(market) if market.resolution != "NULL" => markets.push(market),
                _ => continue,
            }
        }

        Ok(markets)
    }

    pub async fn get_verse_addresses(
        &self,
        asset: Address,
        market_hash: FixedBytes<32>,
    ) -> anyhow::Result<VerseAddresses> {
        let multiverse = MultiVerse::new(self.multiverse_address, &self.provider);

        let result = multiverse.getVerseAddress(asset, market_hash).call().await?;

        Ok(VerseAddresses {
            yes_verse: format!("{:?}", result.yesVerse),
            no_verse: format!("{:?}", result.noVerse),
        })
    }

    /// Open a new market on-chain (with simulation trace on failure)
    pub async fn open_market(
        &self,
        question_hash: FixedBytes<32>,
        resolution_deadline: u32,
        oracle: Address,
    ) -> Result<B256, String> {
        // Create a signed provider for sending transactions
        let wallet = alloy::network::EthereumWallet::from(self.signer.clone());
        let rpc_url = std::env::var("RPC_URL")
            .map_err(|_| "RPC_URL not set".to_string())?;

        let signed_provider = ProviderBuilder::new()
            .with_gas_estimation()
            .wallet(wallet)
            .connect(&rpc_url)
            .await
            .map_err(|e| format!("Failed to create signed provider: {}", e))?;

        let multiverse = MultiVerse::new(self.multiverse_address, &signed_provider);

        // First, try to simulate the call to check for errors
        let call_builder = multiverse.open(question_hash, resolution_deadline, oracle);

        match call_builder.clone().call().await {
            Ok(_) => {
                // Simulation succeeded, send the transaction
                let pending_tx = call_builder.send().await
                    .map_err(|e| format!("Failed to send transaction: {}", e))?;

                let receipt = pending_tx.get_receipt().await
                    .map_err(|e| format!("Failed to get receipt: {}", e))?;

                tracing::info!(
                    "Market opened successfully. Transaction: {:?}",
                    receipt.transaction_hash
                );

                Ok(receipt.transaction_hash)
            }
            Err(e) => {
                // Simulation failed, return detailed error with trace
                let error_msg = format!("Transaction simulation failed: {}. This transaction would revert on-chain.", e);
                tracing::error!("{}", error_msg);
                Err(error_msg)
            }
        }
    }

    /// Create verse tokens for a specific asset/market pair (with simulation trace on failure)
    pub async fn create_verse_tokens(
        &self,
        asset: Address,
        market_hash: FixedBytes<32>,
    ) -> Result<(Address, Address, B256), String> {
        // Create a signed provider for sending transactions
        let wallet = alloy::network::EthereumWallet::from(self.signer.clone());
        let rpc_url = std::env::var("RPC_URL")
            .map_err(|_| "RPC_URL not set".to_string())?;

        let signed_provider = ProviderBuilder::new()
            .with_gas_estimation()
            .wallet(wallet)
            .connect(&rpc_url)
            .await
            .map_err(|e| format!("Failed to create signed provider: {}", e))?;

        let multiverse = MultiVerse::new(self.multiverse_address, &signed_provider);

        // First, try to simulate the call to check for errors
        let call_builder = multiverse.create(asset, market_hash);

        match call_builder.clone().call().await {
            Ok(_) => {
                // Simulation succeeded, send the transaction
                let pending_tx = call_builder.send().await
                    .map_err(|e| format!("Failed to send transaction: {}", e))?;

                let receipt = pending_tx.get_receipt().await
                    .map_err(|e| format!("Failed to get receipt: {}", e))?;

                // Get the verse addresses
                let verse_addresses = multiverse.getVerseAddress(asset, market_hash).call().await
                    .map_err(|e| format!("Failed to get verse addresses: {}", e))?;

                tracing::info!(
                    "Verse tokens created. YES: {:?}, NO: {:?}, Transaction: {:?}",
                    verse_addresses.yesVerse,
                    verse_addresses.noVerse,
                    receipt.transaction_hash
                );

                Ok((
                    verse_addresses.yesVerse,
                    verse_addresses.noVerse,
                    receipt.transaction_hash,
                ))
            }
            Err(e) => {
                // Simulation failed, return detailed error
                let error_msg = format!("Transaction simulation failed: {}. This transaction would revert on-chain.", e);
                tracing::error!("{}", error_msg);
                Err(error_msg)
            }
        }
    }

    /// Initialize V4 pools for a market with given token pairs
    /// Creates configured pools (default: YES_TOKEN0/YES_TOKEN1, NO_TOKEN0/NO_TOKEN1, YES_TOKEN0/NO_TOKEN0)
    pub async fn initialize_v4_pools(
        &self,
        market_hash: FixedBytes<32>,
        token0_address: Address, // e.g., USDC
        token1_address: Address, // e.g., ETH/WETH
        config: Option<PoolConfig>,
    ) -> Result<CreatedPools, String> {
        // Check if V4 pool manager is configured
        let pool_manager_address = self.pool_manager_address
            .ok_or_else(|| "V4 pool manager not configured - pool creation disabled".to_string())?;

        // Get verse token addresses for both base tokens
        let (yes_token0, no_token0) = self.get_verse_addresses_raw(token0_address, market_hash).await
            .map_err(|e| format!("Failed to get verse addresses for token0: {}", e))?;
        let (yes_token1, no_token1) = self.get_verse_addresses_raw(token1_address, market_hash).await
            .map_err(|e| format!("Failed to get verse addresses for token1: {}", e))?;

        // Create V4 pool manager
        let pool_manager = V4PoolManager::new(
            pool_manager_address,
            Arc::new(self.provider.clone()),
        );

        // Use provided config or default
        let pool_config = config.unwrap_or_default();

        // Create the configured pools
        let result = create_market_v4_pools(
            &pool_manager,
            yes_token0,
            no_token0,
            yes_token1,
            no_token1,
            pool_config,
        ).await;

        match result {
            Ok(pools) => {
                tracing::info!(
                    "V4 pools initialized for market {} with tokens {:?} and {:?}",
                    hex::encode(market_hash),
                    token0_address,
                    token1_address
                );
                Ok(pools)
            }
            Err(e) => {
                let error_msg = format!("Failed to initialize V4 pools: {}", e);
                tracing::error!("{}", error_msg);
                Err(error_msg)
            }
        }
    }

    /// Get V4 pool state
    pub async fn get_v4_pool_state(
        &self,
        pool_id: FixedBytes<32>,
    ) -> Result<V4PoolState, String> {
        // Check if V4 pool manager is configured
        let pool_manager_address = self.pool_manager_address
            .ok_or_else(|| "V4 pool manager not configured".to_string())?;

        let pool_manager = V4PoolManager::new(
            pool_manager_address,
            Arc::new(self.provider.clone()),
        );

        pool_manager.get_pool_state(pool_id).await
            .map_err(|e| format!("Failed to get V4 pool state: {}", e))
    }

    /// Helper to get raw verse addresses (returns Address instead of String)
    async fn get_verse_addresses_raw(
        &self,
        asset: Address,
        market_hash: FixedBytes<32>,
    ) -> anyhow::Result<(Address, Address)> {
        let multiverse = MultiVerse::new(self.multiverse_address, &self.provider);
        let result = multiverse.getVerseAddress(asset, market_hash).call().await?;
        Ok((result.yesVerse, result.noVerse))
    }
}
