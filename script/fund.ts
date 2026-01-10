#!/usr/bin/env bun
/**
 * Fund an address with stablecoins on local devnet or testnet
 * 
 * Usage:
 *   bun run fund <address> [amount] [--token <token>] [--chain <chain-id>] [--faucet]
 * 
 * Examples:
 *   bun run fund 0x3E6F4c045D62107784B13C3D0bA7Ab09F129824E
 *   bun run fund 0x3E6F4c045D62107784B13C3D0bA7Ab09F129824E 5000
 *   bun run fund 0x3E6F4c045D62107784B13C3D0bA7Ab09F129824E 1000 --token path
 *   bun run fund 0x3E6F4c045D62107784B13C3D0bA7Ab09F129824E --chain 42431 1000 --token alpha
 *   bun run fund 0x3E6F4c045D62107784B13C3D0bA7Ab09F129824E --chain 42431 --faucet  # Use faucet
 */

import { createClient, http, publicActions, walletActions, encodeFunctionData, type Chain } from 'viem'
import { privateKeyToAccount } from 'viem/accounts'
import { tempoActions, withFeePayer } from 'viem/tempo'

const TEST_PRIVATE_KEY = '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80' as const

const TOKENS = {
  alpha: { address: '0x20C0000000000000000000000000000000000001' as const, name: 'AlphaUSD' },
  path: { address: '0x20C0000000000000000000000000000000000000' as const, name: 'PathUSD' },
  beta: { address: '0x20C0000000000000000000000000000000000002' as const, name: 'BetaUSD' },
  theta: { address: '0x20C0000000000000000000000000000000000003' as const, name: 'ThetaUSD' },
} as const

const PATH_USD = TOKENS.path.address

// Chain configurations
const TESTNET_CHAIN_ID = 42431
const LOCAL_CHAIN_ID = 1337

const TESTNET_RPC_URL = 'https://rpc.moderato.tempo.xyz'
const TESTNET_FEE_SPONSOR_URL = 'https://sponsor.moderato.tempo.xyz'
const LOCAL_RPC_URL = process.env.RPC_URL || 'http://localhost:9545'

const localDevnet = {
  id: LOCAL_CHAIN_ID,
  name: 'Tempo Local',
  nativeCurrency: { name: 'USD', symbol: 'USD', decimals: 6 },
  rpcUrls: { default: { http: [LOCAL_RPC_URL] } },
} as const satisfies Chain

const tempoModerato = {
  id: TESTNET_CHAIN_ID,
  name: 'Tempo Testnet (Moderato)',
  nativeCurrency: { name: 'USD', symbol: 'USD', decimals: 6 },
  rpcUrls: { 
    default: { 
      http: [TESTNET_RPC_URL],
      webSocket: ['wss://rpc.moderato.tempo.xyz'],
    } 
  },
  blockExplorers: {
    default: {
      name: 'Tempo Explorer',
      url: 'https://explore.moderato.tempo.xyz',
    },
  },
} as const satisfies Chain

function parseArgs(args: string[]) {
  let address: string | undefined
  let amount = '1000'
  let tokenKey: keyof typeof TOKENS = 'alpha'
  let chainId: number = LOCAL_CHAIN_ID
  let useFaucet = false

  for (let i = 0; i < args.length; i++) {
    if (args[i] === '--token' || args[i] === '-t') {
      const tokenArg = args[++i]?.toLowerCase()
      if (tokenArg && tokenArg in TOKENS) {
        tokenKey = tokenArg as keyof typeof TOKENS
      } else {
        console.error(`Error: Invalid token. Choose from: ${Object.keys(TOKENS).join(', ')}`)
        process.exit(1)
      }
    } else if (args[i] === '--chain' || args[i] === '-c') {
      const chainArg = args[++i]
      if (chainArg) {
        chainId = parseInt(chainArg, 10)
        if (isNaN(chainId)) {
          console.error(`Error: Invalid chain ID: ${chainArg}`)
          process.exit(1)
        }
      }
    } else if (args[i] === '--faucet' || args[i] === '-f') {
      useFaucet = true
    } else if (args[i].startsWith('0x')) {
      address = args[i]
    } else if (/^\d+$/.test(args[i])) {
      amount = args[i]
    }
  }

  return { address, amount, tokenKey, chainId, useFaucet }
}

async function fundViaFaucet(targetAddress: string) {
  console.log('Using testnet faucet to fund address...')
  console.log(`Target: ${targetAddress}`)
  console.log(`RPC: ${TESTNET_RPC_URL}`)
  console.log('')

  const privateKey = process.env.PRIVATE_KEY as `0x${string}` | undefined
  
  if (!privateKey) {
    console.error('Error: PRIVATE_KEY environment variable required for testnet')
    process.exit(1)
  }

  const account = privateKeyToAccount(privateKey)
  
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

  try {
    const hashes = await client.faucet.fund({
      account: targetAddress as `0x${string}`,
    })
    
    console.log(`✓ Faucet transactions: ${hashes.length}`)
    for (const hash of hashes) {
      console.log(`  ${hash}`)
    }

    await new Promise(resolve => setTimeout(resolve, 3000))

    console.log('')
    console.log('New balances:')
    for (const [, token] of Object.entries(TOKENS)) {
      try {
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
        console.log(`  ${token.name}: ${Number(balance) / 1e6}`)
      } catch {
        // Token might not exist
      }
    }
  } catch (err: any) {
    console.error('Faucet error:', err.message)
    process.exit(1)
  }
}

async function fundOnTestnet(targetAddress: string, tokenKey: keyof typeof TOKENS, amountStr: string) {
  const token = TOKENS[tokenKey]
  const amount = BigInt(amountStr) * BigInt(1e6) // 6 decimals

  console.log(`Funding on testnet...`)
  console.log(`Target: ${targetAddress}`)
  console.log(`Amount: ${Number(amount) / 1e6} ${token.name}`)
  console.log(`RPC: ${TESTNET_RPC_URL}`)
  console.log('')

  const privateKey = process.env.PRIVATE_KEY as `0x${string}` | undefined
  
  if (!privateKey) {
    console.error('Error: PRIVATE_KEY environment variable required for testnet')
    process.exit(1)
  }

  const account = privateKeyToAccount(privateKey)
  
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

  try {
    // Transfer tokens using tempo token action
    const hash = await client.token.transfer({
      token: token.address,
      to: targetAddress as `0x${string}`,
      amount,
    })

    console.log(`✓ Transaction: ${hash}`)

    // Wait for confirmation
    const receipt = await client.waitForTransactionReceipt({ hash })
    console.log(`✓ Confirmed in block ${receipt.blockNumber}`)

    // Check new balance
    const balance = await client.token.getBalance({
      token: token.address,
      address: targetAddress as `0x${string}`,
    })

    console.log(`✓ New balance: ${Number(balance) / 1e6} ${token.name}`)
  } catch (err: any) {
    console.error('Transfer error:', err.message)
    process.exit(1)
  }
}

async function fundOnLocal(targetAddress: string, tokenKey: keyof typeof TOKENS, amountStr: string) {
  const token = TOKENS[tokenKey]
  const amount = BigInt(amountStr) * BigInt(1e6) // 6 decimals
  const rpcUrl = LOCAL_RPC_URL

  console.log(`Funding on local devnet...`)
  console.log(`Target: ${targetAddress}`)
  console.log(`Amount: ${Number(amount) / 1e6} ${token.name}`)
  console.log(`RPC: ${rpcUrl}`)
  console.log('')

  const account = privateKeyToAccount(TEST_PRIVATE_KEY)

  const client = createClient({
    account,
    chain: localDevnet,
    transport: http(rpcUrl),
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

async function main() {
  const args = process.argv.slice(2)
  
  if (args.length === 0 || args[0] === '--help' || args[0] === '-h') {
    console.log('Usage: bun run fund <address> [amount] [--token <token>] [--chain <chain-id>] [--faucet]')
    console.log('')
    console.log('Arguments:')
    console.log('  address        The address to fund (required)')
    console.log('  amount         Amount to send (default: 1000)')
    console.log('  --token, -t    Token to send: alpha, path, beta, theta (default: alpha)')
    console.log('  --chain, -c    Chain ID: 1337 (local) or 42431 (testnet) (default: 1337)')
    console.log('  --faucet, -f   Use testnet faucet (funds all tokens, ignores amount/token)')
    console.log('')
    console.log('Chains:')
    console.log(`  ${LOCAL_CHAIN_ID}        Local devnet (transfers from test account)`)
    console.log(`  ${TESTNET_CHAIN_ID}       Tempo Moderato testnet (requires PRIVATE_KEY)`)
    console.log('')
    console.log('Tokens:')
    for (const [key, token] of Object.entries(TOKENS)) {
      console.log(`  ${key.padEnd(8)} ${token.name} (${token.address})`)
    }
    console.log('')
    console.log('Examples:')
    console.log('  bun run fund 0x3E6F...824E                           # 1000 AlphaUSD on local')
    console.log('  bun run fund 0x3E6F...824E 5000                      # 5000 AlphaUSD on local')
    console.log('  bun run fund 0x3E6F...824E 1000 --token path         # 1000 PathUSD on local')
    console.log('  bun run fund 0x3E6F...824E --chain 42431 1000        # 1000 AlphaUSD on testnet')
    console.log('  bun run fund 0x3E6F...824E --chain 42431 --faucet    # Use faucet on testnet')
    process.exit(0)
  }

  const { address: targetAddress, amount: amountStr, tokenKey, chainId, useFaucet } = parseArgs(args)

  if (!targetAddress || !targetAddress.startsWith('0x') || targetAddress.length !== 42) {
    console.error('Error: Invalid or missing address')
    process.exit(1)
  }

  if (chainId === TESTNET_CHAIN_ID) {
    if (useFaucet) {
      await fundViaFaucet(targetAddress)
    } else {
      await fundOnTestnet(targetAddress, tokenKey, amountStr)
    }
  } else if (chainId === LOCAL_CHAIN_ID) {
    if (useFaucet) {
      console.error('Error: --faucet is only available on testnet (--chain 42431)')
      process.exit(1)
    }
    await fundOnLocal(targetAddress, tokenKey, amountStr)
  } else {
    console.error(`Error: Unsupported chain ID: ${chainId}`)
    console.error(`Supported chains: ${LOCAL_CHAIN_ID} (local), ${TESTNET_CHAIN_ID} (testnet)`)
    process.exit(1)
  }
}

main().catch((err) => {
  console.error('Error:', err.message)
  process.exit(1)
})
