/**
 * UniswapV2 Deployment Script
 * 
 * Deploys UniV2 Factory and Router02 contracts to Tempo.
 * Uses PATH_USD as the "WETH" equivalent since Tempo is stablecoin-native.
 */

import { type Address, type PublicClient, type WalletClient, parseAbi, encodeDeployData } from 'viem'
import { execSync } from 'child_process'
import { existsSync, readFileSync } from 'fs'
import { join } from 'path'
import { getProjectRoot } from '../deploy'
import { PATH_USD } from '../tempo'
import { saveUniV2Config, loadUniV2Config, type UniV2Config } from './addresses'

// Generic client type compatible with Tempo client
// eslint-disable-next-line @typescript-eslint/no-explicit-any
type AnyClient = any

export interface DeployUniV2Options {
  /** Fee token setter address (usually deployer) */
  feeToSetter?: Address
  /** Force redeploy even if already deployed */
  force?: boolean
  /** Verbose output */
  verbose?: boolean
}

export interface DeployUniV2Result {
  factory: Address
  router: Address
  weth: Address
}

/**
 * Check if a contract exists at an address
 */
async function contractExists(client: AnyClient, address: Address): Promise<boolean> {
  try {
    const code = await client.getCode({ address })
    return code !== undefined && code !== '0x' && code.length > 2
  } catch {
    return false
  }
}

/**
 * Parse forge broadcast output to get deployed address
 */
function parseForgeAddress(output: string, contractName: string): Address | null {
  // Look for "Deployed to: 0x..." pattern
  const deployedMatch = output.match(new RegExp(`${contractName}.*?deployed.*?to:?\\s*(0x[a-fA-F0-9]{40})`, 'i'))
  if (deployedMatch) {
    return deployedMatch[1] as Address
  }
  
  // Also try "Contract Address: 0x..." pattern
  const contractMatch = output.match(/Contract Address:\s*(0x[a-fA-F0-9]{40})/i)
  if (contractMatch) {
    return contractMatch[1] as Address
  }
  
  return null
}

/**
 * Deploy UniV2 contracts using forge script
 */
export async function deployUniV2(
  client: AnyClient,
  options: DeployUniV2Options = {}
): Promise<DeployUniV2Result> {
  const {
    feeToSetter = client.account.address,
    force = false,
    verbose = false,
  } = options

  const projectRoot = getProjectRoot()
  const chainId = await client.getChainId()

  // Check if already deployed
  if (!force) {
    const existing = loadUniV2Config()
    if (existing && existing.chainId === chainId) {
      const factoryExists = await contractExists(client, existing.factory)
      const routerExists = await contractExists(client, existing.router)
      
      if (factoryExists && routerExists) {
        console.log('  UniV2 contracts already deployed:')
        console.log(`    Factory: ${existing.factory}`)
        console.log(`    Router:  ${existing.router}`)
        return {
          factory: existing.factory,
          router: existing.router,
          weth: existing.weth,
        }
      }
    }
  }

  console.log('  Deploying UniV2 contracts...')

  // Deploy via forge script
  const rpcUrl = client.transport.url || `http://localhost:9545`
  const privateKey = client.account.source === 'privateKey' 
    ? client.account.privateKey 
    : process.env.PRIVATE_KEY

  if (!privateKey) {
    throw new Error('Private key not available for deployment')
  }

  const verbosity = verbose ? '-vvvv' : '-vvv'

  try {
    execSync(
      `forge script script/DeployUniV2.s.sol:DeployUniV2 \
        --rpc-url ${rpcUrl} \
        --broadcast \
        --private-key "${privateKey}" \
        ${verbosity}`,
      {
        stdio: verbose ? 'inherit' : 'pipe',
        cwd: `${projectRoot}/contracts`,
      }
    )
  } catch (e) {
    throw new Error(`Failed to deploy UniV2: ${e instanceof Error ? e.message : String(e)}`)
  }

  // Parse broadcast JSON to get addresses
  const broadcastPath = join(projectRoot, `contracts/broadcast/DeployUniV2.s.sol/${chainId}/run-latest.json`)
  
  if (!existsSync(broadcastPath)) {
    throw new Error(`Broadcast file not found: ${broadcastPath}`)
  }

  const broadcast = JSON.parse(readFileSync(broadcastPath, 'utf-8'))
  const transactions = broadcast.transactions as Array<{
    contractName?: string
    contractAddress?: string
  }>

  let factory: Address | undefined
  let router: Address | undefined

  for (const tx of transactions) {
    if (tx.contractName === 'UniswapV2Factory' && tx.contractAddress) {
      factory = tx.contractAddress as Address
    }
    if (tx.contractName === 'UniswapV2Router02' && tx.contractAddress) {
      router = tx.contractAddress as Address
    }
  }

  if (!factory) {
    throw new Error('UniswapV2Factory address not found in broadcast')
  }
  if (!router) {
    throw new Error('UniswapV2Router02 address not found in broadcast')
  }

  // Use PATH_USD as WETH equivalent
  const weth = PATH_USD as Address

  // Save config
  const config: UniV2Config = {
    factory,
    router,
    weth,
    chainId,
    deployedAt: new Date().toISOString(),
  }
  saveUniV2Config(config)

  console.log(`    Factory: ${factory}`)
  console.log(`    Router:  ${router}`)
  console.log(`    WETH:    ${weth} (PATH_USD)`)

  return { factory, router, weth }
}

/**
 * Initialize UniV2 config from already-deployed addresses
 * Use this when addresses come from forge broadcast parsing
 */
export function initUniV2Config(
  factory: Address,
  router: Address,
  chainId: number
): void {
  const config: UniV2Config = {
    factory,
    router,
    weth: PATH_USD as Address,
    chainId,
    deployedAt: new Date().toISOString(),
  }
  saveUniV2Config(config)
}

/**
 * Get deployed UniV2 addresses or deploy if not exists
 */
export async function ensureUniV2Deployed(
  client: AnyClient,
  options: DeployUniV2Options = {}
): Promise<DeployUniV2Result> {
  return deployUniV2(client, { ...options, force: false })
}
