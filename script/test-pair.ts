#!/usr/bin/env bun
/**
 * Test script to create DEX pairs for verse tokens
 */

import { createWalletClient, http } from 'viem'
import { privateKeyToAccount } from 'viem/accounts'
import { tempoActions } from 'viem/tempo'
import { tempoModerato } from './lib/testnet'

const TESTNET_RPC_URL = 'https://rpc.moderato.tempo.xyz'
const PATH_USD = '0x20C0000000000000000000000000000000000000' as const

async function main() {
  const privateKey = process.env.PRIVATE_KEY as `0x${string}`
  if (!privateKey) {
    console.error('PRIVATE_KEY not set')
    process.exit(1)
  }

  const account = privateKeyToAccount(privateKey)
  console.log('Account:', account.address)

  const client = createWalletClient({
    account,
    chain: tempoModerato,
    transport: http(TESTNET_RPC_URL),
  }).extend(tempoActions())

  // Get verse tokens from the recent market
  // These are from the market: 0xaf196b8f5b5429b0...
  // Note: Use lowercase addresses to avoid checksum issues
  const verseTokens = [
    '0x20c000000000000000000000d55933498b485e9e', // YES (pathUSD)
    '0x20c000000000000000000000492217e0da72e51b', // NO (pathUSD)
    '0x20c000000000000000000000e21e79cbce7284f6', // YES (alphaUSD)
    '0x20c00000000000000000000085cb84688afeb07d', // NO (alphaUSD)
  ]

  for (const verse of verseTokens) {
    console.log(`\nCreating pair for: ${verse}`)
    try {
      const result = await client.dex.createPair({
        base: verse as `0x${string}`,
        feeToken: PATH_USD,
      })
      console.log('  Created pair, tx:', result)
    } catch (e: any) {
      if (e.message?.includes('PairAlreadyExists')) {
        console.log('  Pair already exists')
      } else {
        console.error('  Error:', e.message?.slice(0, 150))
      }
    }
  }
}

main().catch(console.error)
