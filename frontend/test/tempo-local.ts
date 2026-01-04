/**
 * Local Tempo Node Setup using Prool + Docker
 * 
 * This module provides utilities for running a local Tempo node for testing.
 * Based on viem's test infrastructure.
 * 
 * Usage:
 *   import { createTempoServer, rpcUrl, getClient } from './tempo-local'
 *   
 *   // Start server (runs Docker container)
 *   const server = await createTempoServer()
 *   await server.start()
 *   
 *   // Use client
 *   const client = getClient()
 *   
 *   // Stop when done
 *   await server.stop()
 */

import { createClient, http, publicActions, walletActions, type Chain } from 'viem'
import { privateKeyToAccount } from 'viem/accounts'
import { tempoTestnet } from 'viem/chains'
import { tempoActions } from 'viem/tempo'
import { createServer } from 'prool'
import { tempo } from 'prool/instances'

// ============================================
// Configuration
// ============================================

const PORT = 9545
const BLOCK_TIME = '100ms' // Fast blocks for testing

// Test mnemonic (DO NOT use in production)
const TEST_MNEMONIC = 'test test test test test test test test test test test junk'

// Derive test accounts from mnemonic
export const testAccounts = Array.from({ length: 10 }, (_, i) => {
  // Simple derivation - in production use proper HD wallet
  const privateKey = `0x${(BigInt('0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80') + BigInt(i)).toString(16).padStart(64, '0')}` as `0x${string}`
  return privateKeyToAccount(privateKey)
})

// ============================================
// RPC URL
// ============================================

export function getRpcUrl(poolId = 1) {
  return `http://localhost:${PORT}/${poolId}`
}

export const rpcUrl = getRpcUrl()

// ============================================
// Local Tempo Chain Definition
// ============================================

export const tempoLocal: Chain = {
  ...tempoTestnet,
  id: 42429,
  name: 'Tempo Local',
  rpcUrls: {
    default: {
      http: [rpcUrl],
      webSocket: [`ws://localhost:${PORT}`],
    },
  },
}

// ============================================
// Server Creation
// ============================================

export async function createTempoServer(options?: {
  port?: number
  blockTime?: string
  image?: string
}) {
  const {
    port = PORT,
    blockTime = BLOCK_TIME,
    image = 'ghcr.io/tempoxyz/tempo:latest',
  } = options ?? {}

  console.log('Creating Tempo local server...')
  console.log(`  Image: ${image}`)
  console.log(`  Port: ${port}`)
  console.log(`  Block time: ${blockTime}`)

  const server = createServer({
    instance: tempo({
      blockTime,
      image,
      port,
    }),
    port,
  })

  return {
    async start() {
      console.log('Starting Tempo Docker container...')
      await server.start()
      console.log(`Tempo node running at ${getRpcUrl()}`)
      return this
    },
    async stop() {
      console.log('Stopping Tempo Docker container...')
      await server.stop()
    },
    async restart() {
      await fetch(`${getRpcUrl()}/restart`)
    },
    rpcUrl: getRpcUrl(),
  }
}

// ============================================
// Client Creation
// ============================================

export function getClient(options?: {
  account?: ReturnType<typeof privateKeyToAccount>
  rpcUrl?: string
}) {
  const {
    account = testAccounts[0],
    rpcUrl: url = rpcUrl,
  } = options ?? {}

  return createClient({
    account,
    chain: tempoLocal,
    transport: http(url),
    pollingInterval: 100, // Fast polling for tests
  })
    .extend(publicActions)
    .extend(walletActions)
    .extend(tempoActions())
}

// ============================================
// Test Utilities
// ============================================

export const PATH_USD = '0x20C0000000000000000000000000000000000000' as const
export const ALPHA_USD = '0x20C0000000000000000000000000000000000001' as const

/**
 * Fund an address with testnet tokens via faucet
 */
export async function fundAddress(
  client: ReturnType<typeof getClient>,
  address: `0x${string}`
) {
  const hashes = await client.faucet.fund({ account: address })
  // Wait for confirmation
  await new Promise(resolve => setTimeout(resolve, 500))
  return hashes
}

/**
 * Get PathUSD balance for an address
 */
export async function getPathUsdBalance(
  client: ReturnType<typeof getClient>,
  address: `0x${string}`
) {
  return client.token.getBalance({
    token: PATH_USD,
    address,
  })
}

// ============================================
// Quick Start Helper
// ============================================

/**
 * Quick start a local Tempo environment
 * 
 * @example
 * const { client, server, accounts } = await startLocalTempo()
 * // ... run tests
 * await server.stop()
 */
export async function startLocalTempo() {
  const server = await createTempoServer()
  await server.start()
  
  const client = getClient()
  
  // Fund the first test account
  await fundAddress(client, testAccounts[0].address)
  
  return {
    server,
    client,
    accounts: testAccounts,
    rpcUrl,
  }
}
