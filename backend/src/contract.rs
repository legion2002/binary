use alloy::primitives::{Address, FixedBytes, keccak256, B256};
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use alloy::rpc::types::Filter;
use alloy::signers::local::PrivateKeySigner;
use alloy::sol_types::{SolEvent, SolValue};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::bindings::r#multi_verse::MultiVerse;
use crate::council::{CouncilOracle, CouncilProof};
use crate::bindings::r#multi_verse::MultiVerse::VerseTokensCreated;
use crate::tempo_amm::{UniV2Client, PairInfo, get_market_pair_info};

alloy::sol! {
    /// MultiVerse.markets as deployed: it also returns the market's resolution. The generated
    /// bindings predate that and still call a resolutions() getter the contract no longer has.
    #[sol(rpc)]
    interface MultiVerseMarkets {
        function markets(bytes32 marketHash)
            external
            view
            returns (uint32 resolutionDeadline, address oracle, bytes32 questionHash, uint8 resolution);
    }
}

/// Names a MultiVerse.Resolution value.
fn resolution_name(value: u8) -> &'static str {
    match value {
        0 => "NULL",
        1 => "UNRESOLVED",
        2 => "YES",
        3 => "NO",
        4 => "EVEN",
        _ => "UNKNOWN",
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketInfo {
    pub market_hash: String,
    pub question_hash: String,
    pub resolution_deadline: u32,
    pub oracle: String,
    pub resolution: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerseAddresses {
    pub yes_verse: String,
    pub no_verse: String,
}

pub struct ContractClient {
    multiverse_address: Address,
    univ2_factory_address: Address,
    provider: DynProvider,
    signer: PrivateKeySigner,
}

impl ContractClient {
    pub fn new(
        multiverse_address: Address,
        univ2_factory_address: Address,
        provider: DynProvider,
        signer: PrivateKeySigner,
    ) -> Self {
        Self {
            multiverse_address,
            univ2_factory_address,
            provider,
            signer,
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
        tracing::debug!("get_market: Fetching market {:?} from multiverse {:?}", market_hash, self.multiverse_address);
        let multiverse = MultiVerseMarkets::new(self.multiverse_address, &self.provider);
        let market = multiverse.markets(market_hash).call().await?;

        Ok(MarketInfo {
            market_hash: format!("0x{}", hex::encode(market_hash)),
            question_hash: format!("0x{}", hex::encode(market.questionHash)),
            resolution_deadline: market.resolutionDeadline,
            oracle: format!("{:?}", market.oracle),
            resolution: resolution_name(market.resolution).to_string(),
        })
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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

        tracing::info!("open_market: Using RPC_URL: {}", rpc_url);

        let signed_provider = ProviderBuilder::new()
            .with_gas_estimation()
            .wallet(wallet)
            .connect(&rpc_url)
            .await
            .map_err(|e| format!("Failed to create signed provider at {}: {}", rpc_url, e))?;

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

    /// Get AMM pair info for market tokens
    /// Returns pair state for YES and NO verse tokens
    pub async fn get_market_pairs(
        &self,
        market_hash: FixedBytes<32>,
        asset: Address,
        quote_token: Address,
    ) -> Result<Vec<PairInfo>, String> {
        // Get verse token addresses
        let (yes_token, no_token) = self.get_verse_addresses_raw(asset, market_hash).await
            .map_err(|e| format!("Failed to get verse addresses: {}", e))?;

        // Create UniV2 client
        let client = UniV2Client::new(Arc::new(self.provider.clone()), self.univ2_factory_address);

        // Get pair info for both tokens
        get_market_pair_info(&client, yes_token, no_token, quote_token)
            .await
            .map_err(|e| format!("Failed to get pair info: {}", e))
    }

    /// Get price from UniV2 pair
    #[allow(dead_code)]
    pub async fn get_price(
        &self,
        token_in: Address,
        token_out: Address,
    ) -> Result<f64, String> {
        let client = UniV2Client::new(Arc::new(self.provider.clone()), self.univ2_factory_address);

        let pair_address = client.get_pair(token_in, token_out)
            .await
            .map_err(|e| format!("Failed to get pair: {}", e))?
            .ok_or("Pair does not exist")?;

        client.get_price(token_in, token_out, pair_address)
            .await
            .map_err(|e| format!("Failed to get price: {}", e))
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

    /// Get just the resolution status for a market (more efficient than get_market)
    /// Returns: "NULL", "UNRESOLVED", "YES", "NO", or "EVEN"
    pub async fn get_resolution(&self, market_hash: FixedBytes<32>) -> anyhow::Result<String> {
        let multiverse = MultiVerseMarkets::new(self.multiverse_address, &self.provider);
        let market = multiverse.markets(market_hash).call().await?;
        Ok(resolution_name(market.resolution).to_string())
    }

    /// Get resolution for multiple markets in parallel
    #[allow(dead_code)]
    pub async fn get_resolutions(
        &self,
        market_hashes: Vec<FixedBytes<32>>,
    ) -> anyhow::Result<Vec<(String, String)>> {
        use futures_util::future::join_all;

        let futures: Vec<_> = market_hashes
            .iter()
            .map(|hash| {
                let hash = *hash;
                async move {
                    let resolution = self.get_resolution(hash).await.unwrap_or_else(|_| "UNKNOWN".to_string());
                    (format!("0x{}", hex::encode(hash)), resolution)
                }
            })
            .collect();

        Ok(join_all(futures).await)
    }

    /// Get all VerseTokensCreated events for a specific market hash
    /// Returns a list of (asset, yes_verse, no_verse) tuples
    pub async fn get_verse_tokens_created_events(
        &self,
        market_hash: FixedBytes<32>,
    ) -> anyhow::Result<Vec<(Address, Address, Address)>> {
        // Build filter for VerseTokensCreated events with specific marketHash
        let filter = Filter::new()
            .address(self.multiverse_address)
            .event_signature(VerseTokensCreated::SIGNATURE_HASH)
            .topic1(market_hash)
            .from_block(0);

        // Query logs
        let logs = self.provider.get_logs(&filter).await?;

        // Decode logs into VerseTokensCreated events
        let mut results = Vec::new();
        for log in logs {
            if let Ok(event) = VerseTokensCreated::decode_log(&log.inner) {
                results.push((event.asset, event.yesVerse, event.noVerse));
            }
        }

        Ok(results)
    }

    // ========================================================================
    // Council oracle
    // ========================================================================

    /// A provider that signs transactions with the backend key
    async fn signed_provider(&self) -> Result<DynProvider, String> {
        let wallet = alloy::network::EthereumWallet::from(self.signer.clone());
        let rpc_url = std::env::var("RPC_URL").map_err(|_| "RPC_URL not set".to_string())?;
        let provider = ProviderBuilder::new()
            .with_gas_estimation()
            .wallet(wallet)
            .connect(&rpc_url)
            .await
            .map_err(|e| format!("Failed to create signed provider: {}", e))?;
        Ok(provider.erased())
    }

    /// Readable message for a failed CouncilOracle call, decoding its custom errors
    fn council_error(e: alloy::contract::Error) -> String {
        match e.as_decoded_interface_error::<CouncilOracle::CouncilOracleErrors>() {
            Some(decoded) => format!("CouncilOracle reverted: {:?}", decoded),
            None => format!("{}", e),
        }
    }

    /// Read a market's council registration and resolution from the CouncilOracle
    pub async fn council_market(
        &self,
        oracle: Address,
        market_hash: FixedBytes<32>,
    ) -> anyhow::Result<CouncilOracle::marketsReturn> {
        let contract = CouncilOracle::new(oracle, &self.provider);
        Ok(contract.markets(market_hash).call().await?)
    }

    /// Pin a market to a council config and salted question hash (simulated first)
    pub async fn register_council_market(
        &self,
        oracle: Address,
        market_hash: FixedBytes<32>,
        council_hash: B256,
        question_hash: B256,
    ) -> Result<B256, String> {
        let provider = self.signed_provider().await?;
        let contract = CouncilOracle::new(oracle, &provider);
        let call = contract.registerMarket(market_hash, council_hash, question_hash);
        call.clone()
            .call()
            .await
            .map_err(|e| format!("registerMarket would revert: {}", Self::council_error(e)))?;
        let receipt = call
            .send()
            .await
            .map_err(|e| format!("Failed to send registerMarket: {}", e))?
            .get_receipt()
            .await
            .map_err(|e| format!("Failed to get receipt: {}", e))?;
        if !receipt.status() {
            return Err(format!("registerMarket reverted: {:?}", receipt.transaction_hash));
        }
        tracing::info!("Council market registered. Transaction: {:?}", receipt.transaction_hash);
        Ok(receipt.transaction_hash)
    }

    /// Submit a signed council decision record to the CouncilOracle (simulated first)
    pub async fn resolve_with_council(
        &self,
        oracle: Address,
        market_hash: FixedBytes<32>,
        proof: &CouncilProof,
    ) -> Result<B256, String> {
        let provider = self.signed_provider().await?;
        let contract = CouncilOracle::new(oracle, &provider);
        let call = contract.resolve(
            market_hash,
            proof.record_jcs.clone().into(),
            proof.commitments_jcs.clone().into(),
            proof.signature.clone().into(),
        );
        call.clone()
            .call()
            .await
            .map_err(|e| format!("resolve would revert: {}", Self::council_error(e)))?;
        let receipt = call
            .send()
            .await
            .map_err(|e| format!("Failed to send resolve: {}", e))?
            .get_receipt()
            .await
            .map_err(|e| format!("Failed to get receipt: {}", e))?;
        if !receipt.status() {
            return Err(format!("resolve reverted: {:?}", receipt.transaction_hash));
        }
        tracing::info!("Council resolution set. Transaction: {:?}", receipt.transaction_hash);
        Ok(receipt.transaction_hash)
    }

    /// Finalize a market on the MultiVerse from its oracle (simulated first)
    pub async fn resolve_market(&self, market_hash: FixedBytes<32>) -> Result<B256, String> {
        let provider = self.signed_provider().await?;
        let multiverse = MultiVerse::new(self.multiverse_address, &provider);
        let call = multiverse.resolve(market_hash);
        call.clone()
            .call()
            .await
            .map_err(|e| format!("MultiVerse.resolve would revert: {}", e))?;
        let receipt = call
            .send()
            .await
            .map_err(|e| format!("Failed to send MultiVerse.resolve: {}", e))?
            .get_receipt()
            .await
            .map_err(|e| format!("Failed to get receipt: {}", e))?;
        if !receipt.status() {
            return Err(format!("MultiVerse.resolve reverted: {:?}", receipt.transaction_hash));
        }
        Ok(receipt.transaction_hash)
    }
}
