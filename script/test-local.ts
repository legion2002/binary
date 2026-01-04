#!/usr/bin/env bun
/**
 * Test Binary contracts on a local Tempo node
 * 
 * This script:
 * 1. Starts a local Tempo Docker container
 * 2. Funds the test account
 * 3. Deploys contracts locally
 * 4. Creates mock markets
 * 5. Tests basic operations
 * 
 * Usage:
 *   cd frontend && bun run ../script/test-local.ts
 * 
 * Prerequisites:
 *   - Docker running
 *   - bun installed
 *   - forge installed
 */

import { createClient, http, publicActions, walletActions } from 'viem'
import { privateKeyToAccount } from 'viem/accounts'
import { tempoActions } from 'viem/tempo'
import { createServer } from 'prool'
import { tempo } from 'prool/instances'
import { execSync } from 'child_process'

// ============================================
// Configuration
// ============================================

const PORT = 9545
const BLOCK_TIME = '100ms'
const IMAGE = 'ghcr.io/tempoxyz/tempo:latest'

// Test private key (from standard test mnemonic)
const TEST_PRIVATE_KEY = '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80' as const
const PATH_USD = '0x20C0000000000000000000000000000000000000' as const

// ============================================
// Main
// ============================================

async function main() {
  console.log('==========================================')
  console.log('Binary Protocol - Local Tempo Testing')
  console.log('==========================================')
  console.log('')

  // Step 1: Start local Tempo node
  console.log('Step 1: Starting local Tempo node...')
  console.log(`  Image: ${IMAGE}`)
  console.log(`  Port: ${PORT}`)
  
  const server = createServer({
    instance: tempo({
      blockTime: BLOCK_TIME,
      image: IMAGE,
      port: PORT,
    }),
    port: PORT,
  })

  try {
    await server.start()
    console.log('  ✓ Tempo node running')
  } catch (error) {
    console.error('  ✗ Failed to start Tempo node')
    console.error('  Make sure Docker is running!')
    console.error(error)
    process.exit(1)
  }

  const rpcUrl = `http://localhost:${PORT}/1`
  console.log(`  RPC URL: ${rpcUrl}`)
  console.log('')

  // Create client
  const account = privateKeyToAccount(TEST_PRIVATE_KEY)
  
  const localChain = {
    id: 42429,
    name: 'Tempo Local',
    nativeCurrency: { name: 'USD', symbol: 'USD', decimals: 6 },
    rpcUrls: {
      default: { http: [rpcUrl] },
    },
  } as const

  const client = createClient({
    account,
    chain: localChain,
    transport: http(rpcUrl),
    pollingInterval: 100,
  })
    .extend(publicActions)
    .extend(walletActions)
    .extend(tempoActions())

  console.log(`Test account: ${account.address}`)
  console.log('')

  // Step 2: Fund test account
  console.log('Step 2: Funding test account via faucet...')
  try {
    const hashes = await client.faucet.fund({ account: account.address })
    console.log(`  ✓ Funded (${hashes.length} transactions)`)
    
    await new Promise(resolve => setTimeout(resolve, 1000))
    
    const balance = await client.token.getBalance({
      token: PATH_USD,
      address: account.address,
    })
    console.log(`  Balance: ${Number(balance) / 1e6} PathUSD`)
  } catch (error) {
    console.error('  ✗ Faucet failed:', error)
  }
  console.log('')

  // Step 3: Deploy contracts
  console.log('Step 3: Deploying contracts locally...')
  
  const projectRoot = process.cwd().includes('frontend') 
    ? process.cwd().replace('/frontend', '')
    : process.cwd()

  try {
    execSync(
      `forge script script/DeployAndSeedMarkets.s.sol:DeployAndSeedMarkets \
        --rpc-url ${rpcUrl} \
        --broadcast \
        --private-key "${TEST_PRIVATE_KEY}" \
        -vvv`,
      { 
        stdio: 'inherit',
        cwd: projectRoot,
      }
    )
    console.log('')
    console.log('  ✓ Contracts deployed')
  } catch (error) {
    console.error('  ✗ Deployment failed')
    console.error(error)
  }

  console.log('')
  console.log('==========================================')
  console.log('Local Testing Environment Ready!')
  console.log('==========================================')
  console.log('')
  console.log(`RPC URL: ${rpcUrl}`)
  console.log(`Test Account: ${account.address}`)
  console.log('')
  console.log('The local node will keep running.')
  console.log('Press Ctrl+C to stop.')
  console.log('')

  // Keep running until interrupted
  process.on('SIGINT', async () => {
    console.log('')
    console.log('Stopping Tempo node...')
    await server.stop()
    console.log('Done!')
    process.exit(0)
  })

  // Keep process alive
  await new Promise(() => {})
}

main().catch(async (error) => {
  console.error('Error:', error)
  process.exit(1)
})
