/**
 * Contract deployment utilities
 */

import { execSync } from 'child_process'
import { readFileSync, existsSync } from 'fs'
import { join } from 'path'
import { TEST_PRIVATE_KEY } from './tempo'
import { TESTNET_RPC_URL, TESTNET_CHAIN_ID } from './testnet'

export interface ContractAddresses {
  multiverse: string
  oracle: string
}

export interface DeployOptions {
  rpcUrl: string
  privateKey?: string
  projectRoot?: string
  verbose?: boolean
  /** Chain ID for broadcast file lookup */
  chainId?: number
}

export async function deployContracts(options: DeployOptions): Promise<ContractAddresses> {
  const {
    rpcUrl,
    privateKey = TEST_PRIVATE_KEY,
    projectRoot = process.cwd(),
    verbose = false,
    chainId,
  } = options

  const verbosity = verbose ? '-vvvv' : '-vvv'

  // Deploy using standard EVM transactions.
  // The user's fee token preference must be set to PathUSD BEFORE calling this
  // (via setUserFeeToken in env.ts). This ensures all transactions use PathUSD for fees.
  // Note: Markets are created via admin API after backend starts, not in this script.
  execSync(
    `forge script script/Deploy.s.sol:Deploy \
      --rpc-url ${rpcUrl} \
      --broadcast \
      --private-key "${privateKey}" \
      ${verbosity}`,
    {
      stdio: verbose ? 'inherit' : 'pipe',
      cwd: `${projectRoot}/contracts`,
    }
  )

  // Parse broadcast JSON to get contract addresses
  const addresses = parseBroadcastAddresses(projectRoot, chainId)
  return addresses
}

/**
 * Deploy contracts to Tempo testnet
 * Uses the testnet RPC URL and expects a funded private key
 */
export async function deployContractsTestnet(options: {
  privateKey: string
  projectRoot?: string
  verbose?: boolean
}): Promise<ContractAddresses> {
  const {
    privateKey,
    projectRoot = process.cwd(),
    verbose = true,
  } = options

  console.log('  Deploying contracts to Tempo testnet...')

  // Deploy using forge script from tempo-foundry (requires alloy-chains 0.2.25+ for chain 42431)
  // Use local tempo-foundry build if available, otherwise fall back to system forge
  const forgeCmd = process.env.FORGE_PATH || 'forge'
  
  // --chain: Explicitly set chain ID for Tempo Moderato testnet (42431)
  // --skip-simulation: Skip simulation since Tempo uses custom precompiles
  // --slow: Send transactions one at a time to avoid nonce issues
  execSync(
    `${forgeCmd} script script/Deploy.s.sol:Deploy \
      --rpc-url ${TESTNET_RPC_URL} \
      --chain ${TESTNET_CHAIN_ID} \
      --broadcast \
      --private-key "${privateKey}" \
      --skip-simulation \
      --slow \
      -vvv`,
    {
      stdio: verbose ? 'inherit' : 'pipe',
      cwd: `${projectRoot}/contracts`,
    }
  )

  // Parse broadcast JSON to get contract addresses
  const addresses = parseBroadcastAddresses(projectRoot, TESTNET_CHAIN_ID)
  return addresses
}

export function parseBroadcastAddresses(projectRoot: string, specificChainId?: number): ContractAddresses {
  // If specific chain ID provided, try that first
  // Otherwise try dev chain ID first (1337), then Tempo testnet (42431)
  const chainIds = specificChainId
    ? [String(specificChainId), '1337', '42431']
    : ['1337', '42431']
  let broadcastPath: string | undefined

  for (const chainId of chainIds) {
    const path = join(projectRoot, `contracts/broadcast/Deploy.s.sol/${chainId}/run-latest.json`)
    if (existsSync(path)) {
      broadcastPath = path
      break
    }
  }

  if (!broadcastPath) {
    throw new Error(`Broadcast file not found for chain IDs: ${chainIds.join(', ')}`)
  }

  const broadcastJson = JSON.parse(readFileSync(broadcastPath, 'utf-8'))
  const transactions = broadcastJson.transactions as Array<{
    contractName?: string
    contractAddress?: string
  }>

  let multiverse: string | undefined
  let oracle: string | undefined

  for (const tx of transactions) {
    if (tx.contractName === 'MultiVerse' && tx.contractAddress) {
      multiverse = tx.contractAddress
    }
    if (tx.contractName === 'TrustedOracle' && tx.contractAddress) {
      oracle = tx.contractAddress
    }
  }

  if (!multiverse) {
    throw new Error('MultiVerse address not found in broadcast')
  }
  if (!oracle) {
    throw new Error('TrustedOracle address not found in broadcast')
  }

  return { multiverse, oracle }
}

export function getProjectRoot(): string {
  // Handle running from different directories
  let cwd = process.cwd()

  // If we're in frontend or backend, go up
  if (cwd.endsWith('/frontend') || cwd.endsWith('/backend')) {
    cwd = join(cwd, '..')
  }

  // If we're in script/lib, go up twice
  if (cwd.endsWith('/script/lib')) {
    cwd = join(cwd, '../..')
  }

  // If we're in script, go up once
  if (cwd.endsWith('/script')) {
    cwd = join(cwd, '..')
  }

  return cwd
}
