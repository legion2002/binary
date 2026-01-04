#!/usr/bin/env bun
/**
 * Binary Protocol - Environment Orchestrator
 *
 * Single command to start the full development/testing environment:
 * - Local Tempo node (via prool/Docker)
 * - Contract deployment
 * - Backend server
 * - Frontend dev server (in dev mode)
 *
 * Usage:
 *   bun run script/env.ts dev              # Interactive development
 *   bun run script/env.ts test             # Run default tests
 *   bun run script/env.ts test -- <cmd>    # Run custom test command
 *
 * Examples:
 *   bun run script/env.ts dev
 *   bun run script/env.ts test -- cargo test --test integration_test -- --ignored
 *   bun run script/env.ts test --with-frontend -- pnpm --prefix frontend test:e2e
 */

import { spawn } from 'child_process'
import { startLocalTempo, type TempoEnvironment } from './lib/tempo'
import { deployContracts, getProjectRoot, type ContractAddresses } from './lib/deploy'
import { startBackend, TEST_API_KEY, type BackendProcess } from './lib/backend'
import { startFrontend, type FrontendProcess } from './lib/frontend'

// Colors for console output
const cyan = (s: string) => `\x1b[36m${s}\x1b[0m`
const green = (s: string) => `\x1b[32m${s}\x1b[0m`
const yellow = (s: string) => `\x1b[33m${s}\x1b[0m`
const red = (s: string) => `\x1b[31m${s}\x1b[0m`

type Mode = 'dev' | 'test'

interface Environment {
  tempo?: TempoEnvironment
  contracts?: ContractAddresses
  backend?: BackendProcess
  frontend?: FrontendProcess
}

const env: Environment = {}

function parseArgs(): { mode: Mode; withFrontend: boolean; testCmd: string[] } {
  const args = process.argv.slice(2)
  const mode = (args[0] as Mode) || 'dev'

  if (mode !== 'dev' && mode !== 'test') {
    console.error(red(`Unknown mode: ${mode}`))
    console.error('Usage: bun run script/env.ts [dev|test] [--with-frontend] [-- <test command>]')
    process.exit(1)
  }

  const withFrontend = args.includes('--with-frontend')
  const dashDashIndex = args.indexOf('--')
  const testCmd = dashDashIndex >= 0 ? args.slice(dashDashIndex + 1) : []

  return { mode, withFrontend, testCmd }
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

  if (env.tempo) {
    console.log('  Stopping Tempo node...')
    await env.tempo.server.stop()
  }

  console.log(green('✓ All services stopped'))
}

async function runTests(testCmd: string[], projectRoot: string): Promise<number> {
  return new Promise((resolve) => {
    const [cmd, ...args] = testCmd.length > 0 ? testCmd : ['cargo', 'test', '--test', 'integration_test', '--', '--ignored']

    console.log(cyan(`Running: ${cmd} ${args.join(' ')}`))
    console.log('')

    const child = spawn(cmd, args, {
      cwd: testCmd.length > 0 && testCmd[0] === 'cargo' ? `${projectRoot}/backend` : projectRoot,
      stdio: 'inherit',
      env: {
        ...process.env,
        // Pass environment info to tests
        RPC_URL: env.tempo?.rpcUrl,
        SERVER_URL: env.backend?.url,
        ADMIN_API_KEY: TEST_API_KEY,
        MULTIVERSE_ADDRESS: env.contracts?.multiverse,
        ORACLE_ADDRESS: env.contracts?.oracle,
      },
    })

    child.on('exit', (code) => {
      resolve(code ?? 1)
    })

    child.on('error', (err) => {
      console.error(red(`Test command failed: ${err.message}`))
      resolve(1)
    })
  })
}

function printDevSummary(projectRoot: string): void {
  console.log('')
  console.log(cyan('═══════════════════════════════════════════════════════════'))
  console.log(cyan('  Binary Protocol - Development Environment Ready'))
  console.log(cyan('═══════════════════════════════════════════════════════════'))
  console.log('')
  console.log(green('  Services:'))
  console.log(`    Tempo RPC:  ${env.tempo?.rpcUrl}`)
  console.log(`    Backend:    ${env.backend?.url}`)
  if (env.frontend) {
    console.log(`    Frontend:   ${env.frontend.url}`)
  }
  console.log('')
  console.log(green('  Contracts:'))
  console.log(`    MultiVerse: ${env.contracts?.multiverse}`)
  console.log(`    Oracle:     ${env.contracts?.oracle}`)
  console.log('')
  console.log(green('  Test Account:'))
  console.log(`    Address:    ${env.tempo?.account.address}`)
  console.log(`    Private Key: 0xac0974...ff80 (test key)`)
  console.log('')
  console.log(green('  Admin API:'))
  console.log(`    API Key:    ${TEST_API_KEY}`)
  console.log('')
  console.log(yellow('  Press Ctrl+C to stop all services'))
  console.log('')
}

async function main(): Promise<void> {
  const { mode, withFrontend, testCmd } = parseArgs()
  const projectRoot = getProjectRoot()
  const verbose = process.env.VERBOSE === '1'

  console.log('')
  console.log(cyan('═══════════════════════════════════════════════════════════'))
  console.log(cyan(`  Binary Protocol - ${mode === 'dev' ? 'Development' : 'Test'} Environment`))
  console.log(cyan('═══════════════════════════════════════════════════════════'))
  console.log('')

  // Register cleanup handlers
  const cleanup = async () => {
    await shutdown()
    process.exit(0)
  }
  process.on('SIGINT', cleanup)
  process.on('SIGTERM', cleanup)

  try {
    // Step 1: Start Tempo node
    console.log(cyan('Step 1:') + ' Starting local Tempo node...')
    env.tempo = await startLocalTempo()
    console.log(green('  ✓ Tempo node running at ' + env.tempo.rpcUrl))

    // Step 2: Deploy contracts
    console.log('')
    console.log(cyan('Step 2:') + ' Deploying contracts...')
    env.contracts = await deployContracts({
      rpcUrl: env.tempo.rpcUrl,
      projectRoot,
      verbose,
    })
    console.log(green('  ✓ Contracts deployed'))
    console.log(`    MultiVerse: ${env.contracts.multiverse}`)
    console.log(`    Oracle:     ${env.contracts.oracle}`)

    // Step 3: Start backend
    console.log('')
    console.log(cyan('Step 3:') + ' Starting backend server...')
    env.backend = await startBackend({
      rpcUrl: env.tempo.rpcUrl,
      contracts: env.contracts,
      projectRoot,
      mode,
      verbose,
    })
    console.log(green('  ✓ Backend running at ' + env.backend.url))

    // Step 4: Handle mode-specific logic
    if (mode === 'dev') {
      // Start frontend for dev mode
      console.log('')
      console.log(cyan('Step 4:') + ' Starting frontend dev server...')
      env.frontend = await startFrontend({ projectRoot, verbose })
      console.log(green('  ✓ Frontend running at ' + env.frontend.url))

      printDevSummary(projectRoot)

      // Keep running until interrupted
      await new Promise(() => {})
    } else {
      // Test mode
      if (withFrontend) {
        console.log('')
        console.log(cyan('Step 4:') + ' Starting frontend for e2e tests...')
        env.frontend = await startFrontend({ projectRoot, verbose })
        console.log(green('  ✓ Frontend running at ' + env.frontend.url))
      }

      console.log('')
      console.log(cyan('Step ' + (withFrontend ? '5' : '4') + ':') + ' Running tests...')
      console.log('')

      const exitCode = await runTests(testCmd, projectRoot)

      await shutdown()
      process.exit(exitCode)
    }
  } catch (error) {
    console.error('')
    console.error(red('Error: ' + (error instanceof Error ? error.message : String(error))))
    await shutdown()
    process.exit(1)
  }
}

main().catch(async (error) => {
  console.error(red('Fatal error:'), error)
  await shutdown()
  process.exit(1)
})
