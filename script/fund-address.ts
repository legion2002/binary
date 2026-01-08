#!/usr/bin/env bun
/**
 * Fund an address with AlphaUSD via the testnet faucet
 */

import { createClient, http, publicActions, walletActions, encodeFunctionData } from 'viem'
import { privateKeyToAccount } from 'viem/accounts'
import { tempoTestnet } from 'viem/chains'
import { tempoActions, withFeePayer } from 'viem/tempo'

const PRIVATE_KEY = '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80' as const
const ALPHA_USD = '0x20C0000000000000000000000000000000000001' as const
const TARGET_ADDRESS = '0x3E6F4c045D62107784B13C3D0bA7Ab09F129824E' as const
const AMOUNT = BigInt(1000) * BigInt(1e6) // 1000 AlphaUSD (6 decimals)

async function main() {
  const account = privateKeyToAccount(PRIVATE_KEY)
  
  console.log(`Sender: ${account.address}`)
  console.log(`Target: ${TARGET_ADDRESS}`)
  console.log(`Amount: ${Number(AMOUNT) / 1e6} AlphaUSD`)
  console.log('')

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

  // First fund our account via faucet
  console.log('Step 1: Funding sender via faucet...')
  try {
    const hashes = await client.faucet.fund({
      account: account.address,
    })
    console.log(`  Funded! ${hashes.length} transactions`)
    await new Promise(r => setTimeout(r, 2000))
  } catch (e) {
    console.log('  Faucet already used or failed, continuing...')
  }

  // Check balance
  const balance = await client.token.getBalance({
    token: ALPHA_USD,
    address: account.address,
  })
  console.log(`  AlphaUSD balance: ${Number(balance) / 1e6}`)

  // Transfer to target
  console.log('')
  console.log('Step 2: Transferring to target...')
  
  const hash = await client.sendTransaction({
    to: ALPHA_USD,
    data: encodeFunctionData({
      abi: [{
        type: 'function',
        name: 'transfer',
        inputs: [
          { name: 'to', type: 'address' },
          { name: 'amount', type: 'uint256' },
        ],
        outputs: [{ type: 'bool' }],
        stateMutability: 'nonpayable',
      }],
      functionName: 'transfer',
      args: [TARGET_ADDRESS, AMOUNT],
    }),
  })
  
  console.log(`  Transaction: ${hash}`)
  console.log(`  View: https://explore.tempo.xyz/tx/${hash}`)
  
  // Wait and check target balance
  await new Promise(r => setTimeout(r, 2000))
  const targetBalance = await client.token.getBalance({
    token: ALPHA_USD,
    address: TARGET_ADDRESS,
  })
  console.log(`  Target balance: ${Number(targetBalance) / 1e6} AlphaUSD`)
}

main().catch(console.error)
