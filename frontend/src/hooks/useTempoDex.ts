import { type Address } from "viem";
import {
  useUniV2BuyQuote,
  useUniV2SellQuote,
  useUniV2BuySync,
  useUniV2SellSync,
} from "./useUniV2";

interface QuoteParams {
  tokenIn: Address;
  tokenOut: Address;
  query?: {
    enabled?: boolean;
  };
}

interface BuyQuoteParams extends QuoteParams {
  amountOut: bigint;
}

interface SellQuoteParams extends QuoteParams {
  amountIn: bigint;
}

interface BuyParams {
  tokenIn: Address;
  tokenOut: Address;
  amountOut: bigint;
  maxAmountIn: bigint;
  feeToken?: Address;
}

interface SellParams {
  tokenIn: Address;
  tokenOut: Address;
  amountIn: bigint;
  minAmountOut: bigint;
  feeToken?: Address;
}

/**
 * Hook to get a buy quote
 * Returns the amount of tokenIn needed to buy amountOut of tokenOut
 */
export function useBuyQuote(params: BuyQuoteParams) {
  return useUniV2BuyQuote(params);
}

/**
 * Hook to get a sell quote
 * Returns the amount of tokenOut received for selling amountIn of tokenIn
 */
export function useSellQuote(params: SellQuoteParams) {
  return useUniV2SellQuote(params);
}

/**
 * Hook to execute a buy order (synchronous - waits for receipt)
 * Uses call batching to approve + swap in a single atomic transaction.
 */
export function useBuySync() {
  return useUniV2BuySync();
}

/**
 * Hook to execute a sell order (synchronous - waits for receipt)
 * Uses call batching to approve + swap in a single atomic transaction.
 */
export function useSellSync() {
  return useUniV2SellSync();
}

// Re-export types for consumers
export type { BuyQuoteParams, SellQuoteParams, BuyParams, SellParams };
