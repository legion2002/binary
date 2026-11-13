/**
 * Gavel council settings for the testnet environment.
 *
 * With MARKET_ORACLE=council, markets are opened with the CouncilOracle and resolved by a
 * Gavel council: the backend runs the gavel CLI, which pays for each query over MPP on Tempo,
 * and submits the signed decision record on chain. This fills in the COUNCIL_* settings the
 * backend and the deploy script read, asking the Gavel API for what is not set.
 */

import { execFileSync } from 'child_process'

/** The default pack for yes/no questions. */
export const MARKET_PACK = 'market'

export interface GavelSettings {
  url: string
  cli: string
  councilHash: string
  signer: string
  network: string
}

function gavelJson(cli: string, url: string, args: string[]): any {
  const out = execFileSync(cli, ['--url', url, '--json', ...args], { encoding: 'utf-8' })
  return JSON.parse(out)
}

/**
 * Returns null unless MARKET_ORACLE=council. Otherwise sets COUNCIL_SIGNER (deploy),
 * COUNCIL_HASH, COUNCIL_SIGNERS, COUNCIL_URL, COUNCIL_CLI, COUNCIL_NETWORK and
 * COUNCIL_PRIVATE_KEY (backend) in process.env, keeping any already set.
 *
 * - COUNCIL_URL (or GAVEL_URL) is required: the Gavel API to ask.
 * - COUNCIL_HASH defaults to the Gavel "market" pack (5 members, supermajority, binary).
 * - COUNCIL_SIGNERS defaults to the signer in the council service's attestation.
 * - COUNCIL_PRIVATE_KEY defaults to PRIVATE_KEY, so the deployer pays for queries.
 */
export function configureGavel(): GavelSettings | null {
  if ((process.env.MARKET_ORACLE ?? 'trusted') !== 'council') return null
  const env = process.env
  const url = env.COUNCIL_URL || env.GAVEL_URL
  if (!url) {
    throw new Error('MARKET_ORACLE=council needs COUNCIL_URL (or GAVEL_URL): the Gavel API')
  }
  const cli = env.COUNCIL_CLI || 'gavel'

  let councilHash = env.COUNCIL_HASH
  if (!councilHash) {
    const packs = gavelJson(cli, url, ['packs']) as Array<{ id: string; hash: string }>
    const pack = packs.find((p) => p.id === MARKET_PACK)
    councilHash = pack?.hash
    if (!councilHash) throw new Error('Gavel has no "market" pack: set COUNCIL_HASH')
  }

  let signer = env.COUNCIL_SIGNERS?.split(',')[0]?.trim() || env.COUNCIL_SIGNER
  if (!signer) {
    signer = (gavelJson(cli, url, ['attestation']) as { signer: string }).signer
  }

  env.COUNCIL_URL = url
  env.COUNCIL_CLI = cli
  env.COUNCIL_HASH = councilHash
  env.COUNCIL_SIGNER = env.COUNCIL_SIGNER || signer
  env.COUNCIL_SIGNERS = env.COUNCIL_SIGNERS || signer
  env.COUNCIL_NETWORK = env.COUNCIL_NETWORK || 'moderato'
  env.COUNCIL_PRIVATE_KEY = env.COUNCIL_PRIVATE_KEY || env.PRIVATE_KEY

  return { url, cli, councilHash, signer, network: env.COUNCIL_NETWORK }
}
