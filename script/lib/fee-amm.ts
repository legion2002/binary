/**
 * Fee token setup utilities
 * Configures fee token preferences so transactions can work on local dev nodes
 */

import type { TempoClient } from './tempo'
import { PATH_USD } from './tempo'
import { sleep } from './health'

// FeeManager precompile address  
export const FEE_MANAGER = '0xfeec000000000000000000000000000000000000' as const

/**
 * Sets the user's fee token preference to PathUSD.
 * 
 * On local dev nodes, the validator uses PathUSD as its preferred fee token.
 * By setting the user's fee token to PathUSD (same as validator), no Fee AMM
 * swap is needed, avoiding the "Insufficient liquidity" error.
 * 
 * The setUserToken call itself uses the specified token as the fee token
 * (special rule in the fee spec), so this bootstrap transaction works.
 * 
 * After this, all legacy EVM transactions (like forge) will use PathUSD.
 */
export async function setUserFeeToken(
  client: TempoClient,
  token: `0x${string}` = PATH_USD
): Promise<void> {
  const account = client.account
  if (!account) throw new Error('Client must have an account')

  console.log(`  Setting user fee token to ${token.slice(0, 10)}...`)

  try {
    // Check current setting first
    const currentToken = await client.fee.getUserToken({})
    if (currentToken?.address?.toLowerCase() === token.toLowerCase()) {
      console.log('  ✓ User fee token already set')
      return
    }

    // Set the fee token - this call uses the specified token for its own fees
    await client.fee.setUserTokenSync({ token })
    await sleep(500)
    console.log('  ✓ User fee token set to PathUSD')
  } catch (error: any) {
    console.error('  ✗ Failed to set user fee token:', error.message)
    throw error
  }
}
