import { type Address, encodeFunctionData, parseAbi } from "viem";
import { useSendTransactionSync, useAccount } from "wagmi";
import { useQueryClient } from "@tanstack/react-query";
import { UNIV2_ROUTER } from "./useUniV2";
import { FEE_TOKEN } from "../config/wagmi";

const ROUTER_ABI = parseAbi([
  "function addLiquidity(address tokenA, address tokenB, uint amountADesired, uint amountBDesired, uint amountAMin, uint amountBMin, address to, uint deadline) external returns (uint amountA, uint amountB, uint liquidity)",
]);

const TIP20_ABI = parseAbi([
  "function approve(address spender, uint256 amount) external returns (bool)",
]);

export interface AddLiquidityParams {
  tokenA: Address;
  tokenB: Address;
  amountA: bigint;
  amountB: bigint;
  slippageBps?: number; // default 100 = 1%
}

export function useAddLiquidity() {
  const {
    mutateAsync: sendTransactionSync,
    isPending,
    isSuccess,
    error,
    reset,
  } = useSendTransactionSync();
  const queryClient = useQueryClient();
  const { address: account } = useAccount();

  const addLiquidity = async (
    params: AddLiquidityParams,
    options?: { onSuccess?: () => void; onError?: (error: Error) => void }
  ) => {
    if (!account) {
      const err = new Error("No account connected");
      options?.onError?.(err);
      throw err;
    }

    const slippage = params.slippageBps ?? 100; // 1% default
    const amountAMin = (params.amountA * BigInt(10000 - slippage)) / 10000n;
    const amountBMin = (params.amountB * BigInt(10000 - slippage)) / 10000n;
    const deadline = BigInt(Math.floor(Date.now() / 1000) + 1800); // 30 min

    console.log("[useAddLiquidity] Adding liquidity:", {
      tokenA: params.tokenA,
      tokenB: params.tokenB,
      amountA: params.amountA.toString(),
      amountB: params.amountB.toString(),
    });

    try {
      // Approve tokenA
      const approveAData = encodeFunctionData({
        abi: TIP20_ABI,
        functionName: "approve",
        args: [UNIV2_ROUTER, params.amountA],
      });

      // Approve tokenB
      const approveBData = encodeFunctionData({
        abi: TIP20_ABI,
        functionName: "approve",
        args: [UNIV2_ROUTER, params.amountB],
      });

      // Add liquidity
      const addLiquidityData = encodeFunctionData({
        abi: ROUTER_ABI,
        functionName: "addLiquidity",
        args: [
          params.tokenA,
          params.tokenB,
          params.amountA,
          params.amountB,
          amountAMin,
          amountBMin,
          account,
          deadline,
        ],
      });

      // Batch all calls
      const result = await sendTransactionSync({
        calls: [
          { to: params.tokenA, data: approveAData },
          { to: params.tokenB, data: approveBData },
          { to: UNIV2_ROUTER, data: addLiquidityData },
        ],
        feeToken: FEE_TOKEN,
      } as Parameters<typeof sendTransactionSync>[0]);

      console.log("[useAddLiquidity] Success:", result);
      await queryClient.invalidateQueries();
      options?.onSuccess?.();
      return result;
    } catch (err) {
      console.error("[useAddLiquidity] Error:", err);
      options?.onError?.(err as Error);
      throw err;
    }
  };

  return {
    addLiquidity,
    isPending,
    isSuccess,
    error,
    reset,
  };
}
