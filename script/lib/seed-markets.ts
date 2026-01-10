/**
 * Seed markets via admin API
 *
 * Flow:
 * 1. Fetch existing markets from the database
 * 2. For each seed market, check if a similar market already exists
 * 3. If not, try to open the market via /admin/markets/open
 * 4. If market already exists on-chain but not in DB, use /admin/markets/add
 */

import { TEST_API_KEY } from './backend'
import { PATH_USD, ALPHA_USD } from './tempo'

// Seed market definitions - each market now includes both PATH_USD and ALPHA_USD
const SEED_MARKETS = [
  {
    question: 'Will Tempo mainnet launch in 2026?',
    resolutionDeadline: Math.floor(new Date('2027-1-1').getTime() / 1000), // End of 2026
    assets: [PATH_USD, ALPHA_USD],
  },
  {
    question: 'Will stablecoin market cap exceed $1T by the end of 2026?',
    resolutionDeadline: Math.floor(new Date('2027-1-1').getTime() / 1000), // End of 2026
    assets: [PATH_USD, ALPHA_USD],
  },
]

export interface SeedMarketsOptions {
  backendUrl: string
  apiKey?: string
  oracleAddress?: string
  verbose?: boolean
}

export interface CreatedMarket {
  marketHash: string
  question: string
  assets: string[]
}

export interface SeedMarketsResult {
  count: number
  markets: CreatedMarket[]
}

interface ExistingMarket {
  marketHash: string
  question: string
}

/**
 * Fetch existing markets from the backend
 */
async function getExistingMarkets(backendUrl: string): Promise<ExistingMarket[]> {
  try {
    const response = await fetch(`${backendUrl}/markets`)
    if (!response.ok) return []
    const data = (await response.json()) as { markets: ExistingMarket[] }
    return data.markets || []
  } catch {
    return []
  }
}

/**
 * Check if a market with the same question already exists
 */
function findExistingMarket(
  existingMarkets: ExistingMarket[],
  question: string
): ExistingMarket | undefined {
  return existingMarkets.find((m) => m.question === question)
}

export async function createSeedMarkets(options: SeedMarketsOptions): Promise<SeedMarketsResult> {
  const { backendUrl, apiKey = TEST_API_KEY, verbose = false } = options

  const result: SeedMarketsResult = {
    count: 0,
    markets: [],
  }

  // First, fetch existing markets from database
  const existingMarkets = await getExistingMarkets(backendUrl)
  if (verbose && existingMarkets.length > 0) {
    console.log(`  Found ${existingMarkets.length} existing markets in database`)
  }

  for (const market of SEED_MARKETS) {
    try {
      // Check if market already exists in database
      const existing = findExistingMarket(existingMarkets, market.question)
      if (existing) {
        if (verbose) {
          console.log(`  ✓ "${market.question.slice(0, 40)}..." already in database`)
        }
        result.markets.push({
          marketHash: existing.marketHash,
          question: market.question,
          assets: market.assets,
        })
        continue
      }

      if (verbose) {
        console.log(`  Creating market: "${market.question.slice(0, 40)}..."`)
      }

      // Try to open the market
      const response = await fetch(`${backendUrl}/admin/markets/open`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${apiKey}`,
        },
        body: JSON.stringify(market),
      })

      if (response.ok) {
        const data = await response.json()
        result.count++
        result.markets.push({
          marketHash: data.marketHash,
          question: market.question,
          assets: market.assets,
        })
        if (verbose) {
          console.log(`    ✓ Created: ${data.marketHash.slice(0, 18)}...`)
        }
      } else {
        const error = await response.text()

        // Check if market already exists on-chain (MarketAlreadyOpened error)
        // 0x6f4b7b36 is the selector for MarketAlreadyOpened()
        const isAlreadyOpened =
          error.includes('already exists') ||
          error.includes('MarketAlreadyOpened') ||
          error.includes('0x6f4b7b36')

        if (isAlreadyOpened) {
          if (verbose) {
            console.log(`    ⚠ Market exists on-chain but not in database`)
            console.log(`      This can happen if the market was created with different parameters`)
            console.log(`      Consider redeploying contracts with: bun run testnet:redeploy`)
          }
        } else {
          console.error(`  Failed: ${error.slice(0, 100)}`)
        }
      }
    } catch (error) {
      console.error(`  Error creating market: ${error}`)
    }
  }

  return result
}
