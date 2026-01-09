import { type Address, encodeFunctionData, parseAbi } from "viem";
import { Hooks } from "wagmi/tempo";
import { useSendTransactionSync } from "wagmi";
import { useQueryClient } from "@tanstack/react-query";
import { FEE_TOKEN } from "../config/wagmi";

// Stablecoin DEX precompile address
const STABLECOIN_DEX = "0xDEc0000000000000000000000000000000000000" as const;

// ABIs for manual call encoding
const TIP20_ABI = parseAbi([
  "function approve(address spender, uint256 amount) external returns (bool)",
]);

const DEX_ABI = parseAbi([
  "function swapExactAmountIn(address tokenIn, address tokenOut, uint128 amountIn, uint128 minAmountOut) external returns (uint128 amountOut)",
  "function swapExactAmountOut(address tokenIn, address tokenOut, uint128 amountOut, uint128 maxAmountIn) external returns (uint128 amountIn)",
]);

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
 * Invalidate all balance and DEX quote queries to refresh UI after trades
 */
async function invalidateTradeQueries(queryClient: ReturnType<typeof useQueryClient>) {
  console.log("[invalidateTradeQueries] Invalidating all trade-related queries...");

  // Get all queries and log them for debugging
  const allQueries = queryClient.getQueryCache().getAll();
  console.log("[invalidateTradeQueries] All query keys:", allQueries.map(q => q.queryKey));

  // Invalidate ALL queries - this is aggressive but ensures UI updates
  await queryClient.invalidateQueries();

  console.log("[invalidateTradeQueries] All queries invalidated");
}

/**
 * Hook to execute a buy order on the Tempo DEX (synchronous - waits for receipt)
 * Uses call batching to approve + swap in a single atomic transaction.
 * Supports gasless transactions via feePayer option
 */
export function useBuySync() {
  const { mutateAsync: sendTransactionSync, isPending, isSuccess, error, reset } = useSendTransactionSync();
  const queryClient = useQueryClient();

  const mutate = async (
    params: BuyParams,
    options?: { onSuccess?: () => void; onError?: (error: Error) => void }
  ) => {
    console.log("[useBuySync] Executing buy with approve:", {
      tokenIn: params.tokenIn,
      tokenOut: params.tokenOut,
      amountOut: params.amountOut.toString(),
      maxAmountIn: params.maxAmountIn.toString(),
    });

    try {
      // Encode approve call for the DEX to spend tokenIn (the USD token)
      const approveData = encodeFunctionData({
        abi: TIP20_ABI,
        functionName: "approve",
        args: [STABLECOIN_DEX, params.maxAmountIn],
      });

      // Encode the swap call (buy exact amount out, pay up to maxAmountIn)
      const swapData = encodeFunctionData({
        abi: DEX_ABI,
        functionName: "swapExactAmountOut",
        args: [params.tokenIn, params.tokenOut, params.amountOut, params.maxAmountIn],
      });

      // Batch approve + swap in a single transaction
      const result = await sendTransactionSync({
        calls: [
          { to: params.tokenIn, data: approveData },
          { to: STABLECOIN_DEX, data: swapData },
        ],
        feeToken: params.feeToken ?? FEE_TOKEN,
      } as Parameters<typeof sendTransactionSync>[0]);

      console.log("[useBuySync] Buy success:", result);

      // Invalidate queries to refresh balances and prices
      await invalidateTradeQueries(queryClient);

      options?.onSuccess?.();
      return result;
    } catch (err) {
      console.error("[useBuySync] Buy error:", err);
      options?.onError?.(err as Error);
      throw err;
    }
  };

  return {
    mutate: (params: BuyParams, options?: { onSuccess?: () => void; onError?: (error: Error) => void }) => {
      mutate(params, options).catch(() => {}); // Fire and forget for mutate
    },
    mutateAsync: mutate,
    isPending,
    isSuccess,
    error,
    reset,
  };
}

/**
 * Hook to execute a sell order on the Tempo DEX (synchronous - waits for receipt)
 * Uses call batching to approve + swap in a single atomic transaction.
 * Supports gasless transactions via feePayer option
 */
export function useSellSync() {
  const { mutateAsync: sendTransactionSync, isPending, isSuccess, error, reset } = useSendTransactionSync();
  const queryClient = useQueryClient();

  const mutate = async (
    params: SellParams,
    options?: { onSuccess?: () => void; onError?: (error: Error) => void }
  ) => {
    console.log("[useSellSync] Executing sell with approve:", {
      tokenIn: params.tokenIn,
      tokenOut: params.tokenOut,
      amountIn: params.amountIn.toString(),
      minAmountOut: params.minAmountOut.toString(),
    });

    try {
      // Encode approve call for the DEX to spend tokenIn
      const approveData = encodeFunctionData({
        abi: TIP20_ABI,
        functionName: "approve",
        args: [STABLECOIN_DEX, params.amountIn],
      });

      // Encode the swap call
      const swapData = encodeFunctionData({
        abi: DEX_ABI,
        functionName: "swapExactAmountIn",
        args: [params.tokenIn, params.tokenOut, params.amountIn, params.minAmountOut],
      });

      // Batch approve + swap in a single transaction
      const result = await sendTransactionSync({
        calls: [
          { to: params.tokenIn, data: approveData },
          { to: STABLECOIN_DEX, data: swapData },
        ],
        feeToken: params.feeToken ?? FEE_TOKEN,
      } as Parameters<typeof sendTransactionSync>[0]);

      console.log("[useSellSync] Sell success:", result);

      // Invalidate queries to refresh balances and prices
      await invalidateTradeQueries(queryClient);

      options?.onSuccess?.();
      return result;
    } catch (err) {
      console.error("[useSellSync] Sell error:", err);
      options?.onError?.(err as Error);
      throw err;
    }
  };

  return {
    mutate: (params: SellParams, options?: { onSuccess?: () => void; onError?: (error: Error) => void }) => {
      mutate(params, options).catch(() => {}); // Fire and forget for mutate
    },
    mutateAsync: mutate,
    isPending,
    isSuccess,
    error,
    reset,
  };
}

/**
 * Hook to place a limit order on the Tempo DEX orderbook
 * Supports gasless transactions via feePayer option
 */
export function usePlaceOrderSync() {
  const place = Hooks.dex.usePlaceSync();
  const queryClient = useQueryClient();

  return {
    ...place,
    mutate: (params: PlaceOrderParams, options?: { onSuccess?: () => void; onError?: (error: Error) => void }) => {
      console.log("[usePlaceOrderSync] Placing order:", {
        token: params.token,
        amount: params.amount.toString(),
        tick: params.tick,
        type: params.type,
      });
      place.mutate(
        {
          token: params.token,
          amount: params.amount,
          tick: params.tick,
          type: params.type,
          feeToken: params.feeToken ?? FEE_TOKEN,
          feePayer: true,
        },
        {
          onSuccess: async (data) => {
            console.log("[usePlaceOrderSync] Place success:", data);
            // Invalidate queries to refresh balances and prices
            await invalidateTradeQueries(queryClient);
            options?.onSuccess?.();
          },
          onError: (error) => {
            console.error("[usePlaceOrderSync] Place error:", error);
            options?.onError?.(error);
          },
        }
      );
    },
    mutateAsync: async (params: PlaceOrderParams) => {
      console.log("[usePlaceOrderSync] Placing orderAsync:", params);
      const result = await place.mutateAsync({
        token: params.token,
        amount: params.amount,
        tick: params.tick,
        type: params.type,
        feeToken: params.feeToken ?? FEE_TOKEN,
        feePayer: true,
      });
      // Invalidate queries to refresh balances and prices
      await invalidateTradeQueries(queryClient);
      return result;
    },
  };
}
