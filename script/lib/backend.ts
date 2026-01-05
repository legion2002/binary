/**
 * Backend process management
 */

import { spawn, type ChildProcess } from 'child_process'
import { join } from 'path'
import { waitForHttp, clearLine } from './health'
import { TEST_PRIVATE_KEY } from './tempo'
import type { ContractAddresses } from './deploy'

export const BACKEND_PORT = Number(process.env.BACKEND_PORT) || 3000
export const BACKEND_TEST_PORT = 3001

// Pre-computed bcrypt hash for "test_api_key_12345"
// Generated with: bun -e "console.log(await Bun.password.hash('test_api_key_12345', { algorithm: 'bcrypt', cost: 10 }))"
export const TEST_API_KEY = 'test_api_key_12345'
export const TEST_API_KEY_HASH = '$2b$10$pFkqqOn5jFfiIPFGLoU9/uGIF8.QDon0ZOE9ZZ6WxrmSEypkTpI0O'

export interface BackendOptions {
  rpcUrl: string
  contracts: ContractAddresses
  projectRoot: string
  mode: 'dev' | 'test'
  verbose?: boolean
}

export interface BackendProcess {
  process: ChildProcess
  url: string
  port: number
  apiKey: string
  stop: () => Promise<void>
}

export async function startBackend(options: BackendOptions): Promise<BackendProcess> {
  const {
    rpcUrl,
    contracts,
    projectRoot,
    mode,
    verbose = false,
  } = options

  const port = mode === 'dev' ? BACKEND_PORT : BACKEND_TEST_PORT
  const url = `http://127.0.0.1:${port}`
  const backendDir = join(projectRoot, 'backend')

  const env: NodeJS.ProcessEnv = {
    ...process.env,
    HOST: '127.0.0.1',
    PORT: String(port),
    MULTIVERSE_ADDRESS: contracts.multiverse,
    ORACLE_ADDRESS: contracts.oracle,
    RPC_URL: rpcUrl,
    WS_RPC_URL: rpcUrl.replace('http', 'ws'),
    PRIVATE_KEY: TEST_PRIVATE_KEY,
    DATABASE_URL: mode === 'dev' ? 'sqlite:./dev.db?mode=rwc' : 'sqlite::memory:',
    ADMIN_API_KEY_HASH: TEST_API_KEY_HASH,
    RUST_LOG: process.env.RUST_LOG ?? 'info,backend=debug',
  }

  const child = spawn('cargo', ['run', '--bin', 'backend'], {
    cwd: backendDir,
    env,
    stdio: verbose ? 'inherit' : 'pipe',
  })

  // Handle process errors
  child.on('error', (err) => {
    console.error('Backend process error:', err)
  })

  if (!verbose && child.stderr) {
    child.stderr.on('data', (data) => {
      const msg = data.toString()
      // Only show errors
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
      return new Promise((resolve) => {
        if (child.killed) {
          resolve()
          return
        }
        child.on('exit', () => resolve())
        child.kill('SIGTERM')
        // Force kill after 5s
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
