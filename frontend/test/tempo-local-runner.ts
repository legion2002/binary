#!/usr/bin/env bun
/**
 * Start a local Tempo node and deploy Binary contracts
 * 
 * Usage: bun run test/tempo-local-runner.ts
 */

import { createClient, http, publicActions, walletActions, type Chain } from 'viem'
import { privateKeyToAccount } from 'viem/accounts'
import { tempoActions } from 'viem/tempo'
import { Server } from 'prool'
import { Instance } from 'prool/testcontainers'
import { execSync } from 'child_process'

// Configuration
const PORT = 9545
const BLOCK_TIME = '100ms'
const IMAGE = 'ghcr.io/tempoxyz/tempo:latest'
const TEST_PRIVATE_KEY = '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80' as const
const PATH_USD = '0x20C0000000000000000000000000000000000000' as const

async function main() {
  console.log('==========================================')
  console.log('Binary Protocol - Local Tempo Testing')
  console.log('==========================================')
  console.log('')

  // Start local Tempo node
  console.log('Step 1: Starting local Tempo node...')
  console.log(`  Image: ${IMAGE}`)
  console.log(`  Port: ${PORT}`)
  
  const server = Server.create({
    instance: Instance.tempo({
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

  // Use base URL for forge (pool ID suffix may not be compatible)
  const rpcUrl = `http://localhost:${PORT}`
  console.log(`  RPC URL: ${rpcUrl}`)
  console.log('')

  // Create client
  const account = privateKeyToAccount(TEST_PRIVATE_KEY)
  
  const localChain: Chain = {
    id: 42429,
    name: 'Tempo Local',
    nativeCurrency: { name: 'USD', symbol: 'USD', decimals: 6 },
    rpcUrls: {
      default: { http: [rpcUrl] },
    },
  }

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

  // Fund test account by minting PathUSD (local node has open minting)
  console.log('Step 2: Funding test account...')
  try {
    // On local node, we can mint tokens directly using token.mint
    // The deployer account has ISSUER_ROLE on the local node
    const mintAmount = BigInt(1_000_000) * BigInt(1e6) // 1M PathUSD
    
    await client.token.mint({
      token: PATH_USD,
      to: account.address,
      amount: mintAmount,
    })
    console.log(`  ✓ Minted 1,000,000 PathUSD`)
    
    await new Promise(resolve => setTimeout(resolve, 500))
    
    const balance = await client.token.getBalance({
      token: PATH_USD,
      address: account.address,
    })
    console.log(`  Balance: ${Number(balance) / 1e6} PathUSD`)
  } catch (error: any) {
    console.log('  Note: Minting may not work on fresh local node')
    console.log('  Continuing with deployment...')
  }
  console.log('')

  // Deploy contracts
  console.log('Step 3: Deploying contracts locally...')
  
  // Go up from frontend to project root
  const projectRoot = process.cwd().replace('/frontend', '')

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
  }

  console.log('')
  console.log('==========================================')
  console.log('Local Testing Environment Ready!')
  console.log('==========================================')
  console.log('')
  console.log(`RPC URL: ${rpcUrl}`)
  console.log(`Test Account: ${account.address}`)
  console.log(`Private Key: ${TEST_PRIVATE_KEY}`)
  console.log('')
  console.log('Add to MetaMask:')
  console.log(`  Network Name: Tempo Local`)
  console.log(`  RPC URL: ${rpcUrl}`)
  console.log(`  Chain ID: 42429`)
  console.log(`  Currency: USD`)
  console.log('')
  console.log('Press Ctrl+C to stop the node.')
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
