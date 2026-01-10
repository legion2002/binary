/**
 * Tempo Testnet utilities
 * Handles client creation, faucet funding, and fee sponsorship for testnet
 */

import { createClient, http, publicActions, walletActions, type Chain } from 'viem'
import { privateKeyToAccount } from 'viem/accounts'
import { tempoActions, withFeePayer } from 'viem/tempo'
import { existsSync, readFileSync, writeFileSync } from 'fs'
import { join } from 'path'
import { getProjectRoot } from './deploy'
import { sleep } from './health'

// Token addresses
export const PATH_USD = '0x20C0000000000000000000000000000000000000' as const
export const ALPHA_USD = '0x20C0000000000000000000000000000000000001' as const

// Testnet configuration
export const TESTNET_RPC_URL = 'https://rpc.moderato.tempo.xyz'
export const TESTNET_FEE_SPONSOR_URL = 'https://sponsor.moderato.tempo.xyz'
export const TESTNET_CHAIN_ID = 42431

// Define Tempo Moderato chain (chain ID 42431) - not yet in published viem
// Using a simple chain definition without tempo-specific formatters for now
export const tempoModerato = {
  id: TESTNET_CHAIN_ID,
  name: 'Tempo Testnet (Moderato)',
  nativeCurrency: {
    name: 'USD',
    symbol: 'USD',
    decimals: 6,
  },
  rpcUrls: {
    default: {
      http: [TESTNET_RPC_URL],
      webSocket: ['wss://rpc.moderato.tempo.xyz'],
    },
  },
  blockExplorers: {
    default: {
      name: 'Tempo Explorer',
      url: 'https://explore.moderato.tempo.xyz',
    },
  },
} as const satisfies Chain

// Minimum balance required for operations (100 PathUSD)
const MIN_BALANCE = BigInt(100) * BigInt(1e6)

// Config file for persisting deployed addresses
const CONFIG_FILE = '.testnet-config.json'

export interface TestnetConfig {
  multiverse?: string
  oracle?: string
  deployedAt?: string
  deployerAddress?: string
}

export function loadTestnetConfig(): TestnetConfig {
  const projectRoot = getProjectRoot()
  const configPath = join(projectRoot, CONFIG_FILE)

  if (existsSync(configPath)) {
    try {
      return JSON.parse(readFileSync(configPath, 'utf-8'))
    } catch {
      return {}
    }
  }
  return {}
}

export function saveTestnetConfig(config: TestnetConfig): void {
  const projectRoot = getProjectRoot()
  const configPath = join(projectRoot, CONFIG_FILE)
  writeFileSync(configPath, JSON.stringify(config, null, 2))
}

export function getTestnetClient(privateKey: `0x${string}`) {
  const account = privateKeyToAccount(privateKey)

  // Use fee sponsorship for all transactions
  const client = createClient({
    account,
    chain: tempoModerato,
    transport: withFeePayer(
      http(TESTNET_RPC_URL),
      http(TESTNET_FEE_SPONSOR_URL)
    ),
  })
    .extend(publicActions)
    .extend(walletActions)
    .extend(tempoActions())

  return client
}

export type TestnetClient = ReturnType<typeof getTestnetClient>

/**
 * Fund account via faucet if balance is below minimum
 */
export async function ensureFunded(
  client: TestnetClient,
  minBalance: bigint = MIN_BALANCE
): Promise<{ funded: boolean; balance: bigint }> {
  const address = client.account.address

  // Check current balance
  let balance = await client.token.getBalance({
    token: PATH_USD,
    address,
  })

  if (balance >= minBalance) {
    return { funded: false, balance }
  }

  console.log(`  Current balance: ${Number(balance) / 1e6} PathUSD (below ${Number(minBalance) / 1e6} minimum)`)
  console.log('  Requesting funds from faucet...')

  try {
    const hashes = await client.faucet.fund({
      account: address,
    })
    console.log(`  Faucet transactions: ${hashes.length}`)

    // Wait for confirmation
    await sleep(3000)

    // Check new balance
    balance = await client.token.getBalance({
      token: PATH_USD,
      address,
    })
    console.log(`  New balance: ${Number(balance) / 1e6} PathUSD`)

    return { funded: true, balance }
  } catch (error: any) {
    console.error('  Faucet error:', error.message)
    throw new Error(
      `Failed to fund account. Please fund manually at https://docs.tempo.xyz/quickstart/faucet\nAddress: ${address}`
    )
  }
}

/**
 * Check if contracts are deployed by verifying bytecode exists at addresses
 */
export async function checkContractsDeployed(
  client: TestnetClient,
  config: TestnetConfig
): Promise<boolean> {
  if (!config.multiverse || !config.oracle) {
    return false
  }

  try {
    const [multiverseCode, oracleCode] = await Promise.all([
      client.getCode({ address: config.multiverse as `0x${string}` }),
      client.getCode({ address: config.oracle as `0x${string}` }),
    ])

    // Bytecode exists if not empty (0x)
    const multiverseDeployed = multiverseCode && multiverseCode !== '0x' && multiverseCode.length > 2
    const oracleDeployed = oracleCode && oracleCode !== '0x' && oracleCode.length > 2

    return multiverseDeployed && oracleDeployed
  } catch {
    return false
  }
}
