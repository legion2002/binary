/**
 * Seed liquidity for prediction markets on the Stablecoin DEX
 *
 * Strategy:
 * - The Tempo DEX only supports ±2% from peg (tick range ±2000)
 * - For prediction markets, we seed YES/NO tokens at prices near 1.0
 * - Probability is derived from relative prices: yesProbability = yesPrice / (yesPrice + noPrice)
 * - For 50/50 markets, both YES and NO should have the same price (~1.0)
 *
 * Flow:
 * 1. Split stablecoins into YES/NO tokens
 * 2. Create DEX pairs for YES/quote and NO/quote
 * 3. Place symmetric bid/ask orders around the peg
 */

import {
  type Address,
  parseAbi,
} from 'viem'
import type { TempoClient } from './tempo'

// Stablecoin DEX precompile address
const STABLECOIN_DEX = '0xDEc0000000000000000000000000000000000000' as const

// Token addresses (imported from tempo.ts)
import { PATH_USD, ALPHA_USD } from './tempo'
export { PATH_USD, ALPHA_USD }

// DEX constants
const PRICE_SCALE = 100_000
const TICK_SPACING = 10

// ABIs (only for operations not covered by tempo actions)
const multiVerseAbi = parseAbi([
  'function split(address asset, uint256 amount, bytes32 marketHash) external',
  'function getVerseAddress(address asset, bytes32 marketHash) external view returns (address yesVerse, address noVerse)',
])

export interface SeedLiquidityConfig {
  /** Amount of each stablecoin to use for seeding (in base units, 6 decimals) */
  seedAmount: bigint
  /** Number of price levels to seed on each side */
  numLevels: number
  /** Tick step between levels (must be multiple of TICK_SPACING=10) */
  tickStep: number
  /** Amount per order level (in base units) */
  amountPerLevel: bigint
}

// Minimum order size on the DEX is 100 tokens (100_000_000 with 6 decimals)
const MIN_ORDER_SIZE = 100_000_000n

export const DEFAULT_SEED_CONFIG: SeedLiquidityConfig = {
  seedAmount: BigInt(2000) * BigInt(1e6), // 2000 tokens (enough for 5 levels * 2 sides * 2 tokens)
  numLevels: 3, // Fewer levels to stay above minimum
  tickStep: 20, // 2 basis points per level
  amountPerLevel: BigInt(150) * BigInt(1e6), // 150 tokens per level (above 100 min)
}

export interface MarketLiquidityParams {
  marketHash: `0x${string}`
  multiVerseAddress: Address
  quoteToken: Address
}

/**
 * Convert a price to a tick value
 * Formula: tick = (price - 1) * PRICE_SCALE
 */
export function priceToTick(price: number): number {
  const tick = Math.round((price - 1) * PRICE_SCALE)
  // Round to nearest tick spacing
  return Math.round(tick / TICK_SPACING) * TICK_SPACING
}

/**
 * Convert a tick to a price value
 * Formula: price = 1 + tick / PRICE_SCALE
 */
export function tickToPrice(tick: number): number {
  return 1 + tick / PRICE_SCALE
}

/**
 * Seed liquidity for a single market
 */
export async function seedMarketLiquidity(
  client: TempoClient,
  params: MarketLiquidityParams,
  config: SeedLiquidityConfig = DEFAULT_SEED_CONFIG
): Promise<{
  yesOrderIds: bigint[]
  noOrderIds: bigint[]
}> {
  const { marketHash, multiVerseAddress, quoteToken } = params
  const account = client.account.address

  console.log(`  Seeding liquidity for market ${marketHash.slice(0, 10)}...`)

  // Step 1: Get verse token addresses
  const [yesVerse, noVerse] = await client.readContract({
    address: multiVerseAddress,
    abi: multiVerseAbi,
    functionName: 'getVerseAddress',
    args: [quoteToken, marketHash],
  })

  if (yesVerse === '0x0000000000000000000000000000000000000000') {
    throw new Error(`No verse tokens found for market ${marketHash} with asset ${quoteToken}`)
  }

  console.log(`    YES token: ${yesVerse}`)
  console.log(`    NO token: ${noVerse}`)

  // Step 2: Mint tokens to the seeder account (works on local devnet)
  console.log(`    Minting ${Number(config.seedAmount) / 1e6} ${quoteToken.slice(0, 10)}... to seeder...`)
  try {
    await client.token.mint({
      token: quoteToken,
      to: account,
      amount: config.seedAmount,
    })
  } catch (e) {
    console.log(`    (Mint not available, assuming account is already funded)`)
  }

  // Step 3: Approve MultiVerse to spend quote token for splitting
  console.log(`    Approving MultiVerse to spend ${quoteToken}...`)
  await client.token.approve({
    token: quoteToken,
    spender: multiVerseAddress,
    amount: config.seedAmount * 10n,
    feeToken: PATH_USD,
  })
  
  // Wait for approval to be confirmed
  await new Promise(r => setTimeout(r, 500))

  // Step 4: Split quote tokens into YES/NO
  console.log(`    Splitting ${Number(config.seedAmount) / 1e6} tokens into YES/NO...`)
  await client.writeContract({
    address: multiVerseAddress,
    abi: multiVerseAbi,
    functionName: 'split',
    args: [quoteToken, config.seedAmount, marketHash],
    feeToken: PATH_USD,
  })

  // Step 5: Create DEX pairs for YES and NO tokens
  console.log(`    Creating DEX pairs...`)
  try {
    await client.dex.createPair({
      base: yesVerse,
      feeToken: PATH_USD,
    })
    console.log(`      Created YES/quote pair`)
  } catch (e: unknown) {
    const error = e as Error
    if (error.message?.includes('PAIR_EXISTS') || error.message?.includes('PairAlreadyExists')) {
      console.log(`      YES/quote pair already exists`)
    } else {
      throw e
    }
  }

  try {
    await client.dex.createPair({
      base: noVerse,
      feeToken: PATH_USD,
    })
    console.log(`      Created NO/quote pair`)
  } catch (e: unknown) {
    const error = e as Error
    if (error.message?.includes('PAIR_EXISTS') || error.message?.includes('PairAlreadyExists')) {
      console.log(`      NO/quote pair already exists`)
    } else {
      throw e
    }
  }

  // Step 5: Mint additional PATH_USD for bid orders (bids require quote token)
  console.log(`    Minting PATH_USD for bid orders...`)
  try {
    await client.token.mint({
      token: PATH_USD as `0x${string}`,
      to: account,
      amount: config.seedAmount,
    })
  } catch (e) {
    console.log(`    (Mint not available for PATH_USD, assuming account is funded)`)
  }

  // Step 6: Approve DEX to spend YES, NO, and PATH_USD tokens
  // Use client.token.approve for proper TIP-20 approval handling
  const maxApproval = BigInt(2) ** BigInt(128) - 1n // Max uint128
  console.log(`    Approving DEX to spend tokens...`)
  await client.token.approve({
    token: yesVerse,
    spender: STABLECOIN_DEX,
    amount: maxApproval,
    feeToken: PATH_USD,
  })
  await client.token.approve({
    token: noVerse,
    spender: STABLECOIN_DEX,
    amount: maxApproval,
    feeToken: PATH_USD,
  })
  await client.token.approve({
    token: PATH_USD as `0x${string}`,
    spender: STABLECOIN_DEX,
    amount: maxApproval,
    feeToken: PATH_USD,
  })
  
  // Wait for approvals to be confirmed
  await new Promise(r => setTimeout(r, 1000))

  // Step 7: Place orders for YES and NO tokens
  // We place symmetric orders around tick=0 (price=1.0)
  // For 50/50 probability: YES and NO should have same prices
  const yesOrderIds: bigint[] = []
  const noOrderIds: bigint[] = []

  console.log(`    Placing ${config.numLevels} levels of orders...`)

  for (let level = 0; level < config.numLevels; level++) {
    // Calculate ticks for this level
    // Bids are below peg (negative ticks), asks are above peg (positive ticks)
    const bidOffset = (level + 1) * config.tickStep
    const askOffset = (level + 1) * config.tickStep

    const bidTick = -bidOffset as unknown as number
    const askTick = askOffset as unknown as number

    // Calculate amount with decay (less liquidity at outer levels)
    // But ensure we stay above minimum order size
    const decay = Math.pow(0.9, level) // 10% decay per level (gentler)
    const rawAmount = BigInt(Math.floor(Number(config.amountPerLevel) * decay))
    const levelAmount = rawAmount > MIN_ORDER_SIZE ? rawAmount : MIN_ORDER_SIZE

    // Place YES bids (buy YES with quote token)
    try {
      const result = await client.dex.placeSync({
        token: yesVerse,
        amount: levelAmount,
        type: 'buy',
        tick: bidTick,
        feeToken: PATH_USD,
      })
      yesOrderIds.push(result.orderId)
    } catch (e) {
      console.warn(`      Failed to place YES bid at tick ${bidTick}:`, e)
    }

    // Place YES asks (sell YES for quote token)
    try {
      const result = await client.dex.placeSync({
        token: yesVerse,
        amount: levelAmount,
        type: 'sell',
        tick: askTick,
        feeToken: PATH_USD,
      })
      yesOrderIds.push(result.orderId)
    } catch (e) {
      console.warn(`      Failed to place YES ask at tick ${askTick}:`, e)
    }

    // Place NO bids (buy NO with quote token)
    try {
      const result = await client.dex.placeSync({
        token: noVerse,
        amount: levelAmount,
        type: 'buy',
        tick: bidTick,
        feeToken: PATH_USD,
      })
      noOrderIds.push(result.orderId)
    } catch (e) {
      console.warn(`      Failed to place NO bid at tick ${bidTick}:`, e)
    }

    // Place NO asks (sell NO for quote token)
    try {
      const result = await client.dex.placeSync({
        token: noVerse,
        amount: levelAmount,
        type: 'sell',
        tick: askTick,
        feeToken: PATH_USD,
      })
      noOrderIds.push(result.orderId)
    } catch (e) {
      console.warn(`      Failed to place NO ask at tick ${askTick}:`, e)
    }

    console.log(`      Level ${level + 1}: bid@${tickToPrice(bidTick).toFixed(4)} ask@${tickToPrice(askTick).toFixed(4)} amount=${Number(levelAmount) / 1e6}`)
  }

  console.log(`    ✓ Placed ${yesOrderIds.length} YES orders and ${noOrderIds.length} NO orders`)

  return { yesOrderIds, noOrderIds }
}

/**
 * Seed liquidity for multiple markets
 */
export async function seedMarketsLiquidity(
  client: TempoClient,
  markets: MarketLiquidityParams[],
  config: SeedLiquidityConfig = DEFAULT_SEED_CONFIG
): Promise<void> {
  console.log(`Seeding liquidity for ${markets.length} markets...`)

  for (const market of markets) {
    try {
      await seedMarketLiquidity(client, market, config)
    } catch (e) {
      console.error(`  Failed to seed market ${market.marketHash}:`, e)
    }
  }

  console.log(`✓ Liquidity seeding complete`)
}
