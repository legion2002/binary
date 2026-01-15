use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::sol;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// pathUSD token address (root quote token)
pub const PATH_USD_ADDRESS: Address =
    Address::new([0x20, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

sol! {
    #[sol(rpc)]
    interface IUniswapV2Pair {
        function token0() external view returns (address);
        function token1() external view returns (address);
        function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast);
    }

    #[sol(rpc)]
    interface IUniswapV2Factory {
        function getPair(address tokenA, address tokenB) external view returns (address pair);
        function allPairs(uint256) external view returns (address pair);
        function allPairsLength() external view returns (uint256);
    }
}

/// AMM pair information for a market token pair
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PairInfo {
    pub pair_type: String,
    pub pair_address: String,
    pub base_token: String,
    pub quote_token: String,
    pub base_reserve: String,
    pub quote_reserve: String,
    pub price: Option<String>,
    pub liquidity: Option<String>,
}

/// Calculate price from reserves
/// price = quote_reserve / base_reserve (adjusted for decimals)
fn calculate_price_from_reserves(
    reserve_base: u128,
    reserve_quote: u128,
    base_decimals: u8,
    quote_decimals: u8,
) -> f64 {
    if reserve_base == 0 {
        return 0.0;
    }
    let base = reserve_base as f64 / 10_f64.powi(base_decimals as i32);
    let quote = reserve_quote as f64 / 10_f64.powi(quote_decimals as i32);
    quote / base
}

/// Calculate liquidity as sqrt(reserve0 * reserve1)
fn calculate_liquidity(reserve0: u128, reserve1: u128) -> f64 {
    ((reserve0 as f64) * (reserve1 as f64)).sqrt()
}

/// Calculate market probability from YES/NO token prices
///
/// The probability is derived from the relative prices of YES and NO tokens.
/// In a prediction market, YES + NO token prices should sum to ~1.0 (the redemption value).
///
/// Returns (yes_probability, no_probability) as values between 0.0 and 1.0
pub fn calculate_probability(
    yes_price: Option<f64>,
    no_price: Option<f64>,
) -> (f64, f64) {
    match (yes_price, no_price) {
        (Some(yes), Some(no)) => {
            let total = yes + no;
            if total > 0.0 {
                (yes / total, no / total)
            } else {
                (0.5, 0.5)
            }
        }
        (Some(yes), None) => {
            let clamped_yes = yes.clamp(0.0, 1.0);
            (clamped_yes, 1.0 - clamped_yes)
        }
        (None, Some(no)) => {
            let clamped_no = no.clamp(0.0, 1.0);
            (1.0 - clamped_no, clamped_no)
        }
        (None, None) => (0.5, 0.5),
    }
}

/// Calculate probability from pair info
/// Extracts prices and calculates probability
pub fn calculate_probability_from_pairs(pairs: &[PairInfo]) -> (f64, f64) {
    let mut yes_price: Option<f64> = None;
    let mut no_price: Option<f64> = None;

    for pair in pairs {
        if pair.pair_type == "YES/QUOTE" {
            if let Some(ref price) = pair.price {
                yes_price = price.parse::<f64>().ok();
            }
        } else if pair.pair_type == "NO/QUOTE" {
            if let Some(ref price) = pair.price {
                no_price = price.parse::<f64>().ok();
            }
        }
    }

    calculate_probability(yes_price, no_price)
}

pub struct UniV2Client<P> {
    provider: Arc<P>,
    factory_address: Address,
}

impl<P: Provider + Clone + 'static> UniV2Client<P> {
    pub fn new(provider: Arc<P>, factory_address: Address) -> Self {
        Self { provider, factory_address }
    }

    /// Get the pair address for two tokens
    pub async fn get_pair(
        &self,
        token_a: Address,
        token_b: Address,
    ) -> Result<Option<Address>, Box<dyn std::error::Error + Send + Sync>> {
        let factory = IUniswapV2Factory::new(self.factory_address, &self.provider);
        let pair_address = factory.getPair(token_a, token_b).call().await?;
        
        if pair_address == Address::ZERO {
            Ok(None)
        } else {
            Ok(Some(pair_address))
        }
    }

    /// Get reserves for a pair
    pub async fn get_reserves(
        &self,
        pair_address: Address,
    ) -> Result<(u128, u128, Address, Address), Box<dyn std::error::Error + Send + Sync>> {
        let pair = IUniswapV2Pair::new(pair_address, &self.provider);
        
        let reserves = pair.getReserves().call().await?;
        let token0 = pair.token0().call().await?;
        let token1 = pair.token1().call().await?;
        
        Ok((
            reserves.reserve0.to::<u128>(),
            reserves.reserve1.to::<u128>(),
            token0,
            token1,
        ))
    }

    /// Get price of token_in in terms of token_out from a pair
    pub async fn get_price(
        &self,
        token_in: Address,
        _token_out: Address,
        pair_address: Address,
    ) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
        let (reserve0, reserve1, token0, _token1) = self.get_reserves(pair_address).await?;
        
        let (reserve_in, reserve_out) = if token_in == token0 {
            (reserve0, reserve1)
        } else {
            (reserve1, reserve0)
        };
        
        // Both tokens use 6 decimals on Tempo
        Ok(calculate_price_from_reserves(reserve_in, reserve_out, 6, 6))
    }

    /// Get pair info for a base/quote token pair
    pub async fn get_pair_info(
        &self,
        base_token: Address,
        quote_token: Address,
    ) -> Result<PairInfo, Box<dyn std::error::Error + Send + Sync>> {
        let pair_address = self.get_pair(base_token, quote_token).await?
            .ok_or("Pair does not exist")?;
        
        let (reserve0, reserve1, token0, _token1) = self.get_reserves(pair_address).await?;
        
        // Determine which reserve is base and which is quote
        let (base_reserve, quote_reserve) = if base_token == token0 {
            (reserve0, reserve1)
        } else {
            (reserve1, reserve0)
        };
        
        // Calculate price (quote per base) - both use 6 decimals
        let price = calculate_price_from_reserves(base_reserve, quote_reserve, 6, 6);
        let liquidity = calculate_liquidity(reserve0, reserve1);
        
        Ok(PairInfo {
            pair_type: format!("{:?}/{:?}", base_token, quote_token),
            pair_address: format!("{:?}", pair_address),
            base_token: format!("{:?}", base_token),
            quote_token: format!("{:?}", quote_token),
            base_reserve: base_reserve.to_string(),
            quote_reserve: quote_reserve.to_string(),
            price: Some(format!("{:.6}", price)),
            liquidity: Some(format!("{:.0}", liquidity)),
        })
    }
}

/// Get pair info for market tokens
/// Returns pair state for YES and NO tokens against their quote token
pub async fn get_market_pair_info<P: Provider + Clone + 'static>(
    client: &UniV2Client<P>,
    yes_token: Address,
    no_token: Address,
    quote_token: Address,
) -> Result<Vec<PairInfo>, Box<dyn std::error::Error + Send + Sync>> {
    let mut pairs = Vec::new();

    // Get YES token pair
    match client.get_pair_info(yes_token, quote_token).await {
        Ok(mut info) => {
            info.pair_type = "YES/QUOTE".to_string();
            pairs.push(info);
        }
        Err(e) => {
            tracing::debug!("No YES pair found: {}", e);
        }
    }

    // Get NO token pair
    match client.get_pair_info(no_token, quote_token).await {
        Ok(mut info) => {
            info.pair_type = "NO/QUOTE".to_string();
            pairs.push(info);
        }
        Err(e) => {
            tracing::debug!("No NO pair found: {}", e);
        }
    }

    Ok(pairs)
}
