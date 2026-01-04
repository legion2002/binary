#!/usr/bin/env npx tsx
/**
 * Fund deployer address via Tempo faucet and deploy contracts
 * 
 * Usage: npx tsx script/fund-and-deploy.ts
 */

import { createClient, http, publicActions, walletActions } from 'viem'
import { privateKeyToAccount } from 'viem/accounts'
import { tempoTestnet } from 'viem/chains'
import { tempoActions } from 'viem/tempo'
import { execSync } from 'child_process'
import * as dotenv from 'dotenv'

dotenv.config()

const PRIVATE_KEY = process.env.PRIVATE_KEY as `0x${string}`

if (!PRIVATE_KEY) {
  console.error('Error: PRIVATE_KEY not set in .env')
  process.exit(1)
}

async function main() {
  const account = privateKeyToAccount(PRIVATE_KEY)
  
  console.log('==========================================')
  console.log('Binary Prediction Market - Tempo Deployment')
  console.log('==========================================')
  console.log(`Deployer address: ${account.address}`)
  console.log('')

  // Create Tempo client
  const client = createClient({
    account,
    chain: tempoTestnet,
    transport: http(),
  })
    .extend(publicActions)
    .extend(walletActions)
    .extend(tempoActions())

  // Check current balance
  console.log('Checking current PathUSD balance...')
  const pathUSD = '0x20C0000000000000000000000000000000000000' as const
  
  try {
    const balance = await client.token.getBalance({
      token: pathUSD,
      address: account.address,
    })
    console.log(`Current PathUSD balance: ${balance / BigInt(1e6)} PathUSD`)

    if (balance < BigInt(1e6)) { // Less than 1 PathUSD
      console.log('')
      console.log('Funding deployer via faucet...')
      
      const receipts = await client.faucet.fundSync({
        account: account.address,
      })
      
      console.log(`Funded! Transaction hashes:`)
      receipts.forEach((r, i) => console.log(`  ${i + 1}. ${r.transactionHash}`))
      
      // Check new balance
      const newBalance = await client.token.getBalance({
        token: pathUSD,
        address: account.address,
      })
      console.log(`New PathUSD balance: ${newBalance / BigInt(1e6)} PathUSD`)
    } else {
      console.log('Sufficient balance, skipping faucet.')
    }
  } catch (error) {
    console.error('Error checking/funding balance:', error)
    console.log('')
    console.log('Attempting deployment anyway...')
  }

  console.log('')
  console.log('Deploying contracts...')
  console.log('')

  // Run forge deployment
  try {
    execSync(
      `forge script script/DeployAndSeedMarkets.s.sol:DeployAndSeedMarkets \
        --rpc-url https://rpc.testnet.tempo.xyz \
        --broadcast \
        --private-key "${PRIVATE_KEY}" \
        -vvv`,
      { 
        stdio: 'inherit',
        cwd: process.cwd(),
      }
    )
    
    console.log('')
    console.log('==========================================')
    console.log('Deployment complete!')
    console.log('==========================================')
  } catch (error) {
    console.error('Deployment failed:', error)
    process.exit(1)
  }
}

main().catch(console.error)
