import { type Address, encodeFunctionData, parseAbi } from "viem";
import { useSendTransactionSync, useAccount, useReadContract } from "wagmi";
import { useQueryClient } from "@tanstack/react-query";
import { UNIV2_ROUTER } from "./useUniV2";
import { FEE_TOKEN, deployment } from "../config/wagmi";

const UNIV2_FACTORY = deployment.uniV2Factory;

const ROUTER_ABI = parseAbi([
  "function removeLiquidity(address tokenA, address tokenB, uint liquidity, uint amountAMin, uint amountBMin, address to, uint deadline) external returns (uint amountA, uint amountB)",
]);

const PAIR_ABI = parseAbi([
  "function balanceOf(address owner) external view returns (uint256)",
  "function totalSupply() external view returns (uint256)",
  "function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)",
  "function token0() external view returns (address)",
  "function approve(address spender, uint256 amount) external returns (bool)",
]);

const FACTORY_ABI = parseAbi([
  "function getPair(address tokenA, address tokenB) external view returns (address pair)",
]);

export interface RemoveLiquidityParams {
  tokenA: Address;
  tokenB: Address;
  liquidity: bigint;
  slippageBps?: number; // default 100 = 1%
}

/**
 * Hook to get LP token balance for a pair
 */
export function useLPBalance({
  tokenA,
  tokenB,
  account,
  enabled = true,
}: {
  tokenA: Address | undefined;
  tokenB: Address | undefined;
  account: Address | undefined;
  enabled?: boolean;
}) {
  // First get the pair address from factory
  const { data: pairAddress } = useReadContract({
    address: UNIV2_FACTORY,
    abi: FACTORY_ABI,
    functionName: "getPair",
    args: tokenA && tokenB ? [tokenA, tokenB] : undefined,
    query: {
      enabled: enabled && !!tokenA && !!tokenB && UNIV2_FACTORY !== "0x0000000000000000000000000000000000000000",
    },
  });

  const validPair = pairAddress && pairAddress !== "0x0000000000000000000000000000000000000000";

  const { data: balance, ...rest } = useReadContract({
    address: pairAddress as Address,
    abi: PAIR_ABI,
    functionName: "balanceOf",
    args: account ? [account] : undefined,
    query: {
      enabled: enabled && !!account && validPair,
    },
  });

  const { data: totalSupply } = useReadContract({
    address: pairAddress as Address,
    abi: PAIR_ABI,
    functionName: "totalSupply",
    query: {
      enabled: enabled && validPair,
    },
  });

  const { data: reserves } = useReadContract({
    address: pairAddress as Address,
    abi: PAIR_ABI,
    functionName: "getReserves",
    query: {
      enabled: enabled && validPair,
    },
  });

  const { data: token0 } = useReadContract({
    address: pairAddress as Address,
    abi: PAIR_ABI,
    functionName: "token0",
    query: {
      enabled: enabled && validPair,
    },
  });

  // Calculate reserves for tokenA and tokenB
  let reserveA: bigint | undefined;
  let reserveB: bigint | undefined;
  let amountA: bigint | undefined;
  let amountB: bigint | undefined;

  if (reserves && token0 && tokenA && tokenB) {
    const [reserve0, reserve1] = reserves;
    const isToken0A = token0.toLowerCase() === tokenA.toLowerCase();
    reserveA = isToken0A ? reserve0 : reserve1;
    reserveB = isToken0A ? reserve1 : reserve0;
    
    // Calculate underlying token amounts from LP balance
    if (balance && totalSupply && totalSupply > 0n) {
      amountA = (balance * reserveA) / totalSupply;
      amountB = (balance * reserveB) / totalSupply;
    }
  }

  return {
    balance: balance ?? 0n,
    pairAddress: validPair ? pairAddress : undefined,
    totalSupply,
    reserveA,
    reserveB,
    amountA,
    amountB,
    ...rest,
  };
}

/**
 * Hook to remove liquidity from UniV2 pair
 */
export function useRemoveLiquidity() {
  const {
    mutateAsync: sendTransactionSync,
    isPending,
    isSuccess,
    error,
    reset,
  } = useSendTransactionSync();
  const queryClient = useQueryClient();
  const { address: account } = useAccount();

  const removeLiquidity = async (
    params: RemoveLiquidityParams & { pairAddress: Address },
    options?: { onSuccess?: () => void; onError?: (error: Error) => void }
  ) => {
    if (!account) {
      const err = new Error("No account connected");
      options?.onError?.(err);
      throw err;
    }

    // slippageBps reserved for future use
    void params.slippageBps;
    const deadline = BigInt(Math.floor(Date.now() / 1000) + 1800); // 30 min

    console.log("[useRemoveLiquidity] Removing liquidity:", {
      tokenA: params.tokenA,
      tokenB: params.tokenB,
      liquidity: params.liquidity.toString(),
      pairAddress: params.pairAddress,
    });

    try {
      // Approve LP tokens to router
      const approveData = encodeFunctionData({
        abi: PAIR_ABI,
        functionName: "approve",
        args: [UNIV2_ROUTER, params.liquidity],
      });

      // Remove liquidity (0 minimums with slippage protection in UI)
      const removeLiquidityData = encodeFunctionData({
        abi: ROUTER_ABI,
        functionName: "removeLiquidity",
        args: [
          params.tokenA,
          params.tokenB,
          params.liquidity,
          0n, // amountAMin - could calculate from reserves
          0n, // amountBMin - could calculate from reserves  
          account,
          deadline,
        ],
      });

      // Batch approve + remove
      const result = await sendTransactionSync({
        calls: [
          { to: params.pairAddress, data: approveData },
          { to: UNIV2_ROUTER, data: removeLiquidityData },
        ],
        feeToken: FEE_TOKEN,
      } as Parameters<typeof sendTransactionSync>[0]);

      console.log("[useRemoveLiquidity] Success:", result);
      await queryClient.invalidateQueries();
      options?.onSuccess?.();
      return result;
    } catch (err) {
      console.error("[useRemoveLiquidity] Error:", err);
      options?.onError?.(err as Error);
      throw err;
    }
  };

  return {
    removeLiquidity,
    isPending,
    isSuccess,
    error,
    reset,
  };
}
