#!/usr/bin/env bun
/**
 * Updates deployments.json from forge broadcast files
 * 
 * Usage:
 *   bun run script/update-deployments.ts [chainId]
 *   bun run script/update-deployments.ts          # Updates all chains with broadcast files
 *   bun run script/update-deployments.ts 42431    # Updates only chain 42431
 */

import { readFileSync, writeFileSync, existsSync } from 'fs'
import { join } from 'path'

const CHAIN_NAMES: Record<string, string> = {
  '1337': 'Tempo Local',
  '42431': 'Tempo Moderato',
}

interface Deployment {
  name: string
  multiverse: string
  oracle: string
  uniV2Factory: string
  uniV2Router: string
}

interface BroadcastTx {
  transactionType?: string
  contractName?: string
  contractAddress?: string
}

function getProjectRoot(): string {
  let cwd = process.cwd()
  if (cwd.endsWith('/script')) cwd = join(cwd, '..')
  return cwd
}

function parseDeployBroadcast(projectRoot: string, chainId: string): { multiverse?: string; oracle?: string } {
  const path = join(projectRoot, `contracts/broadcast/Deploy.s.sol/${chainId}/run-latest.json`)
  if (!existsSync(path)) return {}

  const broadcast = JSON.parse(readFileSync(path, 'utf-8'))
  const txs = broadcast.transactions as BroadcastTx[]

  let multiverse: string | undefined
  let oracle: string | undefined

  for (const tx of txs) {
    if (tx.contractName === 'MultiVerse' && tx.contractAddress) {
      multiverse = tx.contractAddress
    }
    if (tx.contractName === 'TrustedOracle' && tx.contractAddress) {
      oracle = tx.contractAddress
    }
  }

  return { multiverse, oracle }
}

function parseUniV2Broadcast(projectRoot: string, chainId: string): { uniV2Factory?: string; uniV2Router?: string } {
  const path = join(projectRoot, `contracts/broadcast/DeployUniV2.s.sol/${chainId}/run-latest.json`)
  if (!existsSync(path)) return {}

  const broadcast = JSON.parse(readFileSync(path, 'utf-8'))
  const txs = broadcast.transactions as BroadcastTx[]

  // UniV2 uses inline assembly create, so contractName is null
  // Parse by order: Factory first, Router second
  const createTxs = txs.filter(tx => tx.transactionType === 'CREATE' && tx.contractAddress)

  if (createTxs.length >= 2) {
    return {
      uniV2Factory: createTxs[0].contractAddress,
      uniV2Router: createTxs[1].contractAddress,
    }
  }

  return {}
}

function updateDeployments(chainIds?: string[]): void {
  const projectRoot = getProjectRoot()
  const deploymentsPath = join(projectRoot, 'deployments.json')

  // Load existing deployments or create empty object
  let deployments: Record<string, Deployment> = {}
  if (existsSync(deploymentsPath)) {
    deployments = JSON.parse(readFileSync(deploymentsPath, 'utf-8'))
  }

  // If no chain IDs specified, find all chains with broadcast files
  if (!chainIds || chainIds.length === 0) {
    chainIds = Object.keys(CHAIN_NAMES)
  }

  for (const chainId of chainIds) {
    const existing = deployments[chainId] || {
      name: CHAIN_NAMES[chainId] || `Chain ${chainId}`,
      multiverse: '',
      oracle: '',
      uniV2Factory: '',
      uniV2Router: '',
    }

    // Parse broadcasts
    const deploy = parseDeployBroadcast(projectRoot, chainId)
    const univ2 = parseUniV2Broadcast(projectRoot, chainId)

    // Update with new values (keep existing if not found in broadcast)
    const updated: Deployment = {
      name: existing.name,
      multiverse: deploy.multiverse || existing.multiverse,
      oracle: deploy.oracle || existing.oracle,
      uniV2Factory: univ2.uniV2Factory || existing.uniV2Factory,
      uniV2Router: univ2.uniV2Router || existing.uniV2Router,
    }

    // Only update if we found something
    if (deploy.multiverse || deploy.oracle || univ2.uniV2Factory || univ2.uniV2Router) {
      deployments[chainId] = updated
      console.log(`Updated chain ${chainId} (${updated.name}):`)
      if (deploy.multiverse) console.log(`  multiverse: ${deploy.multiverse}`)
      if (deploy.oracle) console.log(`  oracle: ${deploy.oracle}`)
      if (univ2.uniV2Factory) console.log(`  uniV2Factory: ${univ2.uniV2Factory}`)
      if (univ2.uniV2Router) console.log(`  uniV2Router: ${univ2.uniV2Router}`)
    }
  }

  // Write back
  const content = JSON.stringify(deployments, null, 2) + '\n'
  writeFileSync(deploymentsPath, content)
  console.log(`\nUpdated ${deploymentsPath}`)

  // Also copy to backend and frontend
  const backendPath = join(projectRoot, 'backend/deployments.json')
  writeFileSync(backendPath, content)
  console.log(`Copied to ${backendPath}`)

  const frontendPath = join(projectRoot, 'frontend/src/deployments.json')
  writeFileSync(frontendPath, content)
  console.log(`Copied to ${frontendPath}`)
}

// Main
const chainIds = process.argv.slice(2)
updateDeployments(chainIds.length > 0 ? chainIds : undefined)
