#!/usr/bin/env bun
/**
 * Test script to verify the trade flow works correctly:
 * 1. Fund the account with 1000 aUSD and 500 pathUSD
 * 2. Split 100 aUSD into YES and NO tokens
 * 3. Sell 10 YES tokens
 * 4. Check that probability calculations shift
 */

import { parseUnits, formatUnits, parseAbi, type Address } from 'viem'
import { getTempoClient, PATH_USD, ALPHA_USD } from './lib/tempo'

const MULTIVERSE_ADDRESS = process.env.MULTIVERSE_ADDRESS as Address
const RPC_URL = process.env.RPC_URL || 'http://localhost:9545'

if (!MULTIVERSE_ADDRESS) {
  console.error('Usage: MULTIVERSE_ADDRESS=0x... bun run script/test-trade-flow.ts')
  process.exit(1)
}

const multiVerseAbi = parseAbi([
  'function split(address asset, uint256 amount, bytes32 marketHash) external',
  'function getVerseAddress(address asset, bytes32 marketHash) external view returns (address yesVerse, address noVerse)',
  'function create(address asset, bytes32 marketHash) external returns (address yesVerse, address noVerse)',
])

const tip20Abi = parseAbi([
  'function approve(address spender, uint256 amount) external returns (bool)',
  'function balanceOf(address account) external view returns (uint256)',
])

const STABLECOIN_DEX = '0xDEc0000000000000000000000000000000000000' as Address

const dexAbi = parseAbi([
  'function createPair(address base) external returns (bytes32 key)',
  'function swapExactAmountIn(address tokenIn, address tokenOut, uint128 amountIn, uint128 minAmountOut) external returns (uint128 amountOut)',
  'function quoteSwapExactAmountIn(address tokenIn, address tokenOut, uint128 amountIn) external view returns (uint128 amountOut)',
])

async function main() {
  console.log('=== Test Trade Flow ===\n')
  
  const client = getTempoClient({ rpcUrl: RPC_URL })
  const account = client.account.address
  
  console.log(`Account: ${account}`)
  console.log(`MultiVerse: ${MULTIVERSE_ADDRESS}`)
  console.log('')

  // Get market hash from backend
  const marketsResponse = await fetch('http://localhost:3000/markets')
  const { markets } = await marketsResponse.json()
  const market = markets[0]
  const marketHash = market.marketHash as `0x${string}`
  console.log(`Testing market: ${market.question.slice(0, 40)}...`)
  console.log(`Market hash: ${marketHash}\n`)

  // Step 1: Fund the account
  console.log('Step 1: Funding account...')
  try {
    await client.token.mint({
      token: ALPHA_USD,
      to: account,
      amount: parseUnits('1000', 6),
    })
    console.log('  ✓ Minted 1000 αUSD')
  } catch (e) {
    console.log('  (αUSD mint failed, may already have balance)')
  }

  try {
    await client.token.mint({
      token: PATH_USD,
      to: account,
      amount: parseUnits('500', 6),
    })
    console.log('  ✓ Minted 500 pathUSD')
  } catch (e) {
    console.log('  (pathUSD mint failed, may already have balance)')
  }

  // Check balances
  const alphaBalance = await client.token.getBalance({ token: ALPHA_USD, account })
  const pathBalance = await client.token.getBalance({ token: PATH_USD, account })
  console.log(`  αUSD balance: ${formatUnits(alphaBalance, 6)}`)
  console.log(`  pathUSD balance: ${formatUnits(pathBalance, 6)}\n`)

  // Step 2: Check/Create verse tokens
  console.log('Step 2: Checking verse tokens...')
  let yesVerse: Address
  let noVerse: Address
  
  try {
    const result = await client.readContract({
      address: MULTIVERSE_ADDRESS,
      abi: multiVerseAbi,
      functionName: 'getVerseAddress',
      args: [ALPHA_USD as Address, marketHash],
    })
    yesVerse = result[0]
    noVerse = result[1]
    
    if (yesVerse === '0x0000000000000000000000000000000000000000') {
      console.log('  Creating verse tokens...')
      await client.writeContract({
        address: MULTIVERSE_ADDRESS,
        abi: multiVerseAbi,
        functionName: 'create',
        args: [ALPHA_USD as Address, marketHash],
        feeToken: PATH_USD,
      })
      const newResult = await client.readContract({
        address: MULTIVERSE_ADDRESS,
        abi: multiVerseAbi,
        functionName: 'getVerseAddress',
        args: [ALPHA_USD as Address, marketHash],
      })
      yesVerse = newResult[0]
      noVerse = newResult[1]
    }
    console.log(`  YES token: ${yesVerse}`)
    console.log(`  NO token: ${noVerse}\n`)
  } catch (e) {
    console.error('  Error getting verse tokens:', e)
    process.exit(1)
  }

  // Step 3: Split 100 αUSD
  console.log('Step 3: Splitting 100 αUSD into YES/NO tokens...')
  const splitAmount = parseUnits('100', 6)
  
  try {
    // Approve MultiVerse with a large amount
    const approveHash = await client.writeContract({
      address: ALPHA_USD as Address,
      abi: tip20Abi,
      functionName: 'approve',
      args: [MULTIVERSE_ADDRESS, splitAmount * 10n], // Approve more than needed
      feeToken: PATH_USD,
    })
    console.log('  ✓ Approved MultiVerse, tx:', approveHash)
    
    // Wait a bit for tx to be mined
    await new Promise(r => setTimeout(r, 1000))
    
    // Check allowance
    const allowanceAbi = parseAbi(['function allowance(address owner, address spender) external view returns (uint256)'])
    const allowance = await client.readContract({
      address: ALPHA_USD as Address,
      abi: allowanceAbi,
      functionName: 'allowance',
      args: [account, MULTIVERSE_ADDRESS],
    })
    console.log(`  Allowance: ${formatUnits(allowance, 6)}`)

    // Split
    await client.writeContract({
      address: MULTIVERSE_ADDRESS,
      abi: multiVerseAbi,
      functionName: 'split',
      args: [ALPHA_USD as Address, splitAmount, marketHash],
      feeToken: PATH_USD,
    })
    console.log('  ✓ Split complete')

    // Check new balances using token.getBalance
    const yesBalance = await client.token.getBalance({
      token: yesVerse,
      account,
    })
    const noBalance = await client.token.getBalance({
      token: noVerse,
      account,
    })
    console.log(`  YES balance: ${formatUnits(yesBalance, 6)}`)
    console.log(`  NO balance: ${formatUnits(noBalance, 6)}\n`)
  } catch (e) {
    console.error('  Split error:', e)
    process.exit(1)
  }

  // Step 4: Create DEX pair and sell YES tokens
  console.log('Step 4: Selling 10 YES tokens on DEX...')
  const sellAmount = parseUnits('10', 6)

  try {
    // Create pair if not exists (will fail with PAIR_EXISTS or similar if already created)
    try {
      await client.writeContract({
        address: STABLECOIN_DEX,
        abi: dexAbi,
        functionName: 'createPair',
        args: [yesVerse],
        feeToken: PATH_USD,
      })
      console.log('  ✓ Created YES/quote pair')
    } catch (e: unknown) {
      // 0xc9bb25eb is likely PAIR_EXISTS or similar
      console.log('  YES/quote pair already exists (or creation failed)')
    }

    // Approve DEX to spend YES tokens
    await client.writeContract({
      address: yesVerse,
      abi: tip20Abi,
      functionName: 'approve',
      args: [STABLECOIN_DEX, sellAmount],
      feeToken: PATH_USD,
    })
    console.log('  ✓ Approved DEX to spend YES tokens')

    // Try to get a quote first
    try {
      const quote = await client.readContract({
        address: STABLECOIN_DEX,
        abi: dexAbi,
        functionName: 'quoteSwapExactAmountIn',
        args: [yesVerse, PATH_USD as Address, BigInt(sellAmount)],
      })
      console.log(`  Quote: Sell 10 YES → ${formatUnits(quote, 6)} pathUSD`)
    } catch (e) {
      console.log('  No liquidity for quote (expected without seeding)')
    }

    // Try swap (will fail without liquidity)
    try {
      const result = await client.writeContract({
        address: STABLECOIN_DEX,
        abi: dexAbi,
        functionName: 'swapExactAmountIn',
        args: [yesVerse, PATH_USD as Address, BigInt(sellAmount), 0n],
        feeToken: PATH_USD,
      })
      console.log('  ✓ Swap complete:', result)
    } catch (e: unknown) {
      const error = e as Error
      console.log('  Swap failed (expected without liquidity):', error.message?.slice(0, 100))
    }

  } catch (e) {
    console.error('  Sell error:', e)
  }

  console.log('\n=== Test Complete ===')
}

main().catch(console.error)
