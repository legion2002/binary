/**
 * Seed markets via admin API
 */

import { TEST_API_KEY } from './backend'

// Seed market definitions
const SEED_MARKETS = [
  {
    question: 'Will ETH be above $5000 by end of Q1 2026?',
    resolutionDeadline: Math.floor(Date.now() / 1000) + 90 * 24 * 60 * 60, // 90 days
    assets: ['0x20C0000000000000000000000000000000000000'], // PATH_USD
  },
  {
    question: 'Will BTC reach new ATH within 6 months of 2024 halving?',
    resolutionDeadline: Math.floor(Date.now() / 1000) + 180 * 24 * 60 * 60, // 180 days
    assets: ['0x20C0000000000000000000000000000000000000'],
  },
  {
    question: 'Will Pectra upgrade deploy on mainnet by June 2025?',
    resolutionDeadline: Math.floor(Date.now() / 1000) + 60 * 24 * 60 * 60, // 60 days
    assets: ['0x20C0000000000000000000000000000000000000'],
  },
  {
    question: 'Will stablecoin market cap exceed $200B by end of 2025?',
    resolutionDeadline: Math.floor(Date.now() / 1000) + 365 * 24 * 60 * 60, // 365 days
    assets: ['0x20C0000000000000000000000000000000000000'],
  },
]

export interface SeedMarketsOptions {
  backendUrl: string
  apiKey?: string
}

export async function createSeedMarkets(options: SeedMarketsOptions): Promise<number> {
  const { backendUrl, apiKey = TEST_API_KEY } = options

  let created = 0

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
        created++
      } else {
        const error = await response.text()
        console.error(`  Failed to create market "${market.question.slice(0, 30)}...": ${error}`)
      }
    } catch (error) {
      console.error(`  Error creating market: ${error}`)
    }
  }

  return created
}
