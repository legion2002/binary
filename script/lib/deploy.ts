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

  execSync(
    `forge script script/DeployAndSeedMarkets.s.sol:DeployAndSeedMarkets \
      --rpc-url ${rpcUrl} \
      --broadcast \
      --private-key "${privateKey}" \
      ${verbosity}`,
    {
      stdio: verbose ? 'inherit' : 'pipe',
      cwd: projectRoot,
    }
  )

  // Parse broadcast JSON to get contract addresses
  const addresses = parseBroadcastAddresses(projectRoot)
  return addresses
}

export function parseBroadcastAddresses(projectRoot: string): ContractAddresses {
  // Tempo chain ID is 42429
  const broadcastPath = join(
    projectRoot,
    'broadcast/DeployAndSeedMarkets.s.sol/42429/run-latest.json'
  )

  if (!existsSync(broadcastPath)) {
    throw new Error(`Broadcast file not found: ${broadcastPath}`)
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
