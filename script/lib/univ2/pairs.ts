/**
 * UniswapV2 Pair Management
 * 
 * Functions for creating and querying UniV2 pairs.
 */

import { type Address } from 'viem'
import { FACTORY_ABI, PAIR_ABI, getUniV2Addresses } from './addresses'
import { PATH_USD } from '../tempo'

// Generic client type compatible with Tempo client
// eslint-disable-next-line @typescript-eslint/no-explicit-any
type AnyClient = any

const ZERO_ADDRESS = '0x0000000000000000000000000000000000000000' as Address

export interface PairReserves {
  reserve0: bigint
  reserve1: bigint
  blockTimestampLast: number
  token0: Address
  token1: Address
}

/**
 * Create a new UniV2 trading pair
 * @returns The pair address
 */
export async function createPair(
  client: AnyClient,
  tokenA: Address,
  tokenB: Address
): Promise<Address> {
  const { factory } = getUniV2Addresses()

  // Check if pair already exists
  const existingPair = await client.readContract({
    address: factory,
    abi: FACTORY_ABI,
    functionName: 'getPair',
    args: [tokenA, tokenB],
  })

  if (existingPair !== ZERO_ADDRESS) {
    return existingPair as Address
  }

  // Create the pair
  const hash = await client.writeContract({
    address: factory,
    abi: FACTORY_ABI,
    functionName: 'createPair',
    args: [tokenA, tokenB],
    feeToken: PATH_USD,
  })

  // Wait for confirmation
  await client.waitForTransactionReceipt({ hash })

  // Get the new pair address
  const pair = await client.readContract({
    address: factory,
    abi: FACTORY_ABI,
    functionName: 'getPair',
    args: [tokenA, tokenB],
  })

  return pair as Address
}

/**
 * Get the pair address for two tokens (returns zero address if not exists)
 */
export async function getPair(
  client: AnyClient,
  tokenA: Address,
  tokenB: Address
): Promise<Address> {
  const { factory } = getUniV2Addresses()

  const pair = await client.readContract({
    address: factory,
    abi: FACTORY_ABI,
    functionName: 'getPair',
    args: [tokenA, tokenB],
  })

  return pair as Address
}

/**
 * Check if a pair exists
 */
export async function pairExists(
  client: AnyClient,
  tokenA: Address,
  tokenB: Address
): Promise<boolean> {
  const pair = await getPair(client, tokenA, tokenB)
  return pair !== ZERO_ADDRESS
}

/**
 * Get reserves for a pair
 */
export async function getPairReserves(
  client: AnyClient,
  pairAddress: Address
): Promise<PairReserves> {
  const [reserves, token0, token1] = await Promise.all([
    client.readContract({
      address: pairAddress,
      abi: PAIR_ABI,
      functionName: 'getReserves',
    }),
    client.readContract({
      address: pairAddress,
      abi: PAIR_ABI,
      functionName: 'token0',
    }),
    client.readContract({
      address: pairAddress,
      abi: PAIR_ABI,
      functionName: 'token1',
    }),
  ])

  const [reserve0, reserve1, blockTimestampLast] = reserves as [bigint, bigint, number]

  return {
    reserve0,
    reserve1,
    blockTimestampLast,
    token0: token0 as Address,
    token1: token1 as Address,
  }
}

/**
 * Get reserves for a token pair (ordered by token address)
 */
export async function getReservesForTokens(
  client: AnyClient,
  tokenA: Address,
  tokenB: Address
): Promise<{ reserveA: bigint; reserveB: bigint } | null> {
  const pair = await getPair(client, tokenA, tokenB)
  
  if (pair === ZERO_ADDRESS) {
    return null
  }

  const reserves = await getPairReserves(client, pair)
  
  // Return reserves in the order of input tokens
  if (tokenA.toLowerCase() < tokenB.toLowerCase()) {
    return { reserveA: reserves.reserve0, reserveB: reserves.reserve1 }
  } else {
    return { reserveA: reserves.reserve1, reserveB: reserves.reserve0 }
  }
}

/**
 * Get the number of pairs created
 */
export async function getAllPairsLength(client: AnyClient): Promise<bigint> {
  const { factory } = getUniV2Addresses()

  return client.readContract({
    address: factory,
    abi: FACTORY_ABI,
    functionName: 'allPairsLength',
  })
}

/**
 * Get pair address by index
 */
export async function getPairByIndex(client: AnyClient, index: bigint): Promise<Address> {
  const { factory } = getUniV2Addresses()

  return client.readContract({
    address: factory,
    abi: FACTORY_ABI,
    functionName: 'allPairs',
    args: [index],
  }) as Promise<Address>
}
