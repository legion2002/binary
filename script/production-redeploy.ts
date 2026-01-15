#!/usr/bin/env bun
/**
 * Production Redeploy - One-click production environment setup
 *
 * This script automates the full production redeployment:
 * 1. Deploys new contracts to Tempo testnet
 * 2. Updates Fly.io secrets with new contract addresses
 * 3. Clears the production database
 * 4. Restarts the backend
 * 5. Seeds markets and liquidity
 * 6. Updates deploy.yml with new UniV2 addresses
 * 7. Optionally commits and pushes changes
 *
 * Usage:
 *   PRIVATE_KEY=0x... bun run production:redeploy
 *   PRIVATE_KEY=0x... bun run production:redeploy --no-commit
 *   PRIVATE_KEY=0x... bun run production:redeploy --skip-liquidity
 *
 * Environment:
 *   PRIVATE_KEY - Required. Deployer private key (will be funded via faucet if needed)
 *   FLY_API_TOKEN - Optional. If not set, uses flyctl auth
 */

import { execSync, spawnSync } from 'child_process'
import { readFileSync, writeFileSync } from 'fs'
import { join } from 'path'
import { getTestnetClient, ensureFunded, TESTNET_RPC_URL, TESTNET_CHAIN_ID } from './lib/testnet'
import { deployContractsTestnet, getProjectRoot } from './lib/deploy'
import { createSeedMarkets } from './lib/seed-markets'
import { seedMarketLiquidity, DEFAULT_SEED_CONFIG, ensureStablecoinRouting } from './lib/seed-liquidity'
import { initUniV2Config } from './lib/univ2'
import { TEST_API_KEY } from './lib/backend'
import type { Address } from 'viem'

const cyan = (s: string) => `\x1b[36m${s}\x1b[0m`
const green = (s: string) => `\x1b[32m${s}\x1b[0m`
const yellow = (s: string) => `\x1b[33m${s}\x1b[0m`
const red = (s: string) => `\x1b[31m${s}\x1b[0m`
const dim = (s: string) => `\x1b[2m${s}\x1b[0m`

const BACKEND_URL = 'https://binary-markets-api.fly.dev'
const RATE_LIMIT_DELAY = 2000

interface Args {
  noCommit: boolean
  skipLiquidity: boolean
}

function parseArgs(): Args {
  const args = process.argv.slice(2)
  return {
    noCommit: args.includes('--no-commit'),
    skipLiquidity: args.includes('--skip-liquidity'),
  }
}

function getPrivateKey(): `0x${string}` {
  const pk = process.env.PRIVATE_KEY
  if (!pk) {
    console.error(red('Error: PRIVATE_KEY environment variable not set'))
    console.error('')
    console.error('Usage:')
    console.error('  PRIVATE_KEY=0x... bun run production:redeploy')
    process.exit(1)
  }
  return pk.startsWith('0x') ? (pk as `0x${string}`) : (`0x${pk}` as `0x${string}`)
}

function runCommand(cmd: string, options?: { cwd?: string; silent?: boolean }): string {
  const { cwd, silent = false } = options || {}
  try {
    const result = execSync(cmd, {
      cwd,
      encoding: 'utf-8',
      stdio: silent ? 'pipe' : ['pipe', 'pipe', 'pipe'],
    })
    return result.trim()
  } catch (e: any) {
    if (!silent) {
      console.error(red(`Command failed: ${cmd}`))
      console.error(e.stderr?.toString() || e.message)
    }
    throw e
  }
}

function checkFlyctl(): boolean {
  try {
    runCommand('flyctl version', { silent: true })
    return true
  } catch {
    return false
  }
}

async function sleep(ms: number): Promise<void> {
  return new Promise((r) => setTimeout(r, ms))
}

async function main(): Promise<void> {
  const args = parseArgs()
  const projectRoot = getProjectRoot()
  const privateKey = getPrivateKey()

  console.log('')
  console.log(cyan('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━'))
  console.log(cyan('  Binary Markets - Production Redeploy'))
  console.log(cyan('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━'))
  console.log('')

  // Check prerequisites
  console.log(cyan('Checking prerequisites...'))
  
  if (!checkFlyctl()) {
    console.error(red('  ✗ flyctl not found. Install from https://fly.io/docs/flyctl/install/'))
    process.exit(1)
  }
  console.log(green('  ✓ flyctl installed'))

  // Check flyctl auth
  try {
    runCommand('flyctl auth whoami', { cwd: join(projectRoot, 'backend'), silent: true })
    console.log(green('  ✓ flyctl authenticated'))
  } catch {
    console.error(red('  ✗ Not logged into flyctl. Run: flyctl auth login'))
    process.exit(1)
  }

  // Step 1: Initialize client and check balance
  console.log('')
  console.log(cyan('Step 1:') + ' Initializing client...')
  const client = getTestnetClient(privateKey)
  console.log(`  Deployer: ${client.account.address}`)

  const { funded, balance } = await ensureFunded(client)
  if (funded) {
    console.log(green('  Funded via faucet'))
  } else {
    console.log(green(`  Balance OK: ${Number(balance) / 1e6} PathUSD`))
  }

  // Step 2: Deploy contracts
  console.log('')
  console.log(cyan('Step 2:') + ' Deploying contracts to Tempo testnet...')
  
  const contracts = await deployContractsTestnet({
    privateKey,
    projectRoot,
    verbose: true,
  })

  console.log(green('  ✓ Contracts deployed'))
  console.log(`    MultiVerse: ${contracts.multiverse}`)
  console.log(`    Oracle:     ${contracts.oracle}`)
  console.log(`    UniV2 Factory: ${contracts.uniV2Factory}`)
  console.log(`    UniV2 Router:  ${contracts.uniV2Router}`)

  // Initialize UniV2 config
  if (contracts.uniV2Factory && contracts.uniV2Router) {
    initUniV2Config(
      contracts.uniV2Factory as Address,
      contracts.uniV2Router as Address,
      TESTNET_CHAIN_ID
    )
  }

  // Step 3: Update deployments.json and redeploy backend
  console.log('')
  console.log(cyan('Step 3:') + ' Updating deployments.json...')
  
  runCommand('bun run update-deployments', { cwd: projectRoot })
  console.log(green('  ✓ deployments.json updated'))
  
  // Deploy backend with new deployments.json
  console.log('')
  console.log(cyan('Step 3b:') + ' Deploying backend to Fly.io...')
  runCommand('fly deploy --now', { cwd: join(projectRoot, 'backend') })
  console.log(green('  ✓ Backend deployed'))

  // Step 4: Clear database
  console.log('')
  console.log(cyan('Step 4:') + ' Clearing production database...')
  
  try {
    runCommand('flyctl ssh console -C "rm -f /data/markets.db /data/markets.db-shm /data/markets.db-wal"', {
      cwd: join(projectRoot, 'backend'),
      silent: true,
    })
    console.log(green('  ✓ Database cleared'))
  } catch {
    console.log(yellow('  ⚠ Could not clear database (machine may be stopped)'))
  }

  // Step 5: Wait for backend to be ready
  console.log('  Waiting for backend to be healthy...')
  let healthy = false
  for (let i = 0; i < 30; i++) {
    await sleep(2000)
    try {
      const res = await fetch(`${BACKEND_URL}/markets`)
      if (res.ok) {
        healthy = true
        break
      }
    } catch {
      // Keep trying
    }
    process.stdout.write('.')
  }
  console.log('')
  
  if (!healthy) {
    console.error(red('  ✗ Backend failed to become healthy'))
    process.exit(1)
  }
  console.log(green('  ✓ Backend is healthy'))

  // Step 6: Seed markets
  console.log('')
  console.log(cyan('Step 6:') + ' Seeding markets...')
  
  const seedResult = await createSeedMarkets({
    backendUrl: BACKEND_URL,
    apiKey: TEST_API_KEY,
    verbose: true,
  })
  
  if (seedResult.count > 0) {
    console.log(green(`  ✓ Created ${seedResult.count} markets`))
  } else if (seedResult.markets.length > 0) {
    console.log(green(`  ✓ ${seedResult.markets.length} markets exist`))
  }

  // Step 7: Seed liquidity (optional)
  if (!args.skipLiquidity) {
    console.log('')
    console.log(cyan('Step 7:') + ' Seeding liquidity...')
    
    try {
      await ensureStablecoinRouting(client)
      console.log(green('  ✓ Stablecoin routing configured'))
    } catch (e: any) {
      console.log(yellow(`  ⚠ ${e.message?.slice(0, 50)}`))
    }

    await sleep(RATE_LIMIT_DELAY)

    const marketsRes = await fetch(`${BACKEND_URL}/markets`)
    const marketsData = (await marketsRes.json()) as { markets: Array<{ marketHash: string }> }
    let liquiditySeeded = 0

    for (const market of marketsData.markets || []) {
      const verseRes = await fetch(`${BACKEND_URL}/markets/${market.marketHash}/verse-tokens`)
      const verseTokens = (await verseRes.json()) as Array<{
        asset: string
        yesVerse: string
        noVerse: string
      }>

      for (const vt of verseTokens || []) {
        try {
          console.log(`  Seeding ${market.marketHash.slice(0, 10)}...`)
          await seedMarketLiquidity(
            client,
            {
              marketHash: market.marketHash as `0x${string}`,
              multiVerseAddress: contracts.multiverse as Address,
              quoteToken: vt.asset as Address,
            },
            DEFAULT_SEED_CONFIG
          )
          liquiditySeeded++
        } catch (e: any) {
          console.log(yellow(`    ⚠ ${e.message?.slice(0, 50)}`))
        }
        await sleep(RATE_LIMIT_DELAY)
      }
    }
    console.log(green(`  ✓ Seeded ${liquiditySeeded} market/asset pairs`))
  } else {
    console.log('')
    console.log(dim('Step 7: Skipping liquidity (--skip-liquidity)'))
  }

  // Step 8: Commit and push deployments.json (optional)
  if (!args.noCommit) {
    console.log('')
    console.log(cyan('Step 8:') + ' Committing changes...')
    
    try {
      runCommand('git add deployments.json backend/deployments.json frontend/src/deployments.json', {
        cwd: projectRoot,
      })
      runCommand('git commit -m "Update contract addresses for production"', {
        cwd: projectRoot,
      })
      console.log(green('  ✓ Changes committed'))
      
      console.log('  Pushing to origin...')
      runCommand('git push', { cwd: projectRoot })
      console.log(green('  ✓ Pushed to origin (CI will redeploy frontend)'))
    } catch (e: any) {
      console.log(yellow('  ⚠ Git commit/push failed (may have no changes)'))
    }
  } else {
    console.log('')
    console.log(dim('Step 8: Skipping commit (--no-commit)'))
    console.log(dim('  Remember to commit and push to trigger frontend CI'))
  }

  // Summary
  console.log('')
  console.log(cyan('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━'))
  console.log(green('  Production Redeploy Complete!'))
  console.log(cyan('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━'))
  console.log('')
  console.log('  Contracts:')
  console.log(`    MultiVerse:    ${contracts.multiverse}`)
  console.log(`    Oracle:        ${contracts.oracle}`)
  console.log(`    UniV2 Factory: ${contracts.uniV2Factory}`)
  console.log(`    UniV2 Router:  ${contracts.uniV2Router}`)
  console.log('')
  console.log('  Services:')
  console.log(`    Backend:  ${BACKEND_URL}`)
  console.log(`    Frontend: https://binary-markets.pages.dev (after CI completes)`)
  console.log('')
}

main().catch((err) => {
  console.error(red('Fatal error:'), err.message)
  process.exit(1)
})
