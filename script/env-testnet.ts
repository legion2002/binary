#!/usr/bin/env bun
/**
 * Binary Protocol - Tempo Testnet Environment Orchestrator
 *
 * Deploys and runs the full stack on Tempo testnet with fault tolerance.
 * The script is idempotent - safe to run multiple times.
 *
 * Features:
 * - Funds deployer via faucet if balance is low
 * - Checks if contracts are already deployed before deploying
 * - Persists contract addresses to .testnet-config.json
 * - Uses persistent SQLite database for backend
 * - Idempotent market seeding (duplicates are ignored)
 * - DEX pair creation is idempotent
 *
 * Usage:
 *   bun run script/env-testnet.ts              # Full deploy + dev mode
 *   bun run script/env-testnet.ts --skip-frontend  # Backend only
 *   bun run script/env-testnet.ts --redeploy   # Force redeploy contracts
 *
 * Environment:
 *   PRIVATE_KEY - Required. Deployer private key (will be funded via faucet if needed)
 *   ADMIN_API_KEY - Optional. Defaults to test key for dev
 *   DATABASE_URL - Optional. Defaults to sqlite:./testnet.db?mode=rwc
 */

import { spawn } from 'child_process'
import { join } from 'path'
import { tempoModerato, TESTNET_CHAIN_ID } from './lib/testnet'
import {
  getTestnetClient,
  ensureFunded,
  checkContractsDeployed,
  loadTestnetConfig,
  saveTestnetConfig,
  TESTNET_RPC_URL,
  type TestnetClient,
} from './lib/testnet'
import { deployContractsTestnet, getProjectRoot, type ContractAddresses } from './lib/deploy'
import { TEST_API_KEY, TEST_API_KEY_HASH, type BackendProcess } from './lib/backend'
import { startFrontend, type FrontendProcess } from './lib/frontend'
import { createSeedMarkets } from './lib/seed-markets'
import { waitForHttp, clearLine } from './lib/health'
import {
  seedMarketLiquidity,
  DEFAULT_SEED_CONFIG,
  ensureStablecoinRouting,
} from './lib/seed-liquidity'
import type { Address } from 'viem'

// Colors for console output
const cyan = (s: string) => `\x1b[36m${s}\x1b[0m`
const green = (s: string) => `\x1b[32m${s}\x1b[0m`
const yellow = (s: string) => `\x1b[33m${s}\x1b[0m`
const red = (s: string) => `\x1b[31m${s}\x1b[0m`
const dim = (s: string) => `\x1b[2m${s}\x1b[0m`

interface Environment {
  client?: TestnetClient
  contracts?: ContractAddresses
  backend?: BackendProcess
  frontend?: FrontendProcess
}

interface Args {
  skipFrontend: boolean
  redeploy: boolean
}

const env: Environment = {}

function parseArgs(): Args {
  const args = process.argv.slice(2)
  return {
    skipFrontend: args.includes('--skip-frontend') || args.includes('--backend-only'),
    redeploy: args.includes('--redeploy') || args.includes('--force'),
  }
}

function getPrivateKey(): `0x${string}` {
  const pk = process.env.PRIVATE_KEY
  if (!pk) {
    console.error(red('Error: PRIVATE_KEY environment variable not set'))
    console.error('')
    console.error('Create a .env file with:')
    console.error('  PRIVATE_KEY=0x...')
    console.error('')
    console.error('Or export it:')
    console.error('  export PRIVATE_KEY=0x...')
    process.exit(1)
  }
  return pk.startsWith('0x') ? (pk as `0x${string}`) : (`0x${pk}` as `0x${string}`)
}

async function shutdown(): Promise<void> {
  console.log('')
  console.log(yellow('Shutting down...'))

  if (env.frontend) {
    console.log('  Stopping frontend...')
    await env.frontend.stop()
  }

  if (env.backend) {
    console.log('  Stopping backend...')
    await env.backend.stop()
  }

  console.log(green('  All services stopped'))
}

function printSummary(): void {
  const account = env.client?.account
  console.log('')
  console.log(cyan(''))
  console.log(cyan('  Binary Protocol - Tempo Testnet Environment Ready'))
  console.log(cyan(''))
  console.log('')
  console.log(green('  Network:'))
  console.log(`    Chain:      Tempo Testnet Moderato (Chain ID: ${TESTNET_CHAIN_ID})`)
  console.log(`    RPC:        ${TESTNET_RPC_URL}`)
  console.log('')
  console.log(green('  Contracts:'))
  console.log(`    MultiVerse: ${env.contracts?.multiverse}`)
  console.log(`    Oracle:     ${env.contracts?.oracle}`)
  console.log('')
  console.log(green('  Services:'))
  console.log(`    Backend:    ${env.backend?.url}`)
  if (env.frontend) {
    console.log(`    Frontend:   ${env.frontend.url}`)
  }
  console.log('')
  console.log(green('  Account:'))
  console.log(`    Address:    ${account?.address}`)
  console.log('')
  console.log(green('  Admin API:'))
  console.log(`    API Key:    ${TEST_API_KEY}`)
  console.log('')
  console.log(dim('  Contract addresses saved to .testnet-config.json'))
  console.log(dim('  Database persisted at testnet.db'))
  console.log('')
  console.log(yellow('  Press Ctrl+C to stop all services'))
  console.log('')
}

async function main(): Promise<void> {
  const args = parseArgs()
  const projectRoot = getProjectRoot()
  const privateKey = getPrivateKey()
  const verbose = process.env.VERBOSE === '1'

  console.log('')
  console.log(cyan(''))
  console.log(cyan('  Binary Protocol - Tempo Testnet Deployment'))
  console.log(cyan(''))
  console.log('')

  // Register cleanup handlers
  const cleanup = async () => {
    await shutdown()
    process.exit(0)
  }
  process.on('SIGINT', cleanup)
  process.on('SIGTERM', cleanup)

  try {
    // Step 1: Create client and check balance
    console.log(cyan('Step 1:') + ' Initializing client and checking balance...')
    env.client = getTestnetClient(privateKey)
    const account = env.client.account
    console.log(`  Deployer address: ${account.address}`)

    const { funded, balance } = await ensureFunded(env.client)
    if (funded) {
      console.log(green('  Funded via faucet'))
    } else {
      console.log(green(`  Balance OK: ${Number(balance) / 1e6} PathUSD`))
    }

    // Step 2: Check if contracts are deployed
    console.log('')
    console.log(cyan('Step 2:') + ' Checking contract deployment status...')
    let config = loadTestnetConfig()
    const contractsExist = await checkContractsDeployed(env.client, config)

    if (contractsExist && !args.redeploy) {
      console.log(green('  Contracts already deployed'))
      console.log(`    MultiVerse: ${config.multiverse}`)
      console.log(`    Oracle:     ${config.oracle}`)
      env.contracts = {
        multiverse: config.multiverse!,
        oracle: config.oracle!,
      }
    } else {
      // Step 3: Deploy contracts
      console.log('')
      console.log(cyan('Step 3:') + ' Deploying contracts...')

      // Clear old database when redeploying (markets are tied to contract addresses)
      const dbPath = join(projectRoot, 'backend', 'testnet.db')
      try {
        const fs = await import('fs')
        if (fs.existsSync(dbPath)) {
          fs.unlinkSync(dbPath)
          console.log(dim('  Cleared old database'))
        }
      } catch {
        // Ignore if file doesn't exist
      }

      env.contracts = await deployContractsTestnet({
        privateKey,
        projectRoot,
        verbose,
      })

      // Save addresses for future runs
      config = {
        multiverse: env.contracts.multiverse,
        oracle: env.contracts.oracle,
        deployedAt: new Date().toISOString(),
        deployerAddress: account.address,
      }
      saveTestnetConfig(config)

      console.log(green('  Contracts deployed'))
      console.log(`    MultiVerse: ${env.contracts.multiverse}`)
      console.log(`    Oracle:     ${env.contracts.oracle}`)
    }

    // Step 4: Start backend
    console.log('')
    console.log(cyan('Step 4:') + ' Starting backend server...')

    // Use persistent database for testnet
    process.env.DATABASE_URL = process.env.DATABASE_URL || 'sqlite:./testnet.db?mode=rwc'

    env.backend = await startBackendTestnet({
      contracts: env.contracts,
      projectRoot,
      verbose,
    })
    console.log(green('  Backend running at ' + env.backend.url))

    // Step 5: Seed markets (idempotent - duplicates ignored)
    console.log('')
    console.log(cyan('Step 5:') + ' Seeding markets (idempotent)...')
    const seedResult = await createSeedMarkets({
      backendUrl: env.backend.url,
      oracleAddress: env.contracts.oracle,
      verbose: true,
    })
    if (seedResult.count > 0) {
      console.log(green(`  Created ${seedResult.count} new markets`))
    } else if (seedResult.markets.length === 0) {
      console.log(yellow('  No markets created (check backend logs for errors)'))
    } else {
      console.log(green('  Markets already exist'))
    }

    // Step 6: Ensure stablecoin routing liquidity
    console.log('')
    console.log(cyan('Step 6:') + ' Setting up stablecoin routing...')
    try {
      await ensureStablecoinRouting(env.client)
      console.log(green('  Stablecoin routing configured'))
    } catch (e: any) {
      console.log(yellow(`  Warning: ${e.message?.slice(0, 50)}...`))
    }

    // Step 7: Seed market liquidity
    console.log('')
    console.log(cyan('Step 7:') + ' Seeding market liquidity...')
    let liquiditySeeded = 0

    // Get existing markets from backend
    const marketsRes = await fetch(`${env.backend.url}/markets`)
    const marketsData = (await marketsRes.json()) as { markets: Array<{ marketHash: string }> }

    for (const market of marketsData.markets || []) {
      // Get verse tokens for this market
      const verseRes = await fetch(`${env.backend.url}/markets/${market.marketHash}/verse-tokens`)
      // API returns array directly, not wrapped in { verseTokens: [...] }
      const verseTokens = (await verseRes.json()) as Array<{
        asset: string
        yesVerse: string
        noVerse: string
      }>

      for (const vt of verseTokens || []) {
        try {
          console.log(`  Seeding ${market.marketHash.slice(0, 10)}... with ${vt.asset.slice(0, 10)}...`)
          await seedMarketLiquidity(
            env.client,
            {
              marketHash: market.marketHash as `0x${string}`,
              multiVerseAddress: env.contracts.multiverse as Address,
              quoteToken: vt.asset as Address,
            },
            DEFAULT_SEED_CONFIG
          )
          liquiditySeeded++
        } catch (e: any) {
          // Log errors for debugging
          console.warn(
            `  Warning: ${market.marketHash.slice(0, 10)}... / ${vt.asset.slice(0, 10)}...: ${e.message?.slice(0, 80)}`
          )
        }
      }
    }
    console.log(green(`  Seeded liquidity for ${liquiditySeeded} market/asset pairs`))

    // Step 8: Start frontend (unless skipped)
    if (!args.skipFrontend) {
      console.log('')
      console.log(cyan('Step 8:') + ' Starting frontend dev server...')
      env.frontend = await startFrontend({
        projectRoot,
        rpcUrl: TESTNET_RPC_URL,
        verbose,
      })
      console.log(green('  Frontend running at ' + env.frontend.url))
    }

    printSummary()

    // Keep running until interrupted
    await new Promise(() => {})
  } catch (error) {
    console.error('')
    console.error(red('Error: ' + (error instanceof Error ? error.message : String(error))))
    if (verbose && error instanceof Error) {
      console.error(error.stack)
    }
    await shutdown()
    process.exit(1)
  }
}

/**
 * Start backend configured for testnet
 * Uses persistent database and testnet RPC
 */
async function startBackendTestnet(options: {
  contracts: ContractAddresses
  projectRoot: string
  verbose?: boolean
}): Promise<BackendProcess> {
  const { contracts, projectRoot, verbose = false } = options

  const port = Number(process.env.PORT) || 3000
  const url = `http://127.0.0.1:${port}`
  const backendDir = join(projectRoot, 'backend')

  const envVars: Record<string, string | undefined> = {
    ...process.env,
    HOST: '127.0.0.1',
    PORT: String(port),
    MULTIVERSE_ADDRESS: contracts.multiverse,
    ORACLE_ADDRESS: contracts.oracle,
    RPC_URL: TESTNET_RPC_URL,
    PRIVATE_KEY: process.env.PRIVATE_KEY,
    DATABASE_URL: process.env.DATABASE_URL || 'sqlite:./testnet.db?mode=rwc',
    ADMIN_API_KEY_HASH: process.env.ADMIN_API_KEY_HASH || TEST_API_KEY_HASH,
    RUST_LOG: process.env.RUST_LOG ?? 'info,backend=debug',
  }

  const child = spawn('cargo', ['run', '--bin', 'backend'], {
    cwd: backendDir,
    env: envVars,
    stdio: verbose ? 'inherit' : 'pipe',
  })

  child.on('error', (err: Error) => {
    console.error('Backend process error:', err)
  })

  if (!verbose && child.stderr) {
    child.stderr.on('data', (data: Buffer) => {
      const msg = data.toString()
      if (msg.toLowerCase().includes('error')) {
        console.error(msg)
      }
    })
  }

  // Wait for backend to be ready
  await waitForHttp(`${url}/markets`, { timeoutMs: 120_000, retryMs: 2_000 })
  clearLine()

  return {
    process: child,
    url,
    port,
    apiKey: TEST_API_KEY,
    async stop() {
      return new Promise<void>((resolve) => {
        if (child.killed) {
          resolve()
          return
        }
        child.on('exit', () => resolve())
        child.kill('SIGTERM')
        setTimeout(() => {
          if (!child.killed) {
            child.kill('SIGKILL')
          }
          resolve()
        }, 5000)
      })
    },
  }
}

main().catch(async (error) => {
  console.error(red('Fatal error:'), error)
  await shutdown()
  process.exit(1)
})
