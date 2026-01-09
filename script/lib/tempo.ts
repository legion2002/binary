/**
 * Local Tempo Node utilities using Docker Compose
 */

import { createClient, http, publicActions, walletActions, type Chain } from 'viem'
import { privateKeyToAccount } from 'viem/accounts'
import { tempoActions } from 'viem/tempo'
import { execSync, spawn, type ChildProcess } from 'child_process'
import { waitForRpc, clearLine, sleep } from './health'
import { getProjectRoot } from './deploy'

// Configuration
export const TEMPO_PORT = Number(process.env.TEMPO_PORT) || 9545
export const BLOCK_TIME = '100ms'
export const TEMPO_IMAGE = process.env.TEMPO_IMAGE || 'ghcr.io/tempoxyz/tempo:latest'

// Test private key (from standard test mnemonic - DO NOT use in production)
export const TEST_PRIVATE_KEY = '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80' as const

// Token addresses
export const PATH_USD = '0x20C0000000000000000000000000000000000000' as const
export const ALPHA_USD = '0x20C0000000000000000000000000000000000001' as const

// Local Tempo chain definition
export const tempoLocal: Chain = {
  id: 42431,
  name: 'Tempo Local',
  nativeCurrency: { name: 'USD', symbol: 'USD', decimals: 6 },
  rpcUrls: {
    default: { http: [`http://localhost:${TEMPO_PORT}`] },
  },
}

export interface TempoServer {
  rpcUrl: string
  start: () => Promise<void>
  stop: () => Promise<void>
}

export async function createTempoServer(options?: {
  port?: number
  blockTime?: string
  image?: string
}): Promise<TempoServer> {
  const { port = TEMPO_PORT } = options ?? {}
  const projectRoot = getProjectRoot()
  const rpcUrl = `http://localhost:${port}`

  return {
    rpcUrl,
    async start() {
      // Stop any existing container first
      try {
        execSync('docker compose down', { cwd: projectRoot, stdio: 'pipe' })
      } catch {
        // Ignore if not running
      }

      // Start with docker compose
      execSync('docker compose up -d', { cwd: projectRoot, stdio: 'pipe' })

      // Wait for RPC to be ready
      await waitForRpc(rpcUrl, { timeoutMs: 60_000 })
      clearLine()
    },
    async stop() {
      const projectRoot = getProjectRoot()
      try {
        execSync('docker compose down', { cwd: projectRoot, stdio: 'pipe' })
      } catch {
        // Ignore errors
      }
    },
  }
}

export function getTempoClient(options?: {
  privateKey?: `0x${string}`
  rpcUrl?: string
}) {
  const {
    privateKey = TEST_PRIVATE_KEY,
    rpcUrl = `http://localhost:${TEMPO_PORT}`,
  } = options ?? {}

  const account = privateKeyToAccount(privateKey)

  return createClient({
    account,
    chain: tempoLocal,
    transport: http(rpcUrl),
    pollingInterval: 100,
  })
    .extend(publicActions)
    .extend(walletActions)
    .extend(tempoActions())
}

export type TempoClient = ReturnType<typeof getTempoClient>

export async function fundAccount(
  client: TempoClient,
  address: `0x${string}`,
  amount: bigint = BigInt(1_000_000) * BigInt(1e6) // 1M PathUSD
): Promise<void> {
  try {
    // On local node, we can mint tokens directly
    await client.token.mint({
      token: PATH_USD,
      to: address,
      amount,
    })
    await sleep(500)
  } catch {
    // If minting fails, try faucet (works on testnet)
    try {
      await client.faucet.fund({ account: address })
      await sleep(1000)
    } catch {
      console.warn('  Warning: Could not fund account via mint or faucet')
    }
  }
}

export async function getPathUsdBalance(
  client: TempoClient,
  address: `0x${string}`
): Promise<bigint> {
  return client.token.getBalance({
    token: PATH_USD,
    account: address,
  })
}

export interface TempoEnvironment {
  server: TempoServer
  client: TempoClient
  account: ReturnType<typeof privateKeyToAccount>
  rpcUrl: string
}

export async function startLocalTempo(): Promise<TempoEnvironment> {
  const server = await createTempoServer()
  await server.start()

  const account = privateKeyToAccount(TEST_PRIVATE_KEY)
  const client = getTempoClient({ rpcUrl: server.rpcUrl })

  // In dev mode (chain ID 1337), accounts are prefunded with ETH for gas
  // No need to call faucet - the dev genesis handles this
  console.log(`  Account ${account.address.slice(0, 10)}... ready (dev mode prefunded)`)

  return {
    server,
    client,
    account,
    rpcUrl: server.rpcUrl,
  }
}
