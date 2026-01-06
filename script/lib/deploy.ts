/**
 * Contract deployment utilities
 */

import { execSync } from 'child_process'
import { readFileSync, existsSync } from 'fs'
import { join } from 'path'
import { TEST_PRIVATE_KEY } from './tempo'

export interface ContractAddresses {
  multiverse: string
  oracle: string
}

export interface DeployOptions {
  rpcUrl: string
  privateKey?: string
  projectRoot?: string
  verbose?: boolean
}

export async function deployContracts(options: DeployOptions): Promise<ContractAddresses> {
  const {
    rpcUrl,
    privateKey = TEST_PRIVATE_KEY,
    projectRoot = process.cwd(),
    verbose = false,
  } = options

  const verbosity = verbose ? '-vvvv' : '-vvv'

  // Deploy using standard EVM transactions.
  // The user's fee token preference must be set to PathUSD BEFORE calling this
  // (via setUserFeeToken in env.ts). This ensures all transactions use PathUSD for fees.
  execSync(
    `forge script script/DeployAndSeedMarkets.s.sol:DeployAndSeedMarkets \
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
  const addresses = parseBroadcastAddresses(projectRoot)
  return addresses
}

export function parseBroadcastAddresses(projectRoot: string): ContractAddresses {
  // Try dev chain ID first (1337), then Tempo testnet (42429)
  const chainIds = ['1337', '42429']
  let broadcastPath: string | undefined

  for (const chainId of chainIds) {
    const path = join(projectRoot, `contracts/broadcast/DeployAndSeedMarkets.s.sol/${chainId}/run-latest.json`)
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
