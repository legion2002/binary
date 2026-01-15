#!/usr/bin/env bun
/**
 * Check pool reserves and add 10x more liquidity on Moderato testnet
 */

import { createClient, http, publicActions, walletActions, parseAbi, formatUnits, getAddress } from 'viem'
import { privateKeyToAccount } from 'viem/accounts'
import { tempoActions, withFeePayer } from 'viem/tempo'
import type { Address, Chain } from 'viem'
import deployments from '../deployments.json'

const sleep = (ms: number) => new Promise(r => setTimeout(r, ms))
const DELAY = 2000 // 2 second delay between RPC calls

// Testnet config
const CHAIN_ID = 42431
const RPC_URL = 'https://rpc.moderato.tempo.xyz'
const RPC_AUTH = 'Basic ' + Buffer.from('eng:zealous-mayer').toString('base64')
const FEE_SPONSOR_URL = 'https://sponsor.moderato.tempo.xyz'
const BACKEND_URL = process.env.BACKEND_URL || 'https://binary-markets-api.fly.dev'

// Load deployment addresses for this chain
const deployment = deployments[String(CHAIN_ID) as keyof typeof deployments]
if (!deployment) {
  throw new Error(`No deployment found for chain ${CHAIN_ID}`)
}

// Token addresses
const PATH_USD = '0x20C0000000000000000000000000000000000000' as Address
const ALPHA_USD = '0x20C0000000000000000000000000000000000001' as Address

const tempoModerato = {
  id: CHAIN_ID,
  name: deployment.name,
  nativeCurrency: { name: 'USD', symbol: 'USD', decimals: 6 },
  rpcUrls: { 
    default: { http: [RPC_URL] } 
  },
} as const satisfies Chain

// UniV2 ABIs
const factoryAbi = parseAbi([
  'function getPair(address tokenA, address tokenB) external view returns (address pair)',
])

const pairAbi = parseAbi([
  'function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)',
  'function token0() external view returns (address)',
  'function token1() external view returns (address)',
])

const routerAbi = parseAbi([
  'function addLiquidity(address tokenA, address tokenB, uint amountADesired, uint amountBDesired, uint amountAMin, uint amountBMin, address to, uint deadline) external returns (uint amountA, uint amountB, uint liquidity)',
])

const erc20Abi = parseAbi([
  'function approve(address spender, uint256 amount) external returns (bool)',
  'function balanceOf(address account) external view returns (uint256)',
])

const multiVerseAbi = parseAbi([
  'function split(address asset, uint256 amount, bytes32 marketHash) external',
  'function getVerseAddress(address asset, bytes32 marketHash) external view returns (address yesVerse, address noVerse)',
])

// Contract addresses from deployments.json
const UNIV2_FACTORY = getAddress(deployment.uniV2Factory)
const UNIV2_ROUTER = getAddress(deployment.uniV2Router)
const MULTIVERSE = getAddress(deployment.multiverse)

async function main() {
  const privateKey = process.env.PRIVATE_KEY as `0x${string}` | undefined
  if (!privateKey) {
    console.error('Error: PRIVATE_KEY environment variable required')
    process.exit(1)
  }

  const account = privateKeyToAccount(privateKey)
  console.log(`Account: ${account.address}`)
  console.log(`RPC: ${RPC_URL}`)
  // No rate limit cooldown needed with authenticated RPC
  console.log('')

  const client = createClient({
    account,
    chain: tempoModerato,
    transport: withFeePayer(
      http(RPC_URL, {
        fetchOptions: {
          headers: {
            'Authorization': RPC_AUTH,
          },
        },
      }),
      http(FEE_SPONSOR_URL)
    ),
  })
    .extend(publicActions)
    .extend(walletActions)
    .extend(tempoActions())

  // Check balances
  await sleep(DELAY)
  const pathBalance = await client.readContract({
    address: PATH_USD,
    abi: erc20Abi,
    functionName: 'balanceOf',
    args: [account.address],
  })
  await sleep(DELAY)
  const alphaBalance = await client.readContract({
    address: ALPHA_USD,
    abi: erc20Abi,
    functionName: 'balanceOf',
    args: [account.address],
  })
  console.log(`PATH_USD balance: ${formatUnits(pathBalance, 6)}`)
  console.log(`ALPHA_USD balance: ${formatUnits(alphaBalance, 6)}`)
  console.log('')

  // Check AlphaUSD/PathUSD pair reserves
  console.log('=== Checking AlphaUSD/PathUSD Routing Pair ===')
  await sleep(DELAY)
  const routingPair = await client.readContract({
    address: UNIV2_FACTORY,
    abi: factoryAbi,
    functionName: 'getPair',
    args: [ALPHA_USD, PATH_USD],
  })
  
  let needsRoutingLiquidity = false
  if (routingPair === '0x0000000000000000000000000000000000000000') {
    console.log('  Routing pair does not exist - will create with liquidity')
    needsRoutingLiquidity = true
  } else {
    console.log(`  Pair address: ${routingPair}`)
    await sleep(DELAY)
    const reserves = await client.readContract({
      address: routingPair,
      abi: pairAbi,
      functionName: 'getReserves',
    })
    await sleep(DELAY)
    const token0 = await client.readContract({
      address: routingPair,
      abi: pairAbi,
      functionName: 'token0',
    })
    
    const isToken0Alpha = token0.toLowerCase() === ALPHA_USD.toLowerCase()
    const alphaReserve = isToken0Alpha ? reserves[0] : reserves[1]
    const pathReserve = isToken0Alpha ? reserves[1] : reserves[0]
    
    console.log(`  ALPHA_USD reserve: ${formatUnits(alphaReserve, 6)}`)
    console.log(`  PATH_USD reserve: ${formatUnits(pathReserve, 6)}`)
  }
  console.log('')

  // Get first market from backend
  console.log('=== Fetching First Market ===')
  let marketHash: string
  let yesVerse: Address
  let noVerse: Address
  
  try {
    const res = await fetch(`${BACKEND_URL}/markets`)
    const data = await res.json() as { markets: Array<{ marketHash: string }> }
    if (!data.markets || data.markets.length === 0) {
      console.log('No markets found')
      process.exit(0)
    }
    marketHash = data.markets[0].marketHash
    console.log(`  Market hash: ${marketHash}`)
    
    // Get verse tokens for ALPHA_USD (we split ALPHA_USD, not PATH_USD)
    const verseRes = await fetch(`${BACKEND_URL}/markets/${marketHash}/verse-tokens`)
    const verseData = await verseRes.json() as Array<{ asset: string; yesVerse: string; noVerse: string }>
    if (!verseData || verseData.length === 0) {
      console.log('No verse tokens found')
      process.exit(0)
    }
    // Find the ALPHA_USD verse tokens (not PATH_USD)
    const alphaVerse = verseData.find(v => v.asset.toLowerCase() === ALPHA_USD.toLowerCase())
    if (!alphaVerse) {
      console.log('No ALPHA_USD verse tokens found')
      process.exit(0)
    }
    yesVerse = alphaVerse.yesVerse as Address
    noVerse = alphaVerse.noVerse as Address
    console.log(`  YES token (ALPHA_USD): ${yesVerse}`)
    console.log(`  NO token (ALPHA_USD): ${noVerse}`)
  } catch (e: any) {
    console.error('Failed to fetch market:', e.message)
    process.exit(1)
  }
  console.log('')

  // Check YES/PathUSD pair reserves
  console.log('=== Checking YES/PathUSD Pair ===')
  await sleep(DELAY)
  const yesPair = await client.readContract({
    address: UNIV2_FACTORY,
    abi: factoryAbi,
    functionName: 'getPair',
    args: [yesVerse, PATH_USD],
  })
  
  if (yesPair === '0x0000000000000000000000000000000000000000') {
    console.log('  YES/PathUSD pair does not exist!')
  } else {
    console.log(`  Pair address: ${yesPair}`)
    await sleep(DELAY)
    const reserves = await client.readContract({
      address: yesPair,
      abi: pairAbi,
      functionName: 'getReserves',
    })
    await sleep(DELAY)
    const token0 = await client.readContract({
      address: yesPair,
      abi: pairAbi,
      functionName: 'token0',
    })
    
    const isToken0Yes = token0.toLowerCase() === yesVerse.toLowerCase()
    const yesReserve = isToken0Yes ? reserves[0] : reserves[1]
    const pathReserve = isToken0Yes ? reserves[1] : reserves[0]
    
    console.log(`  YES reserve: ${formatUnits(yesReserve, 6)}`)
    console.log(`  PATH_USD reserve: ${formatUnits(pathReserve, 6)}`)
  }
  console.log('')

  // Check NO/PathUSD pair reserves  
  console.log('=== Checking NO/PathUSD Pair ===')
  await sleep(DELAY)
  const noPair = await client.readContract({
    address: UNIV2_FACTORY,
    abi: factoryAbi,
    functionName: 'getPair',
    args: [noVerse, PATH_USD],
  })
  
  if (noPair === '0x0000000000000000000000000000000000000000') {
    console.log('  NO/PathUSD pair does not exist!')
  } else {
    console.log(`  Pair address: ${noPair}`)
    await sleep(DELAY)
    const reserves = await client.readContract({
      address: noPair,
      abi: pairAbi,
      functionName: 'getReserves',
    })
    await sleep(DELAY)
    const token0 = await client.readContract({
      address: noPair,
      abi: pairAbi,
      functionName: 'token0',
    })
    
    const isToken0No = token0.toLowerCase() === noVerse.toLowerCase()
    const noReserve = isToken0No ? reserves[0] : reserves[1]
    const pathReserve = isToken0No ? reserves[1] : reserves[0]
    
    console.log(`  NO reserve: ${formatUnits(noReserve, 6)}`)
    console.log(`  PATH_USD reserve: ${formatUnits(pathReserve, 6)}`)
  }
  console.log('')

  // Add 10x more liquidity
  const addLiquidity = process.argv.includes('--add')
  if (!addLiquidity) {
    console.log('To add 10x more liquidity, run with --add flag')
    process.exit(0)
  }

  console.log('=== Adding Liquidity ===')
  
  // Amount to add
  const liquidityAmount = BigInt(50000) * BigInt(1e6)
  const routingLiquidityAmount = BigInt(20000) * BigInt(1e6)
  
  // Add AlphaUSD/PathUSD routing liquidity if needed
  if (needsRoutingLiquidity) {
    console.log(`Adding ${formatUnits(routingLiquidityAmount, 6)} ALPHA_USD + PATH_USD routing liquidity...`)
    
    // Approve router for ALPHA_USD
    const alphaApproveHash = await client.writeContract({
      address: ALPHA_USD,
      abi: erc20Abi,
      functionName: 'approve',
      args: [UNIV2_ROUTER, routingLiquidityAmount],
    })
    await client.waitForTransactionReceipt({ hash: alphaApproveHash })
    console.log('  ✓ ALPHA_USD approved')
    
    // Approve router for PATH_USD
    const pathApproveHash = await client.writeContract({
      address: PATH_USD,
      abi: erc20Abi,
      functionName: 'approve',
      args: [UNIV2_ROUTER, routingLiquidityAmount],
    })
    await client.waitForTransactionReceipt({ hash: pathApproveHash })
    console.log('  ✓ PATH_USD approved')
    
    const deadline = BigInt(Math.floor(Date.now() / 1000) + 1800)
    const addRoutingHash = await client.writeContract({
      address: UNIV2_ROUTER,
      abi: routerAbi,
      functionName: 'addLiquidity',
      args: [
        ALPHA_USD,
        PATH_USD,
        routingLiquidityAmount,
        routingLiquidityAmount,
        routingLiquidityAmount * 95n / 100n,
        routingLiquidityAmount * 95n / 100n,
        account.address,
        deadline,
      ],
    })
    await client.waitForTransactionReceipt({ hash: addRoutingHash })
    console.log(`  ✓ Added ${formatUnits(routingLiquidityAmount, 6)} ALPHA_USD + ${formatUnits(routingLiquidityAmount, 6)} PATH_USD routing`)
  }
  
  // Split tokens to get YES/NO
  console.log(`Splitting ${formatUnits(liquidityAmount, 6)} ALPHA_USD into YES/NO...`)
  
  // Approve MultiVerse
  const approveHash = await client.writeContract({
    address: ALPHA_USD,
    abi: erc20Abi,
    functionName: 'approve',
    args: [MULTIVERSE, liquidityAmount],
  })
  await client.waitForTransactionReceipt({ hash: approveHash })
  console.log('  ✓ Approved MultiVerse')
  
  // Split
  const splitHash = await client.writeContract({
    address: MULTIVERSE,
    abi: multiVerseAbi,
    functionName: 'split',
    args: [ALPHA_USD, liquidityAmount, marketHash as `0x${string}`],
  })
  await client.waitForTransactionReceipt({ hash: splitHash })
  console.log('  ✓ Split complete')

  // Add liquidity to YES/PathUSD
  console.log('Adding liquidity to YES/PathUSD...')
  const halfAmount = liquidityAmount / 2n
  
  // Approve router for YES
  const yesApproveHash = await client.writeContract({
    address: yesVerse,
    abi: erc20Abi,
    functionName: 'approve',
    args: [UNIV2_ROUTER, halfAmount],
  })
  await client.waitForTransactionReceipt({ hash: yesApproveHash })
  console.log('  ✓ YES approved')
  
  // Approve router for PATH_USD
  const pathApproveHash = await client.writeContract({
    address: PATH_USD,
    abi: erc20Abi,
    functionName: 'approve',
    args: [UNIV2_ROUTER, halfAmount],
  })
  await client.waitForTransactionReceipt({ hash: pathApproveHash })
  console.log('  ✓ PATH_USD approved')
  
  const deadline = BigInt(Math.floor(Date.now() / 1000) + 1800)
  const addYesHash = await client.writeContract({
    address: UNIV2_ROUTER,
    abi: routerAbi,
    functionName: 'addLiquidity',
    args: [
      yesVerse,
      PATH_USD,
      halfAmount,
      halfAmount,
      halfAmount * 95n / 100n,
      halfAmount * 95n / 100n,
      account.address,
      deadline,
    ],
  })
  await client.waitForTransactionReceipt({ hash: addYesHash })
  console.log(`  ✓ Added ${formatUnits(halfAmount, 6)} YES + ${formatUnits(halfAmount, 6)} PATH_USD`)

  // Add liquidity to NO/PathUSD
  console.log('Adding liquidity to NO/PathUSD...')
  
  // Approve router for NO
  const noApproveHash = await client.writeContract({
    address: noVerse,
    abi: erc20Abi,
    functionName: 'approve',
    args: [UNIV2_ROUTER, halfAmount],
  })
  await client.waitForTransactionReceipt({ hash: noApproveHash })
  console.log('  ✓ NO approved')
  
  // Approve router for PATH_USD
  const pathApprove2Hash = await client.writeContract({
    address: PATH_USD,
    abi: erc20Abi,
    functionName: 'approve',
    args: [UNIV2_ROUTER, halfAmount],
  })
  await client.waitForTransactionReceipt({ hash: pathApprove2Hash })
  console.log('  ✓ PATH_USD approved')
  
  const addNoHash = await client.writeContract({
    address: UNIV2_ROUTER,
    abi: routerAbi,
    functionName: 'addLiquidity',
    args: [
      noVerse,
      PATH_USD,
      halfAmount,
      halfAmount,
      halfAmount * 95n / 100n,
      halfAmount * 95n / 100n,
      account.address,
      deadline,
    ],
  })
  await client.waitForTransactionReceipt({ hash: addNoHash })
  console.log(`  ✓ Added ${formatUnits(halfAmount, 6)} NO + ${formatUnits(halfAmount, 6)} PATH_USD`)

  console.log('')
  console.log('✓ Liquidity added successfully!')
}

main().catch(console.error)
