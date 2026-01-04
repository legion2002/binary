import { useQuery, useMutation } from "@tanstack/react-query";
import type { Address } from "viem";
import {
  usePasskey,
  useTempoPublicClient,
  useTempoWalletClient,
} from "../contexts/PasskeyContext";

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
export function useBuyQuote({
  tokenIn,
  tokenOut,
  amountOut,
  query,
}: BuyQuoteParams) {
  const publicClient = useTempoPublicClient();

  return useQuery({
    queryKey: [
      "tempo",
      "dex",
      "buyQuote",
      tokenIn,
      tokenOut,
      amountOut?.toString(),
    ],
    queryFn: async () => {
      return publicClient.dex.getBuyQuote({
        tokenIn,
        tokenOut,
        amountOut,
      });
    },
    enabled: query?.enabled !== false && amountOut > 0n,
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
  const publicClient = useTempoPublicClient();

  return useQuery({
    queryKey: [
      "tempo",
      "dex",
      "sellQuote",
      tokenIn,
      tokenOut,
      amountIn?.toString(),
    ],
    queryFn: async () => {
      return publicClient.dex.getSellQuote({
        tokenIn,
        tokenOut,
        amountIn,
      });
    },
    enabled: query?.enabled !== false && amountIn > 0n,
  });
}

/**
 * Hook to execute a buy order on the Tempo DEX (synchronous - waits for receipt)
 * Supports gasless transactions via feePayer option
 */
export function useBuySync() {
  const walletClient = useTempoWalletClient();
  const { isConnected } = usePasskey();

  return useMutation({
    mutationFn: async (params: BuyParams) => {
      if (!walletClient) throw new Error("No wallet client - please sign in");
      if (!isConnected) throw new Error("Not connected");

      // Use the decorated client's dex.buySync method
      return walletClient.dex.buySync({
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
 * Supports gasless transactions via feePayer option
 */
export function useSellSync() {
  const walletClient = useTempoWalletClient();
  const { isConnected } = usePasskey();

  return useMutation({
    mutationFn: async (params: SellParams) => {
      if (!walletClient) throw new Error("No wallet client - please sign in");
      if (!isConnected) throw new Error("Not connected");

      // Use the decorated client's dex.sellSync method
      return walletClient.dex.sellSync({
        tokenIn: params.tokenIn,
        tokenOut: params.tokenOut,
        amountIn: params.amountIn,
        minAmountOut: params.minAmountOut,
        feeToken: params.feeToken,
      });
    },
  });
}
