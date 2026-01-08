import type { Address } from "viem";
import { Hooks } from "wagmi/tempo";
import { USD_TOKEN } from "../config/wagmi";

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

interface PlaceOrderParams {
  token: Address;
  amount: bigint;
  tick: number;
  type: "buy" | "sell";
  feeToken?: Address;
}

/**
 * Hook to get a buy quote from the Tempo DEX
 * Returns the amount of tokenIn needed to buy amountOut of tokenOut
 */
export function useBuyQuote({
  tokenIn,
  tokenOut,
  amountOut,
  query,
}: BuyQuoteParams) {
  return Hooks.dex.useBuyQuote({
    tokenIn,
    tokenOut,
    amountOut,
    query: {
      enabled: query?.enabled !== false && amountOut > 0n,
    },
  });
}

/**
 * Hook to get a sell quote from the Tempo DEX
 * Returns the amount of tokenOut received for selling amountIn of tokenIn
 */
export function useSellQuote({
  tokenIn,
  tokenOut,
  amountIn,
  query,
}: SellQuoteParams) {
  return Hooks.dex.useSellQuote({
    tokenIn,
    tokenOut,
    amountIn,
    query: {
      enabled: query?.enabled !== false && amountIn > 0n,
    },
  });
}

/**
 * Hook to execute a buy order on the Tempo DEX (synchronous - waits for receipt)
 * Supports gasless transactions via feePayer option
 */
export function useBuySync() {
  const buy = Hooks.dex.useBuy();

  return {
    ...buy,
    mutate: (params: BuyParams) => {
      buy.mutate({
        tokenIn: params.tokenIn,
        tokenOut: params.tokenOut,
        amountOut: params.amountOut,
        maxAmountIn: params.maxAmountIn,
        feeToken: params.feeToken ?? USD_TOKEN,
        feePayer: true,
      });
    },
    mutateAsync: async (params: BuyParams) => {
      return buy.mutateAsync({
        tokenIn: params.tokenIn,
        tokenOut: params.tokenOut,
        amountOut: params.amountOut,
        maxAmountIn: params.maxAmountIn,
        feeToken: params.feeToken ?? USD_TOKEN,
        feePayer: true,
      });
    },
  };
}

/**
 * Hook to execute a sell order on the Tempo DEX (synchronous - waits for receipt)
 * Supports gasless transactions via feePayer option
 */
export function useSellSync() {
  const sell = Hooks.dex.useSell();

  return {
    ...sell,
    mutate: (params: SellParams) => {
      sell.mutate({
        tokenIn: params.tokenIn,
        tokenOut: params.tokenOut,
        amountIn: params.amountIn,
        minAmountOut: params.minAmountOut,
        feeToken: params.feeToken ?? USD_TOKEN,
        feePayer: true,
      });
    },
    mutateAsync: async (params: SellParams) => {
      return sell.mutateAsync({
        tokenIn: params.tokenIn,
        tokenOut: params.tokenOut,
        amountIn: params.amountIn,
        minAmountOut: params.minAmountOut,
        feeToken: params.feeToken ?? USD_TOKEN,
        feePayer: true,
      });
    },
  };
}

/**
 * Hook to place a limit order on the Tempo DEX orderbook
 * Supports gasless transactions via feePayer option
 */
export function usePlaceOrderSync() {
  const place = Hooks.dex.usePlaceSync();

  return {
    ...place,
    mutate: (params: PlaceOrderParams) => {
      place.mutate({
        token: params.token,
        amount: params.amount,
        tick: params.tick,
        type: params.type,
        feeToken: params.feeToken ?? USD_TOKEN,
        feePayer: true,
      });
    },
    mutateAsync: async (params: PlaceOrderParams) => {
      return place.mutateAsync({
        token: params.token,
        amount: params.amount,
        tick: params.tick,
        type: params.type,
        feeToken: params.feeToken ?? USD_TOKEN,
        feePayer: true,
      });
    },
  };
}
