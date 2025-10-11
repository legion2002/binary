use alloy::primitives::{Address, FixedBytes, U160, U256, I24};
use alloy::providers::Provider;
use alloy::sol;
use alloy::sol_types::SolValue;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// Define V4 types and interfaces
sol! {
    #[derive(Debug, PartialEq, Eq)]
    struct PoolKey {
        address currency0;
        address currency1;
        uint24 fee;
        int24 tickSpacing;
        address hooks;
    }

    #[sol(rpc)]
    interface IPoolManager {
        function initialize(
            PoolKey memory key,
            uint160 sqrtPriceX96
        ) external returns (int24 tick);

        function getLiquidity(
            bytes32 id
        ) external view returns (uint128 liquidity);

        function getSlot0(
            bytes32 id
        ) external view returns (
            uint160 sqrtPriceX96,
            int24 tick,
            uint24 protocolFee,
            uint24 lpFee
        );
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V4PoolInfo {
    pub pool_type: String,
    pub pool_id: String,
    pub token0: String,
    pub token1: String,
    pub fee: u32,
    pub tick_spacing: i32,
    pub liquidity: Option<String>,
    pub sqrt_price_x96: Option<String>,
    pub tick: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct V4PoolState {
    pub pool_id: String,
    pub liquidity: String,
    pub sqrt_price_x96: String,
    pub tick: i32,
    pub protocol_fee: u32,
    pub lp_fee: u32,
}

pub struct V4PoolManager<P> {
    pool_manager_address: Address,
    provider: Arc<P>,
}

impl<P: Provider + Clone + 'static> V4PoolManager<P> {
    pub fn new(pool_manager_address: Address, provider: Arc<P>) -> Self {
        Self {
            pool_manager_address,
            provider,
        }
    }

    /// Compute PoolId from PoolKey
    pub fn compute_pool_id(key: &PoolKey) -> FixedBytes<32> {
        let encoded = key.abi_encode();
        alloy::primitives::keccak256(&encoded).into()
    }

    /// Sort tokens for V4 (currency0 must be < currency1)
    pub fn sort_currencies(token0: Address, token1: Address) -> (Address, Address) {
        if token0 < token1 {
            (token0, token1)
        } else {
            (token1, token0)
        }
    }

    /// Create a PoolKey for a token pair
    pub fn create_pool_key(
        token0: Address,
        token1: Address,
        fee: u32,
        tick_spacing: i32,
        hooks: Option<Address>,
    ) -> PoolKey {
        let (currency0, currency1) = Self::sort_currencies(token0, token1);
        PoolKey {
            currency0,
            currency1,
            fee: fee as u32,
            tickSpacing: I24::try_from(tick_spacing).expect("Invalid tick spacing"),
            hooks: hooks.unwrap_or(Address::ZERO),
        }
    }

    /// Initialize a pool with given parameters
    pub async fn initialize_pool(
        &self,
        key: PoolKey,
        sqrt_price_x96: U160,
    ) -> Result<I24, Box<dyn std::error::Error>> {
        let pool_manager = IPoolManager::new(self.pool_manager_address, &self.provider);

        let tick = pool_manager
            .initialize(key, sqrt_price_x96)
            .send()
            .await?
            .get_receipt()
            .await?;

        // Parse tick from receipt/logs if needed
        // For now, return a default value
        Ok(I24::try_from(0).unwrap())
    }

    /// Initialize a pool, or return existing pool ID if already initialized
    pub async fn initialize_or_get_pool(
        &self,
        key: PoolKey,
        sqrt_price_x96: U160,
    ) -> Result<FixedBytes<32>, Box<dyn std::error::Error>> {
        let pool_id = Self::compute_pool_id(&key);

        // Check if pool already exists
        if self.pool_exists(pool_id).await? {
            tracing::info!("Pool already exists: {}", hex::encode(pool_id));
            return Ok(pool_id);
        }

        // Initialize the pool
        match self.initialize_pool(key, sqrt_price_x96).await {
            Ok(_) => {
                tracing::info!("Pool initialized: {}", hex::encode(pool_id));
                Ok(pool_id)
            }
            Err(e) => {
                // Pool might already exist (race condition), check again
                if self.pool_exists(pool_id).await? {
                    tracing::info!("Pool already exists (race condition): {}", hex::encode(pool_id));
                    Ok(pool_id)
                } else {
                    Err(e)
                }
            }
        }
    }

    /// Check if a pool exists
    pub async fn pool_exists(&self, pool_id: FixedBytes<32>) -> Result<bool, Box<dyn std::error::Error>> {
        let pool_manager = IPoolManager::new(self.pool_manager_address, &self.provider);

        // A pool exists if it has been initialized (has a non-zero sqrtPrice)
        let result = pool_manager.getSlot0(pool_id).call().await?;
        Ok(result.sqrtPriceX96 != U160::ZERO)
    }

    /// Get the current state of a pool
    pub async fn get_pool_state(&self, pool_id: FixedBytes<32>) -> Result<V4PoolState, Box<dyn std::error::Error>> {
        let pool_manager = IPoolManager::new(self.pool_manager_address, &self.provider);

        // Get liquidity
        let liquidity_result = pool_manager.getLiquidity(pool_id).call().await?;

        // Get slot0 (price, tick, fees)
        let slot0_result = pool_manager.getSlot0(pool_id).call().await?;

        Ok(V4PoolState {
            pool_id: format!("0x{}", hex::encode(pool_id)),
            liquidity: liquidity_result._0.to_string(),
            sqrt_price_x96: slot0_result.sqrtPriceX96.to_string(),
            tick: slot0_result.tick.as_i32(),
            protocol_fee: slot0_result.protocolFee,
            lp_fee: slot0_result.lpFee,
        })
    }
}

/// Pool configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    pub create_yes0_yes1: bool,   // YES_TOKEN0/YES_TOKEN1 pool
    pub create_no0_no1: bool,     // NO_TOKEN0/NO_TOKEN1 pool
    pub create_yes0_no0: bool,    // YES_TOKEN0/NO_TOKEN0 pool
    pub create_yes1_no1: bool,    // YES_TOKEN1/NO_TOKEN1 pool
}

impl Default for PoolConfig {
    fn default() -> Self {
        // Default to the 3 main pools
        Self {
            create_yes0_yes1: true,   // YES economy
            create_no0_no1: true,     // NO economy
            create_yes0_no0: true,    // Market outcome trading
            create_yes1_no1: false,   // Optional cross-token trading
        }
    }
}

/// Result of pool creation
#[derive(Debug, Clone, Serialize)]
pub struct CreatedPools {
    pub yes0_yes1: Option<FixedBytes<32>>,
    pub no0_no1: Option<FixedBytes<32>>,
    pub yes0_no0: Option<FixedBytes<32>>,
    pub yes1_no1: Option<FixedBytes<32>>,
}

/// Create configured pools for a prediction market
pub async fn create_market_v4_pools<P: Provider + Clone + 'static>(
    pool_manager: &V4PoolManager<P>,
    yes_token0: Address,
    no_token0: Address,
    yes_token1: Address,
    no_token1: Address,
    config: PoolConfig,
) -> Result<CreatedPools, Box<dyn std::error::Error>> {
    // Standard parameters for all pools
    const FEE: u32 = 3000; // 0.3%
    const TICK_SPACING: i32 = 60;

    // Starting price: 1:1 ratio
    let sqrt_price_x96 = U160::from(79228162514264337593543950336u128);

    let mut result = CreatedPools {
        yes0_yes1: None,
        no0_no1: None,
        yes0_no0: None,
        yes1_no1: None,
    };

    // Create YES_TOKEN0/YES_TOKEN1 pool if configured
    if config.create_yes0_yes1 {
        let key = V4PoolManager::<P>::create_pool_key(
            yes_token0,
            yes_token1,
            FEE,
            TICK_SPACING,
            None,
        );
        result.yes0_yes1 = Some(pool_manager.initialize_or_get_pool(key, sqrt_price_x96).await?);
        tracing::info!("Created YES_TOKEN0/YES_TOKEN1 pool: {}", hex::encode(result.yes0_yes1.unwrap()));
    }

    // Create NO_TOKEN0/NO_TOKEN1 pool if configured
    if config.create_no0_no1 {
        let key = V4PoolManager::<P>::create_pool_key(
            no_token0,
            no_token1,
            FEE,
            TICK_SPACING,
            None,
        );
        result.no0_no1 = Some(pool_manager.initialize_or_get_pool(key, sqrt_price_x96).await?);
        tracing::info!("Created NO_TOKEN0/NO_TOKEN1 pool: {}", hex::encode(result.no0_no1.unwrap()));
    }

    // Create YES_TOKEN0/NO_TOKEN0 pool if configured
    if config.create_yes0_no0 {
        let key = V4PoolManager::<P>::create_pool_key(
            yes_token0,
            no_token0,
            FEE,
            TICK_SPACING,
            None,
        );
        result.yes0_no0 = Some(pool_manager.initialize_or_get_pool(key, sqrt_price_x96).await?);
        tracing::info!("Created YES_TOKEN0/NO_TOKEN0 pool: {}", hex::encode(result.yes0_no0.unwrap()));
    }

    // Create YES_TOKEN1/NO_TOKEN1 pool if configured
    if config.create_yes1_no1 {
        let key = V4PoolManager::<P>::create_pool_key(
            yes_token1,
            no_token1,
            FEE,
            TICK_SPACING,
            None,
        );
        result.yes1_no1 = Some(pool_manager.initialize_or_get_pool(key, sqrt_price_x96).await?);
        tracing::info!("Created YES_TOKEN1/NO_TOKEN1 pool: {}", hex::encode(result.yes1_no1.unwrap()));
    }

    Ok(result)
}