#!/usr/bin/env bun
/**
 * Seed production backend with markets and liquidity
 *
 * This script runs the same initialization as env-testnet.ts but against
 * a deployed backend (no local services started).
 *
 * Usage:
 *   PRIVATE_KEY=0x... bun run script/seed-production.ts
 *   PRIVATE_KEY=0x... BACKEND_URL=https://custom-backend.fly.dev bun run script/seed-production.ts
 *
 * Environment:
 *   PRIVATE_KEY  - Required. Account with funds on Tempo testnet
 *   BACKEND_URL  - Optional. Defaults to https://binary-markets-api.fly.dev
 *   API_KEY      - Optional. Defaults to test_api_key_12345
 */

import { createSeedMarkets } from './lib/seed-markets'
import {
  seedMarketLiquidity,
  DEFAULT_SEED_CONFIG,
  ensureStablecoinRouting,
} from './lib/seed-liquidity'
import { getTestnetClient, TESTNET_RPC_URL, TESTNET_CHAIN_ID } from './lib/testnet'
import { TEST_API_KEY } from './lib/backend'
import type { Address } from 'viem'
import deployments from '../deployments.json'

const cyan = (s: string) => `\x1b[36m${s}\x1b[0m`
const green = (s: string) => `\x1b[32m${s}\x1b[0m`
const yellow = (s: string) => `\x1b[33m${s}\x1b[0m`
const red = (s: string) => `\x1b[31m${s}\x1b[0m`

const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms))
const RATE_LIMIT_DELAY = 2000 // 2 seconds between RPC-heavy operations

const BACKEND_URL = process.env.BACKEND_URL || 'https://binary-markets-api.fly.dev'
const API_KEY = process.env.API_KEY || TEST_API_KEY

function getPrivateKey(): `0x${string}` {
  const pk = process.env.PRIVATE_KEY
  if (!pk) {
    console.error(red('Error: PRIVATE_KEY environment variable not set'))
    console.error('')
    console.error('Usage:')
    console.error('  PRIVATE_KEY=0x... bun run production:seed')
    process.exit(1)
  }
  return pk.startsWith('0x') ? (pk as `0x${string}`) : (`0x${pk}` as `0x${string}`)
}

async function main() {
  console.log('')
  console.log(cyan('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━'))
  console.log(cyan('  Binary Markets - Production Seed'))
  console.log(cyan('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━'))
  console.log('')
  console.log(`  Backend:  ${BACKEND_URL}`)
  console.log(`  RPC:      ${TESTNET_RPC_URL}`)
  console.log('')

  const privateKey = getPrivateKey()
  const client = getTestnetClient(privateKey)
  console.log(`  Account:  ${client.account.address}`)

  // Load contract addresses from deployments.json
  const deployment = deployments[String(TESTNET_CHAIN_ID) as keyof typeof deployments]
  if (!deployment?.multiverse) {
    console.error(red('Error: No multiverse address in deployments.json'))
    console.error('Run bun run update-deployments first.')
    process.exit(1)
  }
  const multiverseAddress = deployment.multiverse as Address
  console.log(`  MultiVerse: ${multiverseAddress}`)
  console.log('')

  // Step 1: Verify backend is accessible
  console.log(cyan('Step 1:') + ' Checking backend health...')
  try {
    const res = await fetch(`${BACKEND_URL}/markets`)
    if (!res.ok) throw new Error(`HTTP ${res.status}`)
    console.log(green('  ✓ Backend is healthy'))
  } catch (e: any) {
    console.error(red(`  ✗ Backend unreachable: ${e.message}`))
    process.exit(1)
  }

  // Step 2: Seed markets via admin API
  console.log('')
  console.log(cyan('Step 2:') + ' Seeding markets via admin API...')
  const seedResult = await createSeedMarkets({
    backendUrl: BACKEND_URL,
    apiKey: API_KEY,
    verbose: true,
  })
  if (seedResult.count > 0) {
    console.log(green(`  ✓ Created ${seedResult.count} new markets`))
  } else if (seedResult.markets.length > 0) {
    console.log(green(`  ✓ ${seedResult.markets.length} markets already exist`))
  } else {
    console.log(yellow('  ⚠ No markets created'))
  }

  // Step 3: Ensure stablecoin routing
  console.log('')
  console.log(cyan('Step 3:') + ' Setting up stablecoin routing...')
  try {
    await ensureStablecoinRouting(client)
    console.log(green('  ✓ Stablecoin routing configured'))
  } catch (e: any) {
    console.log(yellow(`  ⚠ ${e.message?.slice(0, 60)}...`))
  }

  // Wait before next batch of RPC calls
  await sleep(RATE_LIMIT_DELAY)

  // Step 4: Seed liquidity for markets
  console.log('')
  console.log(cyan('Step 4:') + ' Seeding market liquidity...')

  const marketsRes = await fetch(`${BACKEND_URL}/markets`)
  const marketsData = (await marketsRes.json()) as { markets: Array<{ marketHash: string }> }
  let liquiditySeeded = 0

  for (const market of marketsData.markets || []) {
    const verseRes = await fetch(`${BACKEND_URL}/markets/${market.marketHash}/verse-tokens`)
    const verseTokens = (await verseRes.json()) as Array<{
      asset: string
      yesVerse: string
      noVerse: string
    }>

    for (const vt of verseTokens || []) {
      try {
        console.log(`  Seeding ${market.marketHash.slice(0, 10)}... with ${vt.asset.slice(0, 10)}...`)
        await seedMarketLiquidity(
          client,
          {
            marketHash: market.marketHash as `0x${string}`,
            multiVerseAddress: multiverseAddress,
            quoteToken: vt.asset as Address,
          },
          DEFAULT_SEED_CONFIG
        )
        liquiditySeeded++
        console.log(green(`    ✓ Done`))
      } catch (e: any) {
        console.log(yellow(`    ⚠ ${e.message?.slice(0, 60)}`))
      }
      // Rate limit protection between each market/asset pair
      await sleep(RATE_LIMIT_DELAY)
    }
  }

  console.log('')
  console.log(green(`  ✓ Seeded liquidity for ${liquiditySeeded} market/asset pairs`))

  // Done
  console.log('')
  console.log(cyan('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━'))
  console.log(green('  Production seeding complete!'))
  console.log(cyan('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━'))
  console.log('')
}

main().catch((err) => {
  console.error(red('Fatal error:'), err.message)
  process.exit(1)
})
