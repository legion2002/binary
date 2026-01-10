#!/usr/bin/env bun
/**
 * Seed liquidity for markets on testnet
 * 
 * This script creates DEX pairs and seeds liquidity for verse tokens on testnet.
 * 
 * Usage:
 *   PRIVATE_KEY=0x... bun run script/seed-testnet-liquidity.ts
 */

import { createClient, http, publicActions, walletActions, parseAbi } from 'viem'
import { privateKeyToAccount } from 'viem/accounts'
import { tempoActions, withFeePayer } from 'viem/tempo'
import type { Chain } from 'viem'

// Testnet configuration
const TESTNET_CHAIN_ID = 42431
const TESTNET_RPC_URL = 'https://rpc.moderato.tempo.xyz'
const TESTNET_FEE_SPONSOR_URL = 'https://sponsor.moderato.tempo.xyz'
const BACKEND_URL = process.env.BACKEND_URL || 'http://127.0.0.1:3000'

// Token addresses
const PATH_USD = '0x20C0000000000000000000000000000000000000' as const
const ALPHA_USD = '0x20C0000000000000000000000000000000000001' as const
const STABLECOIN_DEX = '0xDEc0000000000000000000000000000000000000' as const

const tempoModerato = {
  id: TESTNET_CHAIN_ID,
  name: 'Tempo Testnet (Moderato)',
  nativeCurrency: { name: 'USD', symbol: 'USD', decimals: 6 },
  rpcUrls: { 
    default: { 
      http: [TESTNET_RPC_URL],
      webSocket: ['wss://rpc.moderato.tempo.xyz'],
    } 
  },
} as const satisfies Chain

const multiVerseAbi = parseAbi([
  'function split(address asset, uint256 amount, bytes32 marketHash) external',
  'function getVerseAddress(address asset, bytes32 marketHash) external view returns (address yesVerse, address noVerse)',
])

interface Market {
  marketHash: string
  verseTokens: Array<{
    asset: string
    yesVerse: string
    noVerse: string
  }>
}

async function getMarkets(): Promise<Market[]> {
  const res = await fetch(`${BACKEND_URL}/markets`)
  if (!res.ok) {
    throw new Error(`Failed to fetch markets: ${res.statusText}`)
  }
  const data = await res.json() as { markets: Array<{ marketHash: string }> }
  
  const markets: Market[] = []
  for (const m of data.markets || []) {
    const verseRes = await fetch(`${BACKEND_URL}/markets/${m.marketHash}/verse-tokens`)
    if (verseRes.ok) {
      const verseData = await verseRes.json() as { verseTokens: Market['verseTokens'] }
      markets.push({
        marketHash: m.marketHash,
        verseTokens: verseData.verseTokens || [],
      })
    }
  }
  return markets
}

async function main() {
  const privateKey = process.env.PRIVATE_KEY as `0x${string}` | undefined
  if (!privateKey) {
    console.error('Error: PRIVATE_KEY environment variable required')
    process.exit(1)
  }

  const account = privateKeyToAccount(privateKey)
  console.log(`Account: ${account.address}`)
  console.log(`RPC: ${TESTNET_RPC_URL}`)
  console.log(`Backend: ${BACKEND_URL}`)
  console.log('')

  const client = createClient({
    account,
    chain: tempoModerato,
    transport: withFeePayer(
      http(TESTNET_RPC_URL),
      http(TESTNET_FEE_SPONSOR_URL)
    ),
  })
    .extend(publicActions)
    .extend(walletActions)
    .extend(tempoActions())

  // Check balance
  const balance = await client.token.getBalance({
    token: PATH_USD,
    address: account.address,
  })
  console.log(`PathUSD balance: ${Number(balance) / 1e6}`)

  // Step 1: Ensure stablecoin routing pair exists
  console.log('')
  console.log('Step 1: Setting up stablecoin routing...')
  
  try {
    await client.dex.createPair({
      base: ALPHA_USD,
      feeToken: PATH_USD,
    })
    console.log('  Created AlphaUSD/pathUSD pair')
  } catch (e: any) {
    if (e.message?.includes('PairAlreadyExists')) {
      console.log('  AlphaUSD/pathUSD pair already exists')
    } else {
      console.log('  Pair creation error:', e.message?.slice(0, 80))
    }
  }

  // Approve DEX
  const maxApproval = BigInt(2) ** BigInt(128) - 1n
  console.log('  Approving DEX...')
  await client.token.approve({
    token: PATH_USD,
    spender: STABLECOIN_DEX,
    amount: maxApproval,
    feeToken: PATH_USD,
  })
  await client.token.approve({
    token: ALPHA_USD,
    spender: STABLECOIN_DEX,
    amount: maxApproval,
    feeToken: PATH_USD,
  })

  // Place routing liquidity
  const routingAmount = BigInt(1000) * BigInt(1e6) // 1000 tokens
  try {
    console.log('  Placing routing liquidity...')
    await client.dex.placeSync({
      token: ALPHA_USD,
      amount: routingAmount,
      type: 'buy',
      tick: -10,
      feeToken: PATH_USD,
    })
    await client.dex.placeSync({
      token: ALPHA_USD,
      amount: routingAmount,
      type: 'sell',
      tick: 10,
      feeToken: PATH_USD,
    })
    console.log('  ✓ Routing liquidity placed')
  } catch (e: any) {
    console.log('  Routing liquidity error:', e.message?.slice(0, 80))
  }

  // Step 2: Fetch markets from backend
  console.log('')
  console.log('Step 2: Fetching markets...')
  
  let markets: Market[]
  try {
    markets = await getMarkets()
    console.log(`  Found ${markets.length} markets`)
  } catch (e: any) {
    console.error('  Failed to fetch markets:', e.message)
    console.error('  Make sure the backend is running at', BACKEND_URL)
    process.exit(1)
  }

  if (markets.length === 0) {
    console.log('  No markets found. Run the testnet orchestrator first to seed markets.')
    process.exit(0)
  }

  // Step 3: Seed liquidity for each market
  console.log('')
  console.log('Step 3: Seeding market liquidity...')

  // Load testnet config to get multiverse address
  const configPath = './testnet-config.json'
  let multiVerseAddress: `0x${string}`
  try {
    const config = await Bun.file('./.testnet-config.json').json()
    multiVerseAddress = config.multiverse as `0x${string}`
    console.log(`  MultiVerse: ${multiVerseAddress}`)
  } catch (e) {
    console.error('  Failed to load .testnet-config.json')
    process.exit(1)
  }

  const seedAmount = BigInt(500) * BigInt(1e6) // 500 tokens to split
  const orderAmount = BigInt(150) * BigInt(1e6) // 150 per order level

  for (const market of markets) {
    console.log('')
    console.log(`  Market: ${market.marketHash.slice(0, 18)}...`)
    
    for (const vt of market.verseTokens) {
      console.log(`    Asset: ${vt.asset.slice(0, 10)}...`)
      
      // Get verse addresses
      const [yesVerse, noVerse] = await client.readContract({
        address: multiVerseAddress,
        abi: multiVerseAbi,
        functionName: 'getVerseAddress',
        args: [vt.asset as `0x${string}`, market.marketHash as `0x${string}`],
      })

      if (yesVerse === '0x0000000000000000000000000000000000000000') {
        console.log(`    ✗ No verse tokens found`)
        continue
      }

      console.log(`    YES: ${yesVerse}`)
      console.log(`    NO:  ${noVerse}`)

      // Create DEX pairs
      console.log(`    Creating DEX pairs...`)
      for (const [name, verse] of [['YES', yesVerse], ['NO', noVerse]] as const) {
        try {
          await client.dex.createPair({
            base: verse,
            feeToken: PATH_USD,
          })
          console.log(`      Created ${name}/pathUSD pair`)
        } catch (e: any) {
          if (e.message?.includes('PairAlreadyExists')) {
            console.log(`      ${name}/pathUSD pair exists`)
          } else {
            console.log(`      ${name} pair error:`, e.message?.slice(0, 50))
          }
        }
      }

      // Approve MultiVerse to split tokens
      console.log(`    Approving tokens...`)
      await client.token.approve({
        token: vt.asset as `0x${string}`,
        spender: multiVerseAddress,
        amount: seedAmount * 10n,
        feeToken: PATH_USD,
      })

      // Split tokens into YES/NO
      console.log(`    Splitting ${Number(seedAmount) / 1e6} tokens...`)
      try {
        await client.writeContract({
          address: multiVerseAddress,
          abi: multiVerseAbi,
          functionName: 'split',
          args: [vt.asset as `0x${string}`, seedAmount, market.marketHash as `0x${string}`],
          feeToken: PATH_USD,
        })
      } catch (e: any) {
        console.log(`    Split error:`, e.message?.slice(0, 80))
        continue
      }

      // Approve DEX to spend verse tokens
      console.log(`    Approving DEX for verse tokens...`)
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

      // Wait for approvals
      await new Promise(r => setTimeout(r, 1000))

      // Place orders
      console.log(`    Placing orders...`)
      const ticks = [-20, -10, 10, 20]
      
      for (const [name, verse] of [['YES', yesVerse], ['NO', noVerse]] as const) {
        for (const tick of ticks) {
          const type = tick < 0 ? 'buy' : 'sell'
          try {
            await client.dex.placeSync({
              token: verse,
              amount: orderAmount,
              type: type as 'buy' | 'sell',
              tick,
              feeToken: PATH_USD,
            })
            console.log(`      ${name} ${type}@${tick}: ✓`)
          } catch (e: any) {
            console.log(`      ${name} ${type}@${tick}: ✗ ${e.message?.slice(0, 40)}`)
          }
        }
      }

      console.log(`    ✓ Liquidity seeded for ${vt.asset.slice(0, 10)}...`)
    }
  }

  console.log('')
  console.log('Done!')
}

main().catch((err) => {
  console.error('Error:', err.message)
  process.exit(1)
})
