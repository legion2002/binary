import { type Address, encodeFunctionData, parseAbi } from "viem";
import { useSendTransactionSync, useAccount, useReadContract } from "wagmi";
import { useQueryClient } from "@tanstack/react-query";
import { UNIV2_ROUTER } from "./useUniV2";
import { FEE_TOKEN, deployment } from "../config/wagmi";

const UNIV2_FACTORY = deployment.uniV2Factory;

// All UniV2 pairs are created against PATH_USD
const PATH_USD = "0x20C0000000000000000000000000000000000000" as Address;

const ROUTER_ABI = parseAbi([
  "function addLiquidity(address tokenA, address tokenB, uint amountADesired, uint amountBDesired, uint amountAMin, uint amountBMin, address to, uint deadline) external returns (uint amountA, uint amountB, uint liquidity)",
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

const TIP20_ABI = parseAbi([
  "function approve(address spender, uint256 amount) external returns (bool)",
]);

export interface PairInfo {
  pairAddress: Address | undefined;
  balance: bigint;
  totalSupply: bigint | undefined;
  reserveToken: bigint | undefined;
  reserveUsd: bigint | undefined;
  underlyingToken: bigint | undefined;
  underlyingUsd: bigint | undefined;
}

/**
 * Hook to get LP info for a single token/USD pair
 */
function usePairInfo({
  token,
  usdToken,
  account,
  enabled = true,
}: {
  token: Address | undefined;
  usdToken: Address;
  account: Address | undefined;
  enabled?: boolean;
}): PairInfo {
  const { data: pairAddress } = useReadContract({
    address: UNIV2_FACTORY,
    abi: FACTORY_ABI,
    functionName: "getPair",
    args: token ? [token, usdToken] : undefined,
    query: {
      enabled: enabled && !!token && UNIV2_FACTORY !== "0x0000000000000000000000000000000000000000",
    },
  });

  const validPair = pairAddress && pairAddress !== "0x0000000000000000000000000000000000000000";

  const { data: balance } = useReadContract({
    address: pairAddress as Address,
    abi: PAIR_ABI,
    functionName: "balanceOf",
    args: account ? [account] : undefined,
    query: { enabled: enabled && !!account && validPair },
  });

  const { data: totalSupply } = useReadContract({
    address: pairAddress as Address,
    abi: PAIR_ABI,
    functionName: "totalSupply",
    query: { enabled: enabled && validPair },
  });

  const { data: reserves } = useReadContract({
    address: pairAddress as Address,
    abi: PAIR_ABI,
    functionName: "getReserves",
    query: { enabled: enabled && validPair },
  });

  const { data: token0 } = useReadContract({
    address: pairAddress as Address,
    abi: PAIR_ABI,
    functionName: "token0",
    query: { enabled: enabled && validPair },
  });

  let reserveToken: bigint | undefined;
  let reserveUsd: bigint | undefined;
  let underlyingToken: bigint | undefined;
  let underlyingUsd: bigint | undefined;

  if (reserves && token0 && token) {
    const [reserve0, reserve1] = reserves;
    const isToken0 = token0.toLowerCase() === token.toLowerCase();
    reserveToken = isToken0 ? reserve0 : reserve1;
    reserveUsd = isToken0 ? reserve1 : reserve0;

    if (balance && totalSupply && totalSupply > 0n) {
      underlyingToken = (balance * reserveToken) / totalSupply;
      underlyingUsd = (balance * reserveUsd) / totalSupply;
    }
  }

  return {
    pairAddress: validPair ? pairAddress : undefined,
    balance: balance ?? 0n,
    totalSupply,
    reserveToken,
    reserveUsd,
    underlyingToken,
    underlyingUsd,
  };
}

/**
 * Hook to get LP info for both YES/PATH_USD and NO/PATH_USD pairs
 * Note: All UniV2 pairs are created against PATH_USD, not the user's selected stablecoin
 */
export function useMarketLiquidity({
  yesTokenAddress,
  noTokenAddress,
  account,
  enabled = true,
}: {
  yesTokenAddress: Address | undefined;
  noTokenAddress: Address | undefined;
  account: Address | undefined;
  enabled?: boolean;
}) {
  // Always use PATH_USD since that's what pairs are created against
  const yesPair = usePairInfo({
    token: yesTokenAddress,
    usdToken: PATH_USD,
    account,
    enabled: enabled && !!yesTokenAddress,
  });

  const noPair = usePairInfo({
    token: noTokenAddress,
    usdToken: PATH_USD,
    account,
    enabled: enabled && !!noTokenAddress,
  });

  // Calculate prices from reserves
  const yesPrice = yesPair.reserveToken && yesPair.reserveUsd && yesPair.reserveToken > 0n
    ? Number(yesPair.reserveUsd) / Number(yesPair.reserveToken)
    : null;
  
  const noPrice = noPair.reserveToken && noPair.reserveUsd && noPair.reserveToken > 0n
    ? Number(noPair.reserveUsd) / Number(noPair.reserveToken)
    : null;

  return {
    yesPair,
    noPair,
    yesPrice,
    noPrice,
    hasYesLiquidity: yesPair.balance > 0n,
    hasNoLiquidity: noPair.balance > 0n,
    hasAnyLiquidity: yesPair.balance > 0n || noPair.balance > 0n,
  };
}

export interface AddLiquidityParams {
  token: Address;
  amountToken: bigint;
  amountUsd: bigint;
  slippageBps?: number;
}

/**
 * Hook to add liquidity to a single token/PATH_USD pair
 * Note: All pairs use PATH_USD as the quote token
 */
export function useAddLiquidityToPair() {
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

    const slippage = params.slippageBps ?? 100;
    const amountTokenMin = (params.amountToken * BigInt(10000 - slippage)) / 10000n;
    const amountUsdMin = (params.amountUsd * BigInt(10000 - slippage)) / 10000n;
    const deadline = BigInt(Math.floor(Date.now() / 1000) + 1800);

    console.log("[useAddLiquidityToPair] Adding liquidity:", {
      token: params.token,
      usdToken: PATH_USD,
      amountToken: params.amountToken.toString(),
      amountUsd: params.amountUsd.toString(),
    });

    try {
      const approveTokenData = encodeFunctionData({
        abi: TIP20_ABI,
        functionName: "approve",
        args: [UNIV2_ROUTER, params.amountToken],
      });

      const approveUsdData = encodeFunctionData({
        abi: TIP20_ABI,
        functionName: "approve",
        args: [UNIV2_ROUTER, params.amountUsd],
      });

      const addLiquidityData = encodeFunctionData({
        abi: ROUTER_ABI,
        functionName: "addLiquidity",
        args: [
          params.token,
          PATH_USD,
          params.amountToken,
          params.amountUsd,
          amountTokenMin,
          amountUsdMin,
          account,
          deadline,
        ],
      });

      const result = await sendTransactionSync({
        calls: [
          { to: params.token, data: approveTokenData },
          { to: PATH_USD, data: approveUsdData },
          { to: UNIV2_ROUTER, data: addLiquidityData },
        ],
        feeToken: FEE_TOKEN,
      } as Parameters<typeof sendTransactionSync>[0]);

      console.log("[useAddLiquidityToPair] Success:", result);
      await queryClient.invalidateQueries();
      options?.onSuccess?.();
      return result;
    } catch (err) {
      console.error("[useAddLiquidityToPair] Error:", err);
      options?.onError?.(err as Error);
      throw err;
    }
  };

  return { addLiquidity, isPending, isSuccess, error, reset };
}

export interface RemoveLiquidityParams {
  token: Address;
  pairAddress: Address;
  liquidity: bigint;
  slippageBps?: number;
}

/**
 * Hook to remove liquidity from a single token/PATH_USD pair
 * Note: All pairs use PATH_USD as the quote token
 */
export function useRemoveLiquidityFromPair() {
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
    params: RemoveLiquidityParams,
    options?: { onSuccess?: () => void; onError?: (error: Error) => void }
  ) => {
    if (!account) {
      const err = new Error("No account connected");
      options?.onError?.(err);
      throw err;
    }

    const deadline = BigInt(Math.floor(Date.now() / 1000) + 1800);

    console.log("[useRemoveLiquidityFromPair] Removing liquidity:", {
      token: params.token,
      usdToken: PATH_USD,
      liquidity: params.liquidity.toString(),
    });

    try {
      const approveData = encodeFunctionData({
        abi: PAIR_ABI,
        functionName: "approve",
        args: [UNIV2_ROUTER, params.liquidity],
      });

      const removeLiquidityData = encodeFunctionData({
        abi: ROUTER_ABI,
        functionName: "removeLiquidity",
        args: [
          params.token,
          PATH_USD,
          params.liquidity,
          0n, // amountTokenMin
          0n, // amountUsdMin
          account,
          deadline,
        ],
      });

      const result = await sendTransactionSync({
        calls: [
          { to: params.pairAddress, data: approveData },
          { to: UNIV2_ROUTER, data: removeLiquidityData },
        ],
        feeToken: FEE_TOKEN,
      } as Parameters<typeof sendTransactionSync>[0]);

      console.log("[useRemoveLiquidityFromPair] Success:", result);
      await queryClient.invalidateQueries();
      options?.onSuccess?.();
      return result;
    } catch (err) {
      console.error("[useRemoveLiquidityFromPair] Error:", err);
      options?.onError?.(err as Error);
      throw err;
    }
  };

  return { removeLiquidity, isPending, isSuccess, error, reset };
}
