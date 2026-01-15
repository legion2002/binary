/**
 * UniswapV2 Swap Functions
 * 
 * Functions for swapping tokens and getting quotes.
 */

import { type Address } from 'viem'
import { ROUTER_ABI, getUniV2Addresses } from './addresses'
import { ensureRouterApproval } from './liquidity'
import { PATH_USD } from '../tempo'

// Generic client type compatible with Tempo client
// eslint-disable-next-line @typescript-eslint/no-explicit-any
type AnyClient = any

// Default deadline: 1 hour from now
function getDeadline(): bigint {
  return BigInt(Math.floor(Date.now() / 1000) + 3600)
}

export interface SwapExactTokensParams {
  /** Amount of input token to swap */
  amountIn: bigint
  /** Minimum amount of output token to receive (slippage protection) */
  amountOutMin: bigint
  /** Path of tokens to swap through (e.g., [tokenIn, tokenOut] or [tokenIn, PATH_USD, tokenOut]) */
  path: Address[]
  /** Recipient of output tokens (default: caller) */
  to?: Address
  /** Transaction deadline (default: 1 hour from now) */
  deadline?: bigint
}

export interface SwapTokensForExactParams {
  /** Exact amount of output token to receive */
  amountOut: bigint
  /** Maximum amount of input token to spend (slippage protection) */
  amountInMax: bigint
  /** Path of tokens to swap through */
  path: Address[]
  /** Recipient of output tokens (default: caller) */
  to?: Address
  /** Transaction deadline (default: 1 hour from now) */
  deadline?: bigint
}

/**
 * Swap exact amount of input tokens for output tokens
 */
export async function swapExactTokensForTokens(
  client: AnyClient,
  params: SwapExactTokensParams
): Promise<bigint[]> {
  const { router } = getUniV2Addresses()
  const {
    amountIn,
    amountOutMin,
    path,
    to = client.account.address,
    deadline = getDeadline(),
  } = params

  if (path.length < 2) {
    throw new Error('Path must have at least 2 tokens')
  }

  // Approve router for input token
  await ensureRouterApproval(client, path[0], amountIn)

  // Execute swap
  const hash = await client.writeContract({
    address: router,
    abi: ROUTER_ABI,
    functionName: 'swapExactTokensForTokens',
    args: [amountIn, amountOutMin, path, to, deadline],
    feeToken: PATH_USD,
  })

  await client.waitForTransactionReceipt({ hash })

  // Return the expected amounts (actual amounts are in logs)
  return [amountIn, amountOutMin]
}

/**
 * Swap tokens for exact amount of output tokens
 */
export async function swapTokensForExactTokens(
  client: AnyClient,
  params: SwapTokensForExactParams
): Promise<bigint[]> {
  const { router } = getUniV2Addresses()
  const {
    amountOut,
    amountInMax,
    path,
    to = client.account.address,
    deadline = getDeadline(),
  } = params

  if (path.length < 2) {
    throw new Error('Path must have at least 2 tokens')
  }

  // Approve router for input token
  await ensureRouterApproval(client, path[0], amountInMax)

  // Execute swap
  const hash = await client.writeContract({
    address: router,
    abi: ROUTER_ABI,
    functionName: 'swapTokensForExactTokens',
    args: [amountOut, amountInMax, path, to, deadline],
    feeToken: PATH_USD,
  })

  await client.waitForTransactionReceipt({ hash })

  return [amountInMax, amountOut]
}

/**
 * Get output amounts for a given input amount and path
 */
export async function getAmountsOut(
  client: AnyClient,
  amountIn: bigint,
  path: Address[]
): Promise<bigint[]> {
  const { router } = getUniV2Addresses()

  if (path.length < 2) {
    throw new Error('Path must have at least 2 tokens')
  }

  const amounts = await client.readContract({
    address: router,
    abi: ROUTER_ABI,
    functionName: 'getAmountsOut',
    args: [amountIn, path],
  })

  return amounts as bigint[]
}

/**
 * Get input amounts required for a given output amount and path
 */
export async function getAmountsIn(
  client: AnyClient,
  amountOut: bigint,
  path: Address[]
): Promise<bigint[]> {
  const { router } = getUniV2Addresses()

  if (path.length < 2) {
    throw new Error('Path must have at least 2 tokens')
  }

  const amounts = await client.readContract({
    address: router,
    abi: ROUTER_ABI,
    functionName: 'getAmountsIn',
    args: [amountOut, path],
  })

  return amounts as bigint[]
}

/**
 * Get a quote for how much of tokenB you get for amountA of tokenA
 * This is a simple constant-product calculation without fees.
 */
export async function quote(
  client: AnyClient,
  amountA: bigint,
  reserveA: bigint,
  reserveB: bigint
): Promise<bigint> {
  const { router } = getUniV2Addresses()

  return client.readContract({
    address: router,
    abi: ROUTER_ABI,
    functionName: 'quote',
    args: [amountA, reserveA, reserveB],
  })
}

/**
 * Calculate the output amount for a swap (includes 0.3% fee)
 */
export async function getAmountOut(
  client: AnyClient,
  amountIn: bigint,
  reserveIn: bigint,
  reserveOut: bigint
): Promise<bigint> {
  const { router } = getUniV2Addresses()

  return client.readContract({
    address: router,
    abi: ROUTER_ABI,
    functionName: 'getAmountOut',
    args: [amountIn, reserveIn, reserveOut],
  })
}

/**
 * Calculate the input amount required for a desired output (includes 0.3% fee)
 */
export async function getAmountIn(
  client: AnyClient,
  amountOut: bigint,
  reserveIn: bigint,
  reserveOut: bigint
): Promise<bigint> {
  const { router } = getUniV2Addresses()

  return client.readContract({
    address: router,
    abi: ROUTER_ABI,
    functionName: 'getAmountIn',
    args: [amountOut, reserveIn, reserveOut],
  })
}

/**
 * Simple swap helper: swap tokenIn for tokenOut with automatic path
 * Uses PATH_USD as intermediate if direct pair doesn't exist
 */
export async function simpleSwap(
  client: AnyClient,
  tokenIn: Address,
  tokenOut: Address,
  amountIn: bigint,
  slippageBps: number = 50 // 0.5% default slippage
): Promise<bigint> {
  // Try direct path first
  let path: Address[] = [tokenIn, tokenOut]
  
  try {
    const amounts = await getAmountsOut(client, amountIn, path)
    const amountOutMin = (amounts[1] * BigInt(10000 - slippageBps)) / 10000n
    
    await swapExactTokensForTokens(client, {
      amountIn,
      amountOutMin,
      path,
    })
    
    return amounts[1]
  } catch {
    // Try routing through PATH_USD
    if (tokenIn !== PATH_USD && tokenOut !== PATH_USD) {
      path = [tokenIn, PATH_USD as Address, tokenOut]
      
      const amounts = await getAmountsOut(client, amountIn, path)
      const amountOutMin = (amounts[2] * BigInt(10000 - slippageBps)) / 10000n
      
      await swapExactTokensForTokens(client, {
        amountIn,
        amountOutMin,
        path,
      })
      
      return amounts[2]
    }
    throw new Error(`No swap path found from ${tokenIn} to ${tokenOut}`)
  }
}
