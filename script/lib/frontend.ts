/**
 * Frontend process management
 */

import { spawn, type ChildProcess } from 'child_process'
import { join } from 'path'
import { waitForHttp, clearLine } from './health'

export const FRONTEND_PORT = Number(process.env.FRONTEND_PORT) || 5173

export interface FrontendOptions {
  projectRoot: string
  verbose?: boolean
}

export interface FrontendProcess {
  process: ChildProcess
  url: string
  port: number
  stop: () => Promise<void>
}

export async function startFrontend(options: FrontendOptions): Promise<FrontendProcess> {
  const { projectRoot, verbose = false } = options

  const port = FRONTEND_PORT
  const url = `http://127.0.0.1:${port}`
  const frontendDir = join(projectRoot, 'frontend')

  const child = spawn('pnpm', ['dev'], {
    cwd: frontendDir,
    stdio: verbose ? 'inherit' : 'pipe',
    env: {
      ...process.env,
      // Vite will use these
      VITE_BACKEND_URL: `http://127.0.0.1:3000`,
    },
  })

  // Handle process errors
  child.on('error', (err) => {
    console.error('Frontend process error:', err)
  })

  if (!verbose && child.stdout) {
    child.stdout.on('data', (data) => {
      const msg = data.toString()
      // Show Vite ready message
      if (msg.includes('Local:') || msg.includes('ready in')) {
        console.log(msg.trim())
      }
    })
  }

  // Wait for frontend to be ready
  await waitForHttp(url, { timeoutMs: 60_000, retryMs: 1_000 })
  clearLine()

  return {
    process: child,
    url,
    port,
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
