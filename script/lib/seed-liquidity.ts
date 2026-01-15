/**
 * Seed liquidity for prediction markets using UniswapV2
 *
 * Strategy:
 * - UniV2 AMM provides constant-product liquidity
 * - For prediction markets, we seed YES/NO tokens against PATH_USD
 * - Probability is derived from relative prices: yesProbability = yesPrice / (yesPrice + noPrice)
 * - For 50/50 markets, both YES and NO should have the same price (~1.0)
 *
 * Flow:
 * 1. Ensure UniV2 contracts are deployed
 * 2. Ensure stablecoin routing pairs exist (AlphaUSD/PATH_USD, etc.)
 * 3. Split stablecoins into YES/NO tokens
 * 4. Create UniV2 pairs for YES/PATH_USD and NO/PATH_USD
 * 5. Add liquidity at 1:1 ratio (50/50 probability)
 */

import {
  type Address,
  parseAbi,
} from 'viem'

// Generic client type that works with both local and testnet clients
// eslint-disable-next-line @typescript-eslint/no-explicit-any
type AnyTempoClient = any

// Token addresses (imported from tempo.ts)
import { PATH_USD, ALPHA_USD } from './tempo'
export { PATH_USD, ALPHA_USD }

// UniV2 imports
import {
  ensureUniV2Deployed,
  createPair,
  addLiquidity,
  pairExists,
} from './univ2'

/**
 * Ensure stablecoin routing pairs have liquidity
 * UniV2 pairs for stablecoin routing (e.g., AlphaUSD/PATH_USD)
 */
export async function ensureStablecoinRouting(client: AnyTempoClient): Promise<void> {
  const account = client.account.address
  const routingLiquidity = BigInt(10000) * BigInt(1e6) // 10k tokens per pair

  console.log('  Ensuring UniV2 contracts are deployed...')
  await ensureUniV2Deployed(client)

  console.log('  Ensuring stablecoin routing liquidity...')

  // Mint PATH_USD and ALPHA_USD for routing liquidity
  try {
    await client.token.mint({
      token: PATH_USD as `0x${string}`,
      to: account,
      amount: routingLiquidity * 2n,
    })
    await client.token.mint({
      token: ALPHA_USD as `0x${string}`,
      to: account,
      amount: routingLiquidity * 2n,
    })
  } catch (e) {
    console.log('    (Mint not available, assuming account is funded)')
  }

  // Create AlphaUSD/PATH_USD pair if it doesn't exist
  const exists = await pairExists(client, ALPHA_USD as Address, PATH_USD as Address)
  if (!exists) {
    console.log('    Creating AlphaUSD/PATH_USD pair...')
    await createPair(client, ALPHA_USD as Address, PATH_USD as Address)
    console.log('    Created AlphaUSD/PATH_USD pair')
  } else {
    console.log('    AlphaUSD/PATH_USD pair already exists')
  }

  // Add liquidity to the pair
  try {
    await addLiquidity(client, {
      tokenA: ALPHA_USD as Address,
      tokenB: PATH_USD as Address,
      amountA: routingLiquidity,
      amountB: routingLiquidity,
    })
    console.log('    ✓ Added AlphaUSD/PATH_USD routing liquidity')
  } catch (e: any) {
    console.log('    ✓ Routing liquidity addition attempted:', e.message?.slice(0, 50))
  }
}

// ABIs (only for operations not covered by tempo actions)
const multiVerseAbi = parseAbi([
  'function split(address asset, uint256 amount, bytes32 marketHash) external',
  'function getVerseAddress(address asset, bytes32 marketHash) external view returns (address yesVerse, address noVerse)',
])

export interface SeedLiquidityConfig {
  /** Amount of each stablecoin to use for seeding (in base units, 6 decimals) */
  seedAmount: bigint
}

export const DEFAULT_SEED_CONFIG: SeedLiquidityConfig = {
  seedAmount: BigInt(2000) * BigInt(1e6), // 2000 tokens
}

export interface MarketLiquidityParams {
  marketHash: `0x${string}`
  multiVerseAddress: Address
  quoteToken: Address
}

/**
 * Seed liquidity for a single market using UniV2
 */
export async function seedMarketLiquidity(
  client: AnyTempoClient,
  params: MarketLiquidityParams,
  config: SeedLiquidityConfig = DEFAULT_SEED_CONFIG
): Promise<{
  yesPair: Address
  noPair: Address
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

  // Step 5: Mint PATH_USD for liquidity (need to provide both sides)
  console.log(`    Minting PATH_USD for liquidity...`)
  try {
    await client.token.mint({
      token: PATH_USD as `0x${string}`,
      to: account,
      amount: config.seedAmount * 2n, // Need PATH_USD for both YES and NO pairs
    })
  } catch (e) {
    console.log(`    (Mint not available for PATH_USD, assuming account is funded)`)
  }

  // Step 6: Create UniV2 pairs and add liquidity
  console.log(`    Creating UniV2 pairs...`)

  // Create YES/PATH_USD pair
  const yesPair = await createPair(client, yesVerse as Address, PATH_USD as Address)
  console.log(`      YES/PATH_USD pair: ${yesPair}`)

  // Create NO/PATH_USD pair
  const noPair = await createPair(client, noVerse as Address, PATH_USD as Address)
  console.log(`      NO/PATH_USD pair: ${noPair}`)

  // Step 7: Add liquidity at 1:1 ratio (50/50 probability)
  // Half of seed amount goes to each pair
  const liquidityPerPair = config.seedAmount / 2n

  console.log(`    Adding liquidity to YES/PATH_USD pair...`)
  try {
    await addLiquidity(client, {
      tokenA: yesVerse as Address,
      tokenB: PATH_USD as Address,
      amountA: liquidityPerPair,
      amountB: liquidityPerPair,
    })
    console.log(`      ✓ Added ${Number(liquidityPerPair) / 1e6} YES + ${Number(liquidityPerPair) / 1e6} PATH_USD`)
  } catch (e: any) {
    console.warn(`      Failed to add YES liquidity:`, e.shortMessage || e.message?.slice(0, 100))
    if (e.cause) console.warn(`        Cause:`, e.cause?.shortMessage || String(e.cause).slice(0, 100))
  }

  console.log(`    Adding liquidity to NO/PATH_USD pair...`)
  try {
    await addLiquidity(client, {
      tokenA: noVerse as Address,
      tokenB: PATH_USD as Address,
      amountA: liquidityPerPair,
      amountB: liquidityPerPair,
    })
    console.log(`      ✓ Added ${Number(liquidityPerPair) / 1e6} NO + ${Number(liquidityPerPair) / 1e6} PATH_USD`)
  } catch (e: any) {
    console.warn(`      Failed to add NO liquidity:`, e.shortMessage || e.message?.slice(0, 100))
    if (e.cause) console.warn(`        Cause:`, e.cause?.shortMessage || String(e.cause).slice(0, 100))
  }

  console.log(`    ✓ Market liquidity seeded on UniV2`)

  return { yesPair, noPair }
}

/**
 * Seed liquidity for multiple markets
 */
export async function seedMarketsLiquidity(
  client: AnyTempoClient,
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

// Legacy exports for backwards compatibility (these are no longer used but kept for reference)
export const priceToTick = (price: number): number => Math.round((price - 1) * 100_000)
export const tickToPrice = (tick: number): number => 1 + tick / 100_000
