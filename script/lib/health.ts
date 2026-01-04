/**
 * Health check utilities for waiting on services
 */

export interface HealthCheckOptions {
  timeoutMs?: number
  retryMs?: number
  silent?: boolean
}

const defaultOptions: Required<HealthCheckOptions> = {
  timeoutMs: 30_000,
  retryMs: 1_000,
  silent: false,
}

export async function waitForHttp(
  url: string,
  opts: HealthCheckOptions = {}
): Promise<void> {
  const { timeoutMs, retryMs, silent } = { ...defaultOptions, ...opts }
  const start = Date.now()

  while (true) {
    try {
      const res = await fetch(url)
      if (res.ok) return
    } catch {
      // Service not ready yet
    }

    if (Date.now() - start > timeoutMs) {
      throw new Error(`Service at ${url} not healthy after ${timeoutMs}ms`)
    }

    if (!silent) {
      const elapsed = Math.round((Date.now() - start) / 1000)
      process.stdout.write(`\r  Waiting for ${url}... (${elapsed}s)`)
    }

    await sleep(retryMs)
  }
}

export async function waitForRpc(
  rpcUrl: string,
  opts: HealthCheckOptions = {}
): Promise<void> {
  const { timeoutMs, retryMs, silent } = { ...defaultOptions, ...opts }
  const start = Date.now()

  while (true) {
    try {
      const res = await fetch(rpcUrl, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          jsonrpc: '2.0',
          method: 'eth_blockNumber',
          params: [],
          id: 1,
        }),
      })
      const data = await res.json()
      if (data.result) return
    } catch {
      // RPC not ready yet
    }

    if (Date.now() - start > timeoutMs) {
      throw new Error(`RPC at ${rpcUrl} not healthy after ${timeoutMs}ms`)
    }

    if (!silent) {
      const elapsed = Math.round((Date.now() - start) / 1000)
      process.stdout.write(`\r  Waiting for RPC at ${rpcUrl}... (${elapsed}s)`)
    }

    await sleep(retryMs)
  }
}

export function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

export function clearLine(): void {
  process.stdout.write('\r\x1b[K')
}
