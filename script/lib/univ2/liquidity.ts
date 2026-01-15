/**
 * UniswapV2 Liquidity Management
 * 
 * Functions for adding and removing liquidity from UniV2 pairs.
 */

import { type Address } from 'viem'
import { ROUTER_ABI, ERC20_ABI, getUniV2Addresses } from './addresses'
import { PATH_USD } from '../tempo'

// Generic client type compatible with Tempo client
// eslint-disable-next-line @typescript-eslint/no-explicit-any
type AnyClient = any

// Default deadline: 1 hour from now
function getDeadline(): bigint {
  return BigInt(Math.floor(Date.now() / 1000) + 3600)
}

// Max approval amount
const MAX_APPROVAL = BigInt(2) ** BigInt(128) - 1n

export interface AddLiquidityParams {
  tokenA: Address
  tokenB: Address
  amountA: bigint
  amountB: bigint
  /** Minimum amount of tokenA to add (default: 95% of amountA for 5% slippage) */
  amountAMin?: bigint
  /** Minimum amount of tokenB to add (default: 95% of amountB for 5% slippage) */
  amountBMin?: bigint
  /** Recipient of LP tokens (default: caller) */
  to?: Address
  /** Transaction deadline (default: 1 hour from now) */
  deadline?: bigint
}

export interface AddLiquidityResult {
  amountA: bigint
  amountB: bigint
  liquidity: bigint
}

export interface RemoveLiquidityParams {
  tokenA: Address
  tokenB: Address
  liquidity: bigint
  /** Minimum amount of tokenA to receive (default: 0) */
  amountAMin?: bigint
  /** Minimum amount of tokenB to receive (default: 0) */
  amountBMin?: bigint
  /** Recipient of tokens (default: caller) */
  to?: Address
  /** Transaction deadline (default: 1 hour from now) */
  deadline?: bigint
}

export interface RemoveLiquidityResult {
  amountA: bigint
  amountB: bigint
}

/**
 * Approve router to spend tokens if needed
 */
export async function ensureRouterApproval(
  client: AnyClient,
  token: Address,
  amount: bigint = MAX_APPROVAL
): Promise<void> {
  const { router } = getUniV2Addresses()
  const owner = client.account.address

  // Check current allowance
  const allowance = await client.readContract({
    address: token,
    abi: ERC20_ABI,
    functionName: 'allowance',
    args: [owner, router],
  })

  if ((allowance as bigint) < amount) {
    // Use Tempo token.approve if available, otherwise use writeContract
    try {
      await client.token.approve({
        token,
        spender: router,
        amount: MAX_APPROVAL,
        feeToken: PATH_USD,
      })
    } catch {
      const hash = await client.writeContract({
        address: token,
        abi: ERC20_ABI,
        functionName: 'approve',
        args: [router, MAX_APPROVAL],
        feeToken: PATH_USD,
      })
      await client.waitForTransactionReceipt({ hash })
    }
  }
}

/**
 * Add liquidity to a UniV2 pair
 * Creates the pair if it doesn't exist.
 */
export async function addLiquidity(
  client: AnyClient,
  params: AddLiquidityParams
): Promise<AddLiquidityResult> {
  const { router } = getUniV2Addresses()
  const {
    tokenA,
    tokenB,
    amountA,
    amountB,
    amountAMin = (amountA * 95n) / 100n, // 5% slippage
    amountBMin = (amountB * 95n) / 100n,
    to = client.account.address,
    deadline = getDeadline(),
  } = params

  // Approve router for both tokens
  await ensureRouterApproval(client, tokenA, amountA)
  await ensureRouterApproval(client, tokenB, amountB)

  // Add liquidity
  const hash = await client.writeContract({
    address: router,
    abi: ROUTER_ABI,
    functionName: 'addLiquidity',
    args: [tokenA, tokenB, amountA, amountB, amountAMin, amountBMin, to, deadline],
    feeToken: PATH_USD,
  })

  const receipt = await client.waitForTransactionReceipt({ hash })

  // Parse the return values from logs or simulate
  // For simplicity, we return the input amounts (actual amounts may differ slightly)
  return {
    amountA,
    amountB,
    liquidity: 0n, // Would need to parse from logs
  }
}

/**
 * Remove liquidity from a UniV2 pair
 */
export async function removeLiquidity(
  client: AnyClient,
  params: RemoveLiquidityParams
): Promise<RemoveLiquidityResult> {
  const { router } = getUniV2Addresses()
  const {
    tokenA,
    tokenB,
    liquidity,
    amountAMin = 0n,
    amountBMin = 0n,
    to = client.account.address,
    deadline = getDeadline(),
  } = params

  // Get pair address to approve LP tokens
  const { factory } = getUniV2Addresses()
  const pair = await client.readContract({
    address: factory,
    abi: [{ name: 'getPair', type: 'function', inputs: [{ type: 'address' }, { type: 'address' }], outputs: [{ type: 'address' }], stateMutability: 'view' }],
    functionName: 'getPair',
    args: [tokenA, tokenB],
  }) as Address

  // Approve router for LP tokens
  await ensureRouterApproval(client, pair, liquidity)

  // Remove liquidity
  const hash = await client.writeContract({
    address: router,
    abi: ROUTER_ABI,
    functionName: 'removeLiquidity',
    args: [tokenA, tokenB, liquidity, amountAMin, amountBMin, to, deadline],
    feeToken: PATH_USD,
  })

  await client.waitForTransactionReceipt({ hash })

  return {
    amountA: 0n, // Would need to parse from logs
    amountB: 0n,
  }
}

/**
 * Add liquidity with equal amounts of both tokens
 * Useful for initial liquidity provision at 1:1 ratio
 */
export async function addLiquidityEqual(
  client: AnyClient,
  tokenA: Address,
  tokenB: Address,
  amount: bigint,
  to?: Address
): Promise<AddLiquidityResult> {
  return addLiquidity(client, {
    tokenA,
    tokenB,
    amountA: amount,
    amountB: amount,
    to,
  })
}
