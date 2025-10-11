import type { V4PoolInfo } from '../api/types';

/**
 * Calculate market probability from V4 pool data
 * Uses YES/NO pool combinations to derive probability
 */
export function calculateProbabilityFromPools(pools: V4PoolInfo[]): {
  yesProbability: number;
  noProbability: number;
  source?: string;
} | null {
  if (!pools || pools.length === 0) {
    return null;
  }

  // Look for YES/NO combination pools
  const yesNoPools = pools.filter(pool => {
    const hasYes = pool.poolType.includes('YES');
    const hasNo = pool.poolType.includes('NO');
    return hasYes && hasNo;
  });

  // If we have a YES/NO pool with price data
  if (yesNoPools.length > 0) {
    const pool = yesNoPools[0];

    // If sqrtPriceX96 is available, calculate from it
    if (pool.sqrtPriceX96) {
      // sqrtPriceX96 represents sqrt(price) * 2^96
      // price = (sqrtPriceX96 / 2^96)^2
      const sqrtPrice = BigInt(pool.sqrtPriceX96);
      const Q96 = BigInt(2) ** BigInt(96);

      // Calculate price ratio
      // This gives us YES/NO price ratio
      const priceRatio = Number(sqrtPrice) / Number(Q96);
      const price = priceRatio * priceRatio;

      // Determine which token is token0 and token1
      const isYesToken0 = pool.token0.includes('YES');

      let yesNoRatio: number;
      if (isYesToken0) {
        // price = token0/token1 = YES/NO
        yesNoRatio = price;
      } else {
        // price = token0/token1 = NO/YES, so YES/NO = 1/price
        yesNoRatio = 1 / price;
      }

      // Calculate probabilities
      // YES probability = YES price / (YES price + NO price)
      // If YES/NO ratio = R, then YES prob = R / (R + 1)
      const yesProbability = (yesNoRatio / (yesNoRatio + 1)) * 100;
      const noProbability = 100 - yesProbability;

      return {
        yesProbability: Math.round(yesProbability * 10) / 10,
        noProbability: Math.round(noProbability * 10) / 10,
        source: pool.poolType
      };
    }
  }

  // Fallback: If no price data available, check for other pool types
  // For now, return 50/50 if pools exist but no price data
  if (pools.length > 0) {
    return {
      yesProbability: 50.0,
      noProbability: 50.0,
      source: 'No price data - showing 50/50'
    };
  }

  return null;
}

/**
 * Format probability for display
 */
export function formatProbability(probability: number): string {
  return `${probability.toFixed(1)}%`;
}