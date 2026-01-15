/**
 * UniswapV2 Infrastructure for Tempo
 * 
 * This module provides a complete UniV2 implementation for Tempo blockchain,
 * replacing the Stablecoin DEX precompile.
 * 
 * Usage:
 * ```typescript
 * import { deployUniV2, createPair, addLiquidity, swapExactTokensForTokens } from './lib/univ2'
 * 
 * // Deploy UniV2 contracts (idempotent)
 * const { factory, router } = await deployUniV2(client)
 * 
 * // Create a trading pair
 * const pair = await createPair(client, tokenA, tokenB)
 * 
 * // Add liquidity
 * await addLiquidity(client, { tokenA, tokenB, amountA, amountB })
 * 
 * // Swap tokens
 * await swapExactTokensForTokens(client, {
 *   amountIn: 1000n,
 *   amountOutMin: 950n,
 *   path: [tokenA, tokenB],
 * })
 * ```
 */

// Addresses and ABIs
export {
  FACTORY_ABI,
  ROUTER_ABI,
  PAIR_ABI,
  ERC20_ABI,
  loadUniV2Config,
  saveUniV2Config,
  getUniV2Addresses,
  type UniV2Config,
} from './addresses'

// Deployment
export {
  deployUniV2,
  ensureUniV2Deployed,
  initUniV2Config,
  type DeployUniV2Options,
  type DeployUniV2Result,
} from './deploy'

// Pair management
export {
  createPair,
  getPair,
  pairExists,
  getPairReserves,
  getReservesForTokens,
  getAllPairsLength,
  getPairByIndex,
  type PairReserves,
} from './pairs'

// Liquidity
export {
  addLiquidity,
  removeLiquidity,
  addLiquidityEqual,
  ensureRouterApproval,
  type AddLiquidityParams,
  type AddLiquidityResult,
  type RemoveLiquidityParams,
  type RemoveLiquidityResult,
} from './liquidity'

// Swaps
export {
  swapExactTokensForTokens,
  swapTokensForExactTokens,
  getAmountsOut,
  getAmountsIn,
  quote,
  getAmountOut,
  getAmountIn,
  simpleSwap,
  type SwapExactTokensParams,
  type SwapTokensForExactParams,
} from './swap'
