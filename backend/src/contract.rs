use alloy::primitives::{Address, FixedBytes, keccak256, B256};
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use alloy::sol_types::SolValue;
use alloy::rpc::types::trace::geth::{GethDebugTracingOptions, GethTrace};
use serde::{Deserialize, Serialize};

use crate::bindings::r#multi_verse::MultiVerse;

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
}

impl ContractClient {
    pub fn new(multiverse_address: Address, provider: DynProvider, signer: PrivateKeySigner) -> Self {
        Self {
            multiverse_address,
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
}
