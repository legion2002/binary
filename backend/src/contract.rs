use alloy::primitives::{Address, FixedBytes};
use alloy::providers::Provider;
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
    provider: alloy::providers::ReqwestProvider,
}

impl ContractClient {
    pub fn new(multiverse_address: Address, provider: alloy::providers::ReqwestProvider) -> Self {
        Self {
            multiverse_address,
            provider,
        }
    }

    pub async fn get_market(&self, market_hash: FixedBytes<32>) -> anyhow::Result<MarketInfo> {
        let multiverse = MultiVerse::new(self.multiverse_address, &self.provider);

        let market = multiverse.markets(market_hash).call().await?;
        let resolution = multiverse.resolutions(market_hash).call().await?;

        let resolution_str = match resolution._0 {
            MultiVerse::Resolution::NULL => "NULL",
            MultiVerse::Resolution::UNRESOLVED => "UNRESOLVED",
            MultiVerse::Resolution::YES => "YES",
            MultiVerse::Resolution::NO => "NO",
            MultiVerse::Resolution::EVEN => "EVEN",
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
}
