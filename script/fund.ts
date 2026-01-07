#!/usr/bin/env bun
/**
 * Fund an address with stablecoins on the local devnet
 * 
 * Usage:
 *   bun run fund <address> [amount] [--token <token>]
 * 
 * Examples:
 *   bun run fund 0x3E6F4c045D62107784B13C3D0bA7Ab09F129824E
 *   bun run fund 0x3E6F4c045D62107784B13C3D0bA7Ab09F129824E 5000
 *   bun run fund 0x3E6F4c045D62107784B13C3D0bA7Ab09F129824E 1000 --token path
 */

import { createClient, http, publicActions, walletActions, encodeFunctionData } from 'viem'
import { privateKeyToAccount } from 'viem/accounts'

const TEST_PRIVATE_KEY = '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80' as const

const TOKENS = {
  alpha: { address: '0x20C0000000000000000000000000000000000001' as const, name: 'AlphaUSD' },
  path: { address: '0x20C0000000000000000000000000000000000000' as const, name: 'PathUSD' },
  beta: { address: '0x20C0000000000000000000000000000000000002' as const, name: 'BetaUSD' },
  theta: { address: '0x20C0000000000000000000000000000000000003' as const, name: 'ThetaUSD' },
} as const

const PATH_USD = TOKENS.path.address
const RPC_URL = process.env.RPC_URL || 'http://localhost:9545'

const localDevnet = {
  id: 1337,
  name: 'Tempo Local',
  nativeCurrency: { name: 'Ether', symbol: 'ETH', decimals: 18 },
  rpcUrls: { default: { http: [RPC_URL] } },
} as const

function parseArgs(args: string[]) {
  let address: string | undefined
  let amount = '1000'
  let tokenKey: keyof typeof TOKENS = 'alpha'

  for (let i = 0; i < args.length; i++) {
    if (args[i] === '--token' || args[i] === '-t') {
      const tokenArg = args[++i]?.toLowerCase()
      if (tokenArg && tokenArg in TOKENS) {
        tokenKey = tokenArg as keyof typeof TOKENS
      } else {
        console.error(`Error: Invalid token. Choose from: ${Object.keys(TOKENS).join(', ')}`)
        process.exit(1)
      }
    } else if (args[i].startsWith('0x')) {
      address = args[i]
    } else if (/^\d+$/.test(args[i])) {
      amount = args[i]
    }
  }

  return { address, amount, tokenKey }
}

async function main() {
  const args = process.argv.slice(2)
  
  if (args.length === 0 || args[0] === '--help' || args[0] === '-h') {
    console.log('Usage: bun run fund <address> [amount] [--token <token>]')
    console.log('')
    console.log('Arguments:')
    console.log('  address        The address to fund (required)')
    console.log('  amount         Amount to send (default: 1000)')
    console.log('  --token, -t    Token to send: alpha, path, beta, theta (default: alpha)')
    console.log('')
    console.log('Tokens:')
    for (const [key, token] of Object.entries(TOKENS)) {
      console.log(`  ${key.padEnd(8)} ${token.name} (${token.address})`)
    }
    console.log('')
    console.log('Examples:')
    console.log('  bun run fund 0x3E6F...824E                    # 1000 AlphaUSD')
    console.log('  bun run fund 0x3E6F...824E 5000               # 5000 AlphaUSD')
    console.log('  bun run fund 0x3E6F...824E 1000 --token path  # 1000 PathUSD')
    console.log('  bun run fund 0x3E6F...824E -t beta            # 1000 BetaUSD')
    process.exit(0)
  }

  const { address: targetAddress, amount: amountStr, tokenKey } = parseArgs(args)

  if (!targetAddress || !targetAddress.startsWith('0x') || targetAddress.length !== 42) {
    console.error('Error: Invalid or missing address')
    process.exit(1)
  }

  const token = TOKENS[tokenKey]
  const amount = BigInt(amountStr) * BigInt(1e6) // 6 decimals

  const account = privateKeyToAccount(TEST_PRIVATE_KEY)
  
  console.log(`Funding ${targetAddress}`)
  console.log(`Amount: ${Number(amount) / 1e6} ${token.name}`)
  console.log(`RPC: ${RPC_URL}`)
  console.log('')

  const client = createClient({
    account,
    chain: localDevnet,
    transport: http(RPC_URL),
  })
    .extend(publicActions)
    .extend(walletActions)

  // Transfer tokens
  const hash = await client.sendTransaction({
    to: token.address,
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
      args: [targetAddress as `0x${string}`, amount],
    }),
    feeToken: PATH_USD,
  } as Parameters<typeof client.sendTransaction>[0])

  console.log(`✓ Transaction: ${hash}`)

  // Wait for confirmation
  const receipt = await client.waitForTransactionReceipt({ hash })
  console.log(`✓ Confirmed in block ${receipt.blockNumber}`)

  // Check new balance
  const balance = await client.readContract({
    address: token.address,
    abi: [{
      type: 'function',
      name: 'balanceOf',
      inputs: [{ name: 'account', type: 'address' }],
      outputs: [{ type: 'uint256' }],
      stateMutability: 'view',
    }],
    functionName: 'balanceOf',
    args: [targetAddress as `0x${string}`],
  })

  console.log(`✓ New balance: ${Number(balance) / 1e6} ${token.name}`)
}

main().catch((err) => {
  console.error('Error:', err.message)
  process.exit(1)
})
