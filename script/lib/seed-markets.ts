/**
 * Seed markets via admin API
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

export async function createSeedMarkets(options: SeedMarketsOptions): Promise<SeedMarketsResult> {
  const { backendUrl, apiKey = TEST_API_KEY } = options

  const result: SeedMarketsResult = {
    count: 0,
    markets: [],
  }

  for (const market of SEED_MARKETS) {
    try {
      const response = await fetch(`${backendUrl}/admin/markets/open`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${apiKey}`,
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
      } else {
        const error = await response.text()
        console.error(`  Failed to create market "${market.question.slice(0, 30)}...": ${error}`)
      }
    } catch (error) {
      console.error(`  Error creating market: ${error}`)
    }
  }

  return result
}
