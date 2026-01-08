#!/usr/bin/env bun
/**
 * Deploy Binary contracts to Tempo Testnet
 * 
 * This script:
 * 1. Funds the deployer via faucet (if needed)
 * 2. Deploys contracts and creates mock markets
 * 
 * Usage: 
 *   cd frontend && bun run ../script/deploy-tempo.ts
 * 
 * Or from project root with bun installed:
 *   bun run script/deploy-tempo.ts
 * 
 * Prerequisites:
 *   - PRIVATE_KEY set in .env file
 *   - bun installed (for TypeScript execution)
 *   - forge installed (for contract deployment)
 */

import { createClient, http, publicActions, walletActions } from 'viem'
import { privateKeyToAccount } from 'viem/accounts'
import { tempoTestnet } from 'viem/chains'
import { tempoActions, withFeePayer } from 'viem/tempo'
import { execSync } from 'child_process'

const PRIVATE_KEY = process.env.PRIVATE_KEY as `0x${string}`
const PATH_USD = '0x20C0000000000000000000000000000000000000' as const
const MIN_BALANCE = BigInt(1e8) // 100 PathUSD minimum

if (!PRIVATE_KEY) {
  console.error('Error: PRIVATE_KEY not set in .env')
  console.error('Create a .env file with: PRIVATE_KEY=0x...')
  process.exit(1)
}

async function main() {
  const account = privateKeyToAccount(PRIVATE_KEY)
  
  console.log('==========================================')
  console.log('Binary Protocol - Tempo Testnet Deployment')
  console.log('==========================================')
  console.log('')
  console.log(`Network:  Tempo Testnet (Chain ID: 42431)`)
  console.log(`Deployer: ${account.address}`)
  console.log('')

  // Create Tempo client with fee sponsorship for faucet calls
  const client = createClient({
    account,
    chain: tempoTestnet,
    transport: withFeePayer(
      http(),
      http('https://sponsor.moderato.tempo.xyz')
    ),
  })
    .extend(publicActions)
    .extend(walletActions)
    .extend(tempoActions())

  // Step 1: Check balance and fund if needed
  console.log('Step 1: Checking deployer balance...')
  const balance = await client.token.getBalance({
    token: PATH_USD,
    address: account.address,
  })
  console.log(`  PathUSD balance: ${Number(balance) / 1e6} PathUSD`)

  if (balance < MIN_BALANCE) {
    console.log('')
    console.log('Step 2: Funding deployer via faucet...')
    
    try {
      const hashes = await client.faucet.fund({
        account: account.address,
      })
      console.log(`  Funded! Transactions: ${hashes.length}`)
      
      // Wait for confirmation
      await new Promise(resolve => setTimeout(resolve, 2000))
      
      const newBalance = await client.token.getBalance({
        token: PATH_USD,
        address: account.address,
      })
      console.log(`  New balance: ${Number(newBalance) / 1e6} PathUSD`)
    } catch (error) {
      console.error('  Faucet failed:', error)
      console.log('')
      console.log('  Please fund manually: https://docs.tempo.xyz/quickstart/faucet')
      console.log(`  Address: ${account.address}`)
      process.exit(1)
    }
  } else {
    console.log('  Sufficient balance ✓')
  }

  // Step 2: Deploy contracts
  console.log('')
  console.log('Step 3: Deploying contracts...')
  console.log('')

  try {
    // Find the project root (where foundry.toml is)
    const projectRoot = process.cwd().includes('frontend')
      ? process.cwd().replace('/frontend', '')
      : process.cwd()

    execSync(
      `forge script script/DeployAndSeedMarkets.s.sol:DeployAndSeedMarkets \
        --rpc-url https://rpc.moderato.tempo.xyz \
        --broadcast \
        --private-key "${PRIVATE_KEY}" \
        -vvv`,
      {
        stdio: 'inherit',
        cwd: `${projectRoot}/contracts`,
      }
    )
    
    console.log('')
    console.log('==========================================')
    console.log('Deployment Complete!')
    console.log('==========================================')
    console.log('')
    console.log('View on explorer: https://explore.tempo.xyz')
    
  } catch (error) {
    console.error('')
    console.error('Deployment failed!')
    console.error(error)
    process.exit(1)
  }
}

main().catch(console.error)
