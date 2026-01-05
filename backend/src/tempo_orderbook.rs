use alloy::primitives::{Address, FixedBytes};
use alloy::providers::Provider;
use alloy::sol;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Tempo Stablecoin DEX precompile address
pub const STABLECOIN_EXCHANGE_ADDRESS: Address =
    Address::new([0xde, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

/// pathUSD token address (root quote token)
pub const PATH_USD_ADDRESS: Address =
    Address::new([0x20, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

sol! {
    #[derive(Debug, PartialEq, Eq)]
    struct Order {
        uint128 orderId;
        address maker;
        bytes32 bookKey;
        bool isBid;
        int16 tick;
        uint128 amount;
        uint128 remaining;
        uint128 prev;
        uint128 next;
        bool isFlip;
        int16 flipTick;
    }

    #[sol(rpc)]
    interface IStablecoinExchange {
        // Constants
        function PRICE_SCALE() external view returns (uint32);
        function TICK_SPACING() external view returns (int16);
        function MIN_TICK() external view returns (int16);
        function MAX_TICK() external view returns (int16);

        // Quote functions (view, no gas)
        function quoteSwapExactAmountIn(address tokenIn, address tokenOut, uint128 amountIn)
            external view returns (uint128 amountOut);

        function quoteSwapExactAmountOut(address tokenIn, address tokenOut, uint128 amountOut)
            external view returns (uint128 amountIn);

        // Swap functions
        function swapExactAmountIn(address tokenIn, address tokenOut, uint128 amountIn, uint128 minAmountOut)
            external returns (uint128 amountOut);

        function swapExactAmountOut(address tokenIn, address tokenOut, uint128 amountOut, uint128 maxAmountIn)
            external returns (uint128 amountIn);

        // Order management
        function place(address token, uint128 amount, bool isBid, int16 tick)
            external returns (uint128 orderId);

        function placeFlip(address token, uint128 amount, bool isBid, int16 tick, int16 flipTick)
            external returns (uint128 orderId);

        function cancel(uint128 orderId) external;

        // Pair management
        function createPair(address base) external returns (bytes32 key);

        function pairKey(address tokenA, address tokenB) external pure returns (bytes32 key);

        function books(bytes32 pairKey)
            external view returns (address base, address quote, int16 bestBidTick, int16 bestAskTick);

        // Order queries
        function getTickLevel(address base, int16 tick, bool isBid)
            external view returns (uint128 head, uint128 tail, uint128 totalLiquidity);

        function nextOrderId() external view returns (uint128);

        // Balance management
        function balanceOf(address user, address token) external view returns (uint128);

        function withdraw(address token, uint128 amount) external;

        // Utility
        function tickToPrice(int16 tick) external pure returns (uint32 price);
        function priceToTick(uint32 price) external pure returns (int16 tick);
    }
}

/// Orderbook information for a market token pair
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderbookInfo {
    pub pair_type: String,
    pub pair_key: String,
    pub base_token: String,
    pub quote_token: String,
    pub best_bid_tick: Option<i16>,
    pub best_ask_tick: Option<i16>,
    pub best_bid_price: Option<String>,
    pub best_ask_price: Option<String>,
    pub mid_price: Option<String>,
    pub spread_bps: Option<f64>,
}

/// Orderbook state with liquidity info
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderbookState {
    pub pair_key: String,
    pub base_token: String,
    pub quote_token: String,
    pub best_bid_tick: i16,
    pub best_ask_tick: i16,
    pub bid_liquidity: String,
    pub ask_liquidity: String,
}

/// Price scale constant (from Tempo DEX spec)
pub const PRICE_SCALE: u32 = 100_000;

/// Convert a tick to a price string
/// price = (PRICE_SCALE + tick) / PRICE_SCALE = 1 + tick/100000
pub fn tick_to_price_string(tick: i16) -> String {
    let price = (PRICE_SCALE as i32 + tick as i32) as f64 / PRICE_SCALE as f64;
    format!("{:.6}", price)
}

/// Convert a price to tick
/// tick = (price - 1) * PRICE_SCALE
pub fn price_to_tick(price: f64) -> i16 {
    ((price - 1.0) * PRICE_SCALE as f64).round() as i16
}

/// Calculate mid price from bid and ask ticks
pub fn calculate_mid_price(best_bid_tick: i16, best_ask_tick: i16) -> f64 {
    let bid_price = (PRICE_SCALE as i32 + best_bid_tick as i32) as f64 / PRICE_SCALE as f64;
    let ask_price = (PRICE_SCALE as i32 + best_ask_tick as i32) as f64 / PRICE_SCALE as f64;
    (bid_price + ask_price) / 2.0
}

/// Calculate spread in basis points
pub fn calculate_spread_bps(best_bid_tick: i16, best_ask_tick: i16) -> f64 {
    let bid_price = (PRICE_SCALE as i32 + best_bid_tick as i32) as f64 / PRICE_SCALE as f64;
    let ask_price = (PRICE_SCALE as i32 + best_ask_tick as i32) as f64 / PRICE_SCALE as f64;
    let mid_price = (bid_price + ask_price) / 2.0;
    ((ask_price - bid_price) / mid_price) * 10000.0
}

/// Calculate market probability from YES/NO token mid prices
///
/// The probability is derived from the relative prices of YES and NO tokens.
/// In a prediction market, YES + NO token prices should sum to ~1.0 (the redemption value).
///
/// Returns (yes_probability, no_probability) as values between 0.0 and 1.0
pub fn calculate_probability(
    yes_mid_price: Option<f64>,
    no_mid_price: Option<f64>,
) -> (f64, f64) {
    match (yes_mid_price, no_mid_price) {
        (Some(yes), Some(no)) => {
            let total = yes + no;
            if total > 0.0 {
                (yes / total, no / total)
            } else {
                (0.5, 0.5)
            }
        }
        // If only YES price available, derive NO probability
        (Some(yes), None) => {
            let clamped_yes = yes.clamp(0.0, 1.0);
            (clamped_yes, 1.0 - clamped_yes)
        }
        // If only NO price available, derive YES probability
        (None, Some(no)) => {
            let clamped_no = no.clamp(0.0, 1.0);
            (1.0 - clamped_no, clamped_no)
        }
        // No price data - return 50/50
        (None, None) => (0.5, 0.5),
    }
}

/// Calculate probability from orderbook info
/// Extracts mid prices and calculates probability
pub fn calculate_probability_from_orderbooks(orderbooks: &[OrderbookInfo]) -> (f64, f64) {
    let mut yes_mid: Option<f64> = None;
    let mut no_mid: Option<f64> = None;

    for ob in orderbooks {
        if ob.pair_type == "YES/QUOTE" {
            if let Some(ref mid) = ob.mid_price {
                yes_mid = mid.parse::<f64>().ok();
            }
        } else if ob.pair_type == "NO/QUOTE" {
            if let Some(ref mid) = ob.mid_price {
                no_mid = mid.parse::<f64>().ok();
            }
        }
    }

    calculate_probability(yes_mid, no_mid)
}

/// Sentinel tick values indicating no liquidity
const NO_BID_TICK: i16 = i16::MIN;
const NO_ASK_TICK: i16 = i16::MAX;

pub struct StablecoinExchangeClient<P> {
    provider: Arc<P>,
}

impl<P: Provider + Clone + 'static> StablecoinExchangeClient<P> {
    pub fn new(provider: Arc<P>) -> Self {
        Self { provider }
    }

    /// Get the pair key for two tokens
    pub async fn get_pair_key(
        &self,
        token_a: Address,
        token_b: Address,
    ) -> Result<FixedBytes<32>, Box<dyn std::error::Error + Send + Sync>> {
        let exchange = IStablecoinExchange::new(STABLECOIN_EXCHANGE_ADDRESS, &self.provider);
        let result = exchange.pairKey(token_a, token_b).call().await?;
        Ok(result)
    }

    /// Get orderbook state for a token pair
    pub async fn get_orderbook_state(
        &self,
        base_token: Address,
        quote_token: Address,
    ) -> Result<OrderbookInfo, Box<dyn std::error::Error + Send + Sync>> {
        let exchange = IStablecoinExchange::new(STABLECOIN_EXCHANGE_ADDRESS, &self.provider);

        let pair_key = exchange.pairKey(base_token, quote_token).call().await?;
        let book_result = exchange.books(pair_key).call().await?;

        let best_bid_tick = if book_result.bestBidTick == NO_BID_TICK {
            None
        } else {
            Some(book_result.bestBidTick)
        };

        let best_ask_tick = if book_result.bestAskTick == NO_ASK_TICK {
            None
        } else {
            Some(book_result.bestAskTick)
        };

        let (best_bid_price, best_ask_price, mid_price, spread_bps) =
            match (best_bid_tick, best_ask_tick) {
                (Some(bid), Some(ask)) => {
                    let bid_price = tick_to_price_string(bid);
                    let ask_price = tick_to_price_string(ask);
                    let mid = calculate_mid_price(bid, ask);
                    let spread = calculate_spread_bps(bid, ask);
                    (
                        Some(bid_price),
                        Some(ask_price),
                        Some(format!("{:.6}", mid)),
                        Some(spread),
                    )
                }
                (Some(bid), None) => (Some(tick_to_price_string(bid)), None, None, None),
                (None, Some(ask)) => (None, Some(tick_to_price_string(ask)), None, None),
                (None, None) => (None, None, None, None),
            };

        Ok(OrderbookInfo {
            pair_type: format!("{:?}/{:?}", base_token, quote_token),
            pair_key: format!("0x{}", hex::encode(pair_key)),
            base_token: format!("{:?}", book_result.base),
            quote_token: format!("{:?}", book_result.quote),
            best_bid_tick,
            best_ask_tick,
            best_bid_price,
            best_ask_price,
            mid_price,
            spread_bps,
        })
    }

    /// Quote a swap: how much tokenOut for a given tokenIn amount
    pub async fn quote_swap_exact_in(
        &self,
        token_in: Address,
        token_out: Address,
        amount_in: u128,
    ) -> Result<u128, Box<dyn std::error::Error + Send + Sync>> {
        let exchange = IStablecoinExchange::new(STABLECOIN_EXCHANGE_ADDRESS, &self.provider);
        let result = exchange
            .quoteSwapExactAmountIn(token_in, token_out, amount_in)
            .call()
            .await?;
        Ok(result)
    }

    /// Quote a swap: how much tokenIn needed for a given tokenOut amount
    pub async fn quote_swap_exact_out(
        &self,
        token_in: Address,
        token_out: Address,
        amount_out: u128,
    ) -> Result<u128, Box<dyn std::error::Error + Send + Sync>> {
        let exchange = IStablecoinExchange::new(STABLECOIN_EXCHANGE_ADDRESS, &self.provider);
        let result = exchange
            .quoteSwapExactAmountOut(token_in, token_out, amount_out)
            .call()
            .await?;
        Ok(result)
    }

    /// Get liquidity at a specific tick level
    pub async fn get_tick_liquidity(
        &self,
        base_token: Address,
        tick: i16,
        is_bid: bool,
    ) -> Result<u128, Box<dyn std::error::Error + Send + Sync>> {
        let exchange = IStablecoinExchange::new(STABLECOIN_EXCHANGE_ADDRESS, &self.provider);
        let result = exchange.getTickLevel(base_token, tick, is_bid).call().await?;
        Ok(result.totalLiquidity)
    }

    /// Get a user's balance on the DEX
    pub async fn get_dex_balance(
        &self,
        user: Address,
        token: Address,
    ) -> Result<u128, Box<dyn std::error::Error + Send + Sync>> {
        let exchange = IStablecoinExchange::new(STABLECOIN_EXCHANGE_ADDRESS, &self.provider);
        let result = exchange.balanceOf(user, token).call().await?;
        Ok(result)
    }

    /// Check if a pair exists by querying the books
    pub async fn pair_exists(
        &self,
        base_token: Address,
        quote_token: Address,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let exchange = IStablecoinExchange::new(STABLECOIN_EXCHANGE_ADDRESS, &self.provider);
        let pair_key = exchange.pairKey(base_token, quote_token).call().await?;

        match exchange.books(pair_key).call().await {
            Ok(result) => Ok(result.base != Address::ZERO),
            Err(_) => Ok(false),
        }
    }
}

/// Configuration for which orderbook pairs to create for a market
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OrderbookConfig {
    /// Create YES/quoteToken pair (for trading YES tokens)
    pub create_yes_pair: bool,
    /// Create NO/quoteToken pair (for trading NO tokens)
    pub create_no_pair: bool,
}

impl OrderbookConfig {
    pub fn default_config() -> Self {
        Self {
            create_yes_pair: true,
            create_no_pair: true,
        }
    }
}

/// Result of orderbook pair creation
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedOrderbooks {
    pub yes_pair_key: Option<String>,
    pub no_pair_key: Option<String>,
}

/// Get orderbook info for market tokens
/// Returns orderbook state for YES and NO tokens against their quote token
pub async fn get_market_orderbook_info<P: Provider + Clone + 'static>(
    client: &StablecoinExchangeClient<P>,
    yes_token: Address,
    no_token: Address,
    quote_token: Address,
) -> Result<Vec<OrderbookInfo>, Box<dyn std::error::Error + Send + Sync>> {
    let mut orderbooks = Vec::new();

    // Get YES token orderbook
    match client.get_orderbook_state(yes_token, quote_token).await {
        Ok(mut info) => {
            info.pair_type = "YES/QUOTE".to_string();
            orderbooks.push(info);
        }
        Err(e) => {
            tracing::debug!("No YES orderbook found: {}", e);
        }
    }

    // Get NO token orderbook
    match client.get_orderbook_state(no_token, quote_token).await {
        Ok(mut info) => {
            info.pair_type = "NO/QUOTE".to_string();
            orderbooks.push(info);
        }
        Err(e) => {
            tracing::debug!("No NO orderbook found: {}", e);
        }
    }

    Ok(orderbooks)
}
