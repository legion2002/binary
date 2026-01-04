import type { OrderbookInfo } from '../api/types';

/**
 * Calculate market probability from orderbook data
 * Uses YES/NO orderbook mid prices to derive probability
 *
 * In a prediction market:
 * - YES token price reflects probability of YES outcome
 * - NO token price reflects probability of NO outcome
 * - Prices should sum to ~1.0 (with some spread)
 */
export function calculateProbabilityFromOrderbooks(orderbooks: OrderbookInfo[]): {
  yesProbability: number;
  noProbability: number;
  source?: string;
} | null {
  if (!orderbooks || orderbooks.length === 0) {
    return null;
  }

  // Find YES and NO orderbooks
  const yesOrderbook = orderbooks.find(ob => ob.pairType.includes('YES'));
  const noOrderbook = orderbooks.find(ob => ob.pairType.includes('NO'));

  // Try to use mid price if available
  if (yesOrderbook?.midPrice && noOrderbook?.midPrice) {
    const yesPrice = parseFloat(yesOrderbook.midPrice);
    const noPrice = parseFloat(noOrderbook.midPrice);

    if (yesPrice > 0 && noPrice > 0) {
      // Calculate probability from relative prices
      // YES probability = YES price / (YES price + NO price)
      const yesProbability = (yesPrice / (yesPrice + noPrice)) * 100;
      const noProbability = 100 - yesProbability;

      return {
        yesProbability: Math.round(yesProbability * 10) / 10,
        noProbability: Math.round(noProbability * 10) / 10,
        source: 'Orderbook mid prices'
      };
    }
  }

  // Try to use best bid/ask prices
  if (yesOrderbook?.bestBidPrice && yesOrderbook?.bestAskPrice) {
    const yesBid = parseFloat(yesOrderbook.bestBidPrice);
    const yesAsk = parseFloat(yesOrderbook.bestAskPrice);
    const yesMid = (yesBid + yesAsk) / 2;

    let noMid = 1.0 - yesMid; // Default: assume NO price is inverse

    if (noOrderbook?.bestBidPrice && noOrderbook?.bestAskPrice) {
      const noBid = parseFloat(noOrderbook.bestBidPrice);
      const noAsk = parseFloat(noOrderbook.bestAskPrice);
      noMid = (noBid + noAsk) / 2;
    }

    if (yesMid > 0 && noMid > 0) {
      const yesProbability = (yesMid / (yesMid + noMid)) * 100;
      const noProbability = 100 - yesProbability;

      return {
        yesProbability: Math.round(yesProbability * 10) / 10,
        noProbability: Math.round(noProbability * 10) / 10,
        source: 'Orderbook bid/ask'
      };
    }
  }

  // Try to use just YES orderbook mid price (assume price = probability)
  if (yesOrderbook?.midPrice) {
    const yesMid = parseFloat(yesOrderbook.midPrice);
    if (yesMid > 0 && yesMid <= 2) {
      // Assume price close to 1.0 means 50%, price of 0.5 means ~33%, etc.
      // For stablecoins pegged to $1, price deviation from 1.0 indicates probability
      const yesProbability = yesMid * 50; // Simple scaling
      const noProbability = 100 - yesProbability;

      return {
        yesProbability: Math.min(99, Math.max(1, Math.round(yesProbability * 10) / 10)),
        noProbability: Math.min(99, Math.max(1, Math.round(noProbability * 10) / 10)),
        source: 'YES mid price only'
      };
    }
  }

  // Fallback: If orderbooks exist but no price data, return 50/50
  if (orderbooks.length > 0) {
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

/**
 * Format spread in basis points for display
 */
export function formatSpread(spreadBps: number | null | undefined): string {
  if (spreadBps == null) return 'N/A';
  return `${spreadBps.toFixed(1)} bps`;
}

/**
 * Calculate implied probability from a single token price
 * Assumes token price in quote currency (e.g., pathUSD)
 */
export function priceToImpliedProbability(price: number): number {
  // In a binary market, if YES costs $0.60 and NO costs $0.40,
  // the implied probability of YES is 60%
  // This assumes the quote token is always $1 (pathUSD)
  return Math.min(99, Math.max(1, price * 100));
}
