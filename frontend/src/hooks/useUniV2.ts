import { type Address, encodeFunctionData, parseAbi } from "viem";
import { useReadContract, useSendTransactionSync, useAccount } from "wagmi";
import { useQueryClient } from "@tanstack/react-query";
import { FEE_TOKEN } from "../config/wagmi";

// UniV2 Router address (configured via env)
export const UNIV2_ROUTER = (import.meta.env.VITE_UNIV2_ROUTER_ADDRESS ??
  "0x0000000000000000000000000000000000000000") as Address;

// PATH_USD is used as the routing token for multi-hop swaps
const PATH_USD = "0x20C0000000000000000000000000000000000000" as Address;

/**
 * Build swap path - routes through PATH_USD if neither token is PATH_USD
 */
function buildSwapPath(tokenIn: Address, tokenOut: Address): Address[] {
  // If either token is PATH_USD, direct path works
  if (tokenIn.toLowerCase() === PATH_USD.toLowerCase() || 
      tokenOut.toLowerCase() === PATH_USD.toLowerCase()) {
    return [tokenIn, tokenOut];
  }
  // Otherwise route through PATH_USD
  return [tokenIn, PATH_USD, tokenOut];
}

// Router ABI (minimal - just what we need)
const ROUTER_ABI = parseAbi([
  "function swapExactTokensForTokens(uint amountIn, uint amountOutMin, address[] calldata path, address to, uint deadline) external returns (uint[] memory amounts)",
  "function swapTokensForExactTokens(uint amountOut, uint amountInMax, address[] calldata path, address to, uint deadline) external returns (uint[] memory amounts)",
  "function getAmountsOut(uint amountIn, address[] calldata path) external view returns (uint[] memory amounts)",
  "function getAmountsIn(uint amountOut, address[] calldata path) external view returns (uint[] memory amounts)",
  "function addLiquidity(address tokenA, address tokenB, uint amountADesired, uint amountBDesired, uint amountAMin, uint amountBMin, address to, uint deadline) external returns (uint amountA, uint amountB, uint liquidity)",
]);

const TIP20_ABI = parseAbi([
  "function approve(address spender, uint256 amount) external returns (bool)",
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

async function invalidateTradeQueries(
  queryClient: ReturnType<typeof useQueryClient>
) {
  console.log(
    "[invalidateTradeQueries] Invalidating all trade-related queries..."
  );
  await queryClient.invalidateQueries();
  console.log("[invalidateTradeQueries] All queries invalidated");
}

/**
 * Hook to get a buy quote from UniV2 Router
 * Returns the amount of tokenIn needed to buy amountOut of tokenOut
 */
export function useUniV2BuyQuote({
  tokenIn,
  tokenOut,
  amountOut,
  query,
}: BuyQuoteParams) {
  const path = buildSwapPath(tokenIn, tokenOut);
  const result = useReadContract({
    address: UNIV2_ROUTER,
    abi: ROUTER_ABI,
    functionName: "getAmountsIn",
    args: [amountOut, path],
    query: {
      enabled:
        query?.enabled !== false &&
        amountOut > 0n &&
        UNIV2_ROUTER !== "0x0000000000000000000000000000000000000000",
    },
  });

  // Extract the first element (amountIn) from the amounts array
  const data = result.data ? result.data[0] : undefined;

  return {
    ...result,
    data,
  };
}

/**
 * Hook to get a sell quote from UniV2 Router
 * Returns the amount of tokenOut received for selling amountIn of tokenIn
 */
export function useUniV2SellQuote({
  tokenIn,
  tokenOut,
  amountIn,
  query,
}: SellQuoteParams) {
  const path = buildSwapPath(tokenIn, tokenOut);
  const result = useReadContract({
    address: UNIV2_ROUTER,
    abi: ROUTER_ABI,
    functionName: "getAmountsOut",
    args: [amountIn, path],
    query: {
      enabled:
        query?.enabled !== false &&
        amountIn > 0n &&
        UNIV2_ROUTER !== "0x0000000000000000000000000000000000000000",
    },
  });

  // Extract the last element (amountOut) from the amounts array
  const data = result.data ? result.data[result.data.length - 1] : undefined;

  return {
    ...result,
    data,
  };
}

/**
 * Hook to execute a buy order via UniV2 Router (synchronous - waits for receipt)
 * Uses call batching to approve + swap in a single atomic transaction.
 */
export function useUniV2BuySync() {
  const {
    mutateAsync: sendTransactionSync,
    isPending,
    isSuccess,
    error,
    reset,
  } = useSendTransactionSync();
  const queryClient = useQueryClient();
  const { address: account } = useAccount();

  const mutate = async (
    params: BuyParams,
    options?: { onSuccess?: () => void; onError?: (error: Error) => void }
  ) => {
    if (!account) {
      const err = new Error("No account connected");
      options?.onError?.(err);
      throw err;
    }

    console.log("[useUniV2BuySync] Executing buy with approve:", {
      tokenIn: params.tokenIn,
      tokenOut: params.tokenOut,
      amountOut: params.amountOut.toString(),
      maxAmountIn: params.maxAmountIn.toString(),
    });

    try {
      const deadline = BigInt(Math.floor(Date.now() / 1000) + 1800); // 30 min

      // Encode approve call for the Router to spend tokenIn
      const approveData = encodeFunctionData({
        abi: TIP20_ABI,
        functionName: "approve",
        args: [UNIV2_ROUTER, params.maxAmountIn],
      });

      // Build swap path (routes through PATH_USD if needed)
      const path = buildSwapPath(params.tokenIn, params.tokenOut);

      // Encode the swap call (buy exact amount out, pay up to maxAmountIn)
      const swapData = encodeFunctionData({
        abi: ROUTER_ABI,
        functionName: "swapTokensForExactTokens",
        args: [
          params.amountOut,
          params.maxAmountIn,
          path,
          account,
          deadline,
        ],
      });

      // Batch approve + swap in a single transaction
      const result = await sendTransactionSync({
        calls: [
          { to: params.tokenIn, data: approveData },
          { to: UNIV2_ROUTER, data: swapData },
        ],
        feeToken: params.feeToken ?? FEE_TOKEN,
      } as Parameters<typeof sendTransactionSync>[0]);

      console.log("[useUniV2BuySync] Buy success:", result);

      // Invalidate queries to refresh balances and prices
      await invalidateTradeQueries(queryClient);

      options?.onSuccess?.();
      return result;
    } catch (err) {
      console.error("[useUniV2BuySync] Buy error:", err);
      options?.onError?.(err as Error);
      throw err;
    }
  };

  return {
    mutate: (
      params: BuyParams,
      options?: { onSuccess?: () => void; onError?: (error: Error) => void }
    ) => {
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
 * Hook to execute a sell order via UniV2 Router (synchronous - waits for receipt)
 * Uses call batching to approve + swap in a single atomic transaction.
 */
export function useUniV2SellSync() {
  const {
    mutateAsync: sendTransactionSync,
    isPending,
    isSuccess,
    error,
    reset,
  } = useSendTransactionSync();
  const queryClient = useQueryClient();
  const { address: account } = useAccount();

  const mutate = async (
    params: SellParams,
    options?: { onSuccess?: () => void; onError?: (error: Error) => void }
  ) => {
    if (!account) {
      const err = new Error("No account connected");
      options?.onError?.(err);
      throw err;
    }

    console.log("[useUniV2SellSync] Executing sell with approve:", {
      tokenIn: params.tokenIn,
      tokenOut: params.tokenOut,
      amountIn: params.amountIn.toString(),
      minAmountOut: params.minAmountOut.toString(),
    });

    try {
      const deadline = BigInt(Math.floor(Date.now() / 1000) + 1800); // 30 min

      // Encode approve call for the Router to spend tokenIn
      const approveData = encodeFunctionData({
        abi: TIP20_ABI,
        functionName: "approve",
        args: [UNIV2_ROUTER, params.amountIn],
      });

      // Build swap path (routes through PATH_USD if needed)
      const path = buildSwapPath(params.tokenIn, params.tokenOut);

      // Encode the swap call (sell exact amount in, receive at least minAmountOut)
      const swapData = encodeFunctionData({
        abi: ROUTER_ABI,
        functionName: "swapExactTokensForTokens",
        args: [
          params.amountIn,
          params.minAmountOut,
          path,
          account,
          deadline,
        ],
      });

      // Batch approve + swap in a single transaction
      const result = await sendTransactionSync({
        calls: [
          { to: params.tokenIn, data: approveData },
          { to: UNIV2_ROUTER, data: swapData },
        ],
        feeToken: params.feeToken ?? FEE_TOKEN,
      } as Parameters<typeof sendTransactionSync>[0]);

      console.log("[useUniV2SellSync] Sell success:", result);

      // Invalidate queries to refresh balances and prices
      await invalidateTradeQueries(queryClient);

      options?.onSuccess?.();
      return result;
    } catch (err) {
      console.error("[useUniV2SellSync] Sell error:", err);
      options?.onError?.(err as Error);
      throw err;
    }
  };

  return {
    mutate: (
      params: SellParams,
      options?: { onSuccess?: () => void; onError?: (error: Error) => void }
    ) => {
      mutate(params, options).catch(() => {}); // Fire and forget for mutate
    },
    mutateAsync: mutate,
    isPending,
    isSuccess,
    error,
    reset,
  };
}

// Re-export types for consumers
export type { BuyQuoteParams, SellQuoteParams, BuyParams, SellParams };
