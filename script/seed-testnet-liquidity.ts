#!/usr/bin/env bun
/**
 * Seed liquidity for markets on testnet using UniswapV2
 * 
 * This script creates UniV2 pairs and adds liquidity for verse tokens on testnet.
 * 
 * Usage:
 *   PRIVATE_KEY=0x... bun run script/seed-testnet-liquidity.ts
 */

import { createClient, http, publicActions, walletActions, parseAbi } from 'viem'
import { privateKeyToAccount } from 'viem/accounts'
import { tempoActions, withFeePayer } from 'viem/tempo'
import type { Address, Chain } from 'viem'

// Testnet configuration
const TESTNET_CHAIN_ID = 42431
const TESTNET_RPC_URL = 'https://rpc.moderato.tempo.xyz'
const TESTNET_FEE_SPONSOR_URL = 'https://sponsor.moderato.tempo.xyz'
const BACKEND_URL = process.env.BACKEND_URL || 'http://127.0.0.1:3000'

// Token addresses
const PATH_USD = '0x20C0000000000000000000000000000000000000' as const
const ALPHA_USD = '0x20C0000000000000000000000000000000000001' as const

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
      const verseData = await verseRes.json() as Array<{ asset: string; yesVerse: string; noVerse: string }>
      markets.push({
        marketHash: m.marketHash,
        verseTokens: verseData || [],
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

  // Load UniV2 and testnet config
  console.log('')
  console.log('Loading configuration...')
  
  let multiVerseAddress: Address
  let uniV2Router: Address
  let uniV2Factory: Address
  
  try {
    const config = await Bun.file('./.testnet-config.json').json()
    multiVerseAddress = config.multiverse as Address
    console.log(`  MultiVerse: ${multiVerseAddress}`)
  } catch (e) {
    console.error('  Failed to load .testnet-config.json')
    process.exit(1)
  }

  try {
    const uniV2Config = await Bun.file('./.univ2-config.json').json()
    uniV2Router = uniV2Config.router as Address
    uniV2Factory = uniV2Config.factory as Address
    console.log(`  UniV2 Router: ${uniV2Router}`)
    console.log(`  UniV2 Factory: ${uniV2Factory}`)
  } catch (e) {
    console.error('  Failed to load .univ2-config.json')
    console.error('  Run the testnet orchestrator first to deploy UniV2')
    process.exit(1)
  }

  // Import UniV2 functions
  const { createPair, addLiquidity, pairExists } = await import('./lib/univ2')

  // Step 1: Ensure stablecoin routing pair exists
  console.log('')
  console.log('Step 1: Setting up stablecoin routing...')
  
  const routingAmount = BigInt(1000) * BigInt(1e6) // 1000 tokens

  // Check if AlphaUSD/PathUSD pair exists
  const routingPairExists = await pairExists(client, ALPHA_USD as Address, PATH_USD as Address)
  if (!routingPairExists) {
    console.log('  Creating AlphaUSD/PathUSD pair...')
    await createPair(client, ALPHA_USD as Address, PATH_USD as Address)
  } else {
    console.log('  AlphaUSD/PathUSD pair already exists')
  }

  // Add routing liquidity
  try {
    console.log('  Adding routing liquidity...')
    await addLiquidity(client, {
      tokenA: ALPHA_USD as Address,
      tokenB: PATH_USD as Address,
      amountA: routingAmount,
      amountB: routingAmount,
    })
    console.log('  ✓ Routing liquidity added')
  } catch (e: any) {
    console.log('  Routing liquidity:', e.message?.slice(0, 80))
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
  console.log('Step 3: Seeding market liquidity with UniV2...')

  const seedAmount = BigInt(500) * BigInt(1e6) // 500 tokens to split
  const liquidityPerPair = seedAmount / 2n // 250 per pair (YES and NO)

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
        args: [vt.asset as Address, market.marketHash as `0x${string}`],
      })

      if (yesVerse === '0x0000000000000000000000000000000000000000') {
        console.log(`    ✗ No verse tokens found`)
        continue
      }

      console.log(`    YES: ${yesVerse}`)
      console.log(`    NO:  ${noVerse}`)

      // Create UniV2 pairs
      console.log(`    Creating UniV2 pairs...`)
      
      const yesPairExists = await pairExists(client, yesVerse as Address, PATH_USD as Address)
      if (!yesPairExists) {
        await createPair(client, yesVerse as Address, PATH_USD as Address)
        console.log(`      Created YES/PathUSD pair`)
      } else {
        console.log(`      YES/PathUSD pair exists`)
      }

      const noPairExists = await pairExists(client, noVerse as Address, PATH_USD as Address)
      if (!noPairExists) {
        await createPair(client, noVerse as Address, PATH_USD as Address)
        console.log(`      Created NO/PathUSD pair`)
      } else {
        console.log(`      NO/PathUSD pair exists`)
      }

      // Approve MultiVerse to split tokens
      console.log(`    Approving tokens...`)
      await client.token.approve({
        token: vt.asset as Address,
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
          args: [vt.asset as Address, seedAmount, market.marketHash as `0x${string}`],
          feeToken: PATH_USD,
        })
      } catch (e: any) {
        console.log(`    Split error:`, e.message?.slice(0, 80))
        continue
      }

      // Wait for split to confirm
      await new Promise(r => setTimeout(r, 1000))

      // Add liquidity to YES/PathUSD pair (1:1 ratio for 50/50 probability)
      console.log(`    Adding liquidity to YES/PathUSD...`)
      try {
        await addLiquidity(client, {
          tokenA: yesVerse as Address,
          tokenB: PATH_USD as Address,
          amountA: liquidityPerPair,
          amountB: liquidityPerPair,
        })
        console.log(`      ✓ Added ${Number(liquidityPerPair) / 1e6} YES + ${Number(liquidityPerPair) / 1e6} PathUSD`)
      } catch (e: any) {
        console.log(`      ✗ ${e.message?.slice(0, 60)}`)
      }

      // Add liquidity to NO/PathUSD pair
      console.log(`    Adding liquidity to NO/PathUSD...`)
      try {
        await addLiquidity(client, {
          tokenA: noVerse as Address,
          tokenB: PATH_USD as Address,
          amountA: liquidityPerPair,
          amountB: liquidityPerPair,
        })
        console.log(`      ✓ Added ${Number(liquidityPerPair) / 1e6} NO + ${Number(liquidityPerPair) / 1e6} PathUSD`)
      } catch (e: any) {
        console.log(`      ✗ ${e.message?.slice(0, 60)}`)
      }

      console.log(`    ✓ Liquidity added for ${vt.asset.slice(0, 10)}...`)
    }
  }

  console.log('')
  console.log('Done!')
}

main().catch((err) => {
  console.error('Error:', err.message)
  process.exit(1)
})
