/**
 * UniswapV2 Contract Addresses and ABIs
 * 
 * This module provides type definitions, ABIs, and address management for UniV2 contracts.
 */

import { type Address, parseAbi } from 'viem'
import { readFileSync, writeFileSync, existsSync } from 'fs'
import { join } from 'path'
import { getProjectRoot } from '../deploy'

// Config file path for deployed addresses
const CONFIG_FILE = '.univ2-config.json'

export interface UniV2Config {
  factory: Address
  router: Address
  weth: Address // Using PATH_USD as WETH equivalent
  chainId: number
  deployedAt: string
}

// Factory ABI - core pair creation and management
export const FACTORY_ABI = parseAbi([
  'event PairCreated(address indexed token0, address indexed token1, address pair, uint256)',
  'function feeTo() external view returns (address)',
  'function feeToSetter() external view returns (address)',
  'function getPair(address tokenA, address tokenB) external view returns (address pair)',
  'function allPairs(uint256) external view returns (address pair)',
  'function allPairsLength() external view returns (uint256)',
  'function createPair(address tokenA, address tokenB) external returns (address pair)',
  'function setFeeTo(address) external',
  'function setFeeToSetter(address) external',
])

// Router02 ABI - liquidity and swap functions
export const ROUTER_ABI = parseAbi([
  'function factory() external view returns (address)',
  'function WETH() external view returns (address)',
  // Liquidity
  'function addLiquidity(address tokenA, address tokenB, uint256 amountADesired, uint256 amountBDesired, uint256 amountAMin, uint256 amountBMin, address to, uint256 deadline) external returns (uint256 amountA, uint256 amountB, uint256 liquidity)',
  'function addLiquidityETH(address token, uint256 amountTokenDesired, uint256 amountTokenMin, uint256 amountETHMin, address to, uint256 deadline) external payable returns (uint256 amountToken, uint256 amountETH, uint256 liquidity)',
  'function removeLiquidity(address tokenA, address tokenB, uint256 liquidity, uint256 amountAMin, uint256 amountBMin, address to, uint256 deadline) external returns (uint256 amountA, uint256 amountB)',
  'function removeLiquidityETH(address token, uint256 liquidity, uint256 amountTokenMin, uint256 amountETHMin, address to, uint256 deadline) external returns (uint256 amountToken, uint256 amountETH)',
  'function removeLiquidityWithPermit(address tokenA, address tokenB, uint256 liquidity, uint256 amountAMin, uint256 amountBMin, address to, uint256 deadline, bool approveMax, uint8 v, bytes32 r, bytes32 s) external returns (uint256 amountA, uint256 amountB)',
  'function removeLiquidityETHWithPermit(address token, uint256 liquidity, uint256 amountTokenMin, uint256 amountETHMin, address to, uint256 deadline, bool approveMax, uint8 v, bytes32 r, bytes32 s) external returns (uint256 amountToken, uint256 amountETH)',
  // Swaps
  'function swapExactTokensForTokens(uint256 amountIn, uint256 amountOutMin, address[] calldata path, address to, uint256 deadline) external returns (uint256[] memory amounts)',
  'function swapTokensForExactTokens(uint256 amountOut, uint256 amountInMax, address[] calldata path, address to, uint256 deadline) external returns (uint256[] memory amounts)',
  'function swapExactETHForTokens(uint256 amountOutMin, address[] calldata path, address to, uint256 deadline) external payable returns (uint256[] memory amounts)',
  'function swapTokensForExactETH(uint256 amountOut, uint256 amountInMax, address[] calldata path, address to, uint256 deadline) external returns (uint256[] memory amounts)',
  'function swapExactTokensForETH(uint256 amountIn, uint256 amountOutMin, address[] calldata path, address to, uint256 deadline) external returns (uint256[] memory amounts)',
  'function swapETHForExactTokens(uint256 amountOut, address[] calldata path, address to, uint256 deadline) external payable returns (uint256[] memory amounts)',
  // Quotes
  'function quote(uint256 amountA, uint256 reserveA, uint256 reserveB) external pure returns (uint256 amountB)',
  'function getAmountOut(uint256 amountIn, uint256 reserveIn, uint256 reserveOut) external pure returns (uint256 amountOut)',
  'function getAmountIn(uint256 amountOut, uint256 reserveIn, uint256 reserveOut) external pure returns (uint256 amountIn)',
  'function getAmountsOut(uint256 amountIn, address[] calldata path) external view returns (uint256[] memory amounts)',
  'function getAmountsIn(uint256 amountOut, address[] calldata path) external view returns (uint256[] memory amounts)',
])

// Pair ABI - LP token and reserve queries
export const PAIR_ABI = parseAbi([
  'event Approval(address indexed owner, address indexed spender, uint256 value)',
  'event Transfer(address indexed from, address indexed to, uint256 value)',
  'event Mint(address indexed sender, uint256 amount0, uint256 amount1)',
  'event Burn(address indexed sender, uint256 amount0, uint256 amount1, address indexed to)',
  'event Swap(address indexed sender, uint256 amount0In, uint256 amount1In, uint256 amount0Out, uint256 amount1Out, address indexed to)',
  'event Sync(uint112 reserve0, uint112 reserve1)',
  'function name() external view returns (string memory)',
  'function symbol() external view returns (string memory)',
  'function decimals() external view returns (uint8)',
  'function totalSupply() external view returns (uint256)',
  'function balanceOf(address owner) external view returns (uint256)',
  'function allowance(address owner, address spender) external view returns (uint256)',
  'function approve(address spender, uint256 value) external returns (bool)',
  'function transfer(address to, uint256 value) external returns (bool)',
  'function transferFrom(address from, address to, uint256 value) external returns (bool)',
  'function DOMAIN_SEPARATOR() external view returns (bytes32)',
  'function PERMIT_TYPEHASH() external pure returns (bytes32)',
  'function nonces(address owner) external view returns (uint256)',
  'function permit(address owner, address spender, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s) external',
  'function MINIMUM_LIQUIDITY() external pure returns (uint256)',
  'function factory() external view returns (address)',
  'function token0() external view returns (address)',
  'function token1() external view returns (address)',
  'function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)',
  'function price0CumulativeLast() external view returns (uint256)',
  'function price1CumulativeLast() external view returns (uint256)',
  'function kLast() external view returns (uint256)',
  'function mint(address to) external returns (uint256 liquidity)',
  'function burn(address to) external returns (uint256 amount0, uint256 amount1)',
  'function swap(uint256 amount0Out, uint256 amount1Out, address to, bytes calldata data) external',
  'function skim(address to) external',
  'function sync() external',
  'function initialize(address, address) external',
])

// ERC20 ABI for token approvals
export const ERC20_ABI = parseAbi([
  'function approve(address spender, uint256 amount) external returns (bool)',
  'function allowance(address owner, address spender) external view returns (uint256)',
  'function balanceOf(address account) external view returns (uint256)',
  'function transfer(address to, uint256 amount) external returns (bool)',
  'function transferFrom(address from, address to, uint256 amount) external returns (bool)',
])

/**
 * Load UniV2 config from file
 */
export function loadUniV2Config(): UniV2Config | null {
  const projectRoot = getProjectRoot()
  const configPath = join(projectRoot, CONFIG_FILE)
  
  if (!existsSync(configPath)) {
    return null
  }
  
  try {
    const content = readFileSync(configPath, 'utf-8')
    return JSON.parse(content) as UniV2Config
  } catch {
    return null
  }
}

/**
 * Save UniV2 config to file
 */
export function saveUniV2Config(config: UniV2Config): void {
  const projectRoot = getProjectRoot()
  const configPath = join(projectRoot, CONFIG_FILE)
  writeFileSync(configPath, JSON.stringify(config, null, 2))
}

/**
 * Get UniV2 addresses (throws if not deployed)
 */
export function getUniV2Addresses(): UniV2Config {
  const config = loadUniV2Config()
  if (!config) {
    throw new Error('UniV2 contracts not deployed. Run deployUniV2() first.')
  }
  return config
}
