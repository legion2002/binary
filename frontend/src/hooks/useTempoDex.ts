import { useQuery, useMutation } from "@tanstack/react-query";
import { usePublicClient, useWalletClient } from "wagmi";
import { Actions, tempoActions } from "viem/tempo";
import type { Address } from "viem";

// Types for hook parameters
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
 * Hook to get a buy quote from the Tempo DEX
 * Returns the amount of tokenIn needed to buy amountOut of tokenOut
 */
export function useBuyQuote({ tokenIn, tokenOut, amountOut, query }: BuyQuoteParams) {
  const publicClient = usePublicClient();

  return useQuery({
    queryKey: ["tempo", "dex", "buyQuote", tokenIn, tokenOut, amountOut?.toString()],
    queryFn: async () => {
      if (!publicClient) throw new Error("No public client");
      const client = publicClient.extend(tempoActions());
      return Actions.dex.getBuyQuote(client, {
        tokenIn,
        tokenOut,
        amountOut,
      });
    },
    enabled: query?.enabled !== false && !!publicClient && amountOut > 0n,
  });
}

/**
 * Hook to get a sell quote from the Tempo DEX
 * Returns the amount of tokenOut received for selling amountIn of tokenIn
 */
export function useSellQuote({ tokenIn, tokenOut, amountIn, query }: SellQuoteParams) {
  const publicClient = usePublicClient();

  return useQuery({
    queryKey: ["tempo", "dex", "sellQuote", tokenIn, tokenOut, amountIn?.toString()],
    queryFn: async () => {
      if (!publicClient) throw new Error("No public client");
      const client = publicClient.extend(tempoActions());
      return Actions.dex.getSellQuote(client, {
        tokenIn,
        tokenOut,
        amountIn,
      });
    },
    enabled: query?.enabled !== false && !!publicClient && amountIn > 0n,
  });
}

/**
 * Hook to execute a buy order on the Tempo DEX (synchronous - waits for receipt)
 */
export function useBuySync() {
  const { data: walletClient } = useWalletClient();
  const publicClient = usePublicClient();

  return useMutation({
    mutationFn: async (params: BuyParams) => {
      if (!walletClient) throw new Error("No wallet client");
      if (!publicClient) throw new Error("No public client");

      // Create a client with tempo actions
      const client = walletClient.extend(tempoActions());

      return Actions.dex.buySync(client, {
        tokenIn: params.tokenIn,
        tokenOut: params.tokenOut,
        amountOut: params.amountOut,
        maxAmountIn: params.maxAmountIn,
        feeToken: params.feeToken,
      });
    },
  });
}

/**
 * Hook to execute a sell order on the Tempo DEX (synchronous - waits for receipt)
 */
export function useSellSync() {
  const { data: walletClient } = useWalletClient();
  const publicClient = usePublicClient();

  return useMutation({
    mutationFn: async (params: SellParams) => {
      if (!walletClient) throw new Error("No wallet client");
      if (!publicClient) throw new Error("No public client");

      // Create a client with tempo actions
      const client = walletClient.extend(tempoActions());

      return Actions.dex.sellSync(client, {
        tokenIn: params.tokenIn,
        tokenOut: params.tokenOut,
        amountIn: params.amountIn,
        minAmountOut: params.minAmountOut,
        feeToken: params.feeToken,
      });
    },
  });
}
