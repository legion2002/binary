/**
 * Seed markets via admin API
 */

import { TEST_API_KEY } from './backend'

// Seed market definitions
const SEED_MARKETS = [
    {
    question: 'Will Tempo mainnet launch in 2026?',
    resolutionDeadline: Math.floor(new Date('2027-1-1').getTime() / 1000), // End of 2026
    assets: ['0x20C0000000000000000000000000000000000000'],
  },
  {
    question: 'Will stablecoin market cap exceed $1T by the end of 2026?',
    resolutionDeadline: Math.floor(new Date('2027-1-1').getTime() / 1000), // End of 2026
    assets: ['0x20C0000000000000000000000000000000000000'], // PATH_USD
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
