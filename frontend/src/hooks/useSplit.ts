import { type Address, encodeFunctionData, zeroAddress } from "viem";
import { useSendTransactionSync, usePublicClient } from "wagmi";
import { useQueryClient } from "@tanstack/react-query";
import { useContracts, MULTIVERSE_ABI, TIP20_ABI } from "./useContracts";
import { verseTokensQueryKey } from "./useVerseTokens";

interface SplitParams {
  asset: Address;
  amount: bigint;
  marketHash: string;
}

export function useSplit() {
  const { mutateAsync: sendTransactionSync, isPending, isSuccess, error, reset } = useSendTransactionSync();
  const publicClient = usePublicClient();
  const queryClient = useQueryClient();
  const { contracts, isLoading: contractsLoading } = useContracts();

  const split = async ({ asset, amount, marketHash }: SplitParams) => {
    if (!contracts) {
      throw new Error("Contracts not loaded yet");
    }

    console.log("[useSplit] split called with:", { asset, amount: amount.toString(), marketHash });

    const marketHashBytes = marketHash as `0x${string}`;

    // Check if verse tokens already exist for this asset
    let needsCreate = false;
    if (publicClient) {
      try {
        const [yesVerse] = await publicClient.readContract({
          address: contracts.MULTIVERSE,
          abi: MULTIVERSE_ABI,
          functionName: "getVerseAddress",
          args: [asset, marketHashBytes],
        });
        needsCreate = yesVerse === zeroAddress;
        console.log("[useSplit] getVerseAddress result:", { yesVerse, needsCreate });
      } catch (err) {
        console.log("[useSplit] getVerseAddress failed, will create:", err);
        needsCreate = true;
      }
    }

    const approveData = encodeFunctionData({
      abi: TIP20_ABI,
      functionName: "approve",
      args: [contracts.MULTIVERSE, amount],
    });

    const createData = encodeFunctionData({
      abi: MULTIVERSE_ABI,
      functionName: "create",
      args: [asset, marketHashBytes],
    });

    const splitData = encodeFunctionData({
      abi: MULTIVERSE_ABI,
      functionName: "split",
      args: [asset, amount, marketHashBytes],
    });

    const calls: { to: Address; data: `0x${string}` }[] = [
      {
        to: asset,
        data: approveData,
      },
    ];

    if (needsCreate) {
      calls.push({
        to: contracts.MULTIVERSE,
        data: createData,
      });
    }

    calls.push({
      to: contracts.MULTIVERSE,
      data: splitData,
    });

    console.log("[useSplit] Sending calls:", calls);

    try {
      // Use wagmi's useSendTransactionSync with Tempo properties (calls, feeToken)
      // Use feeToken instead of feePayer for local devnet without fee payer relay
      const receipt = await sendTransactionSync({
        calls,
        feeToken: contracts.PATH_USD,
      } as Parameters<typeof sendTransactionSync>[0]);

      console.log("[useSplit] Transaction receipt:", receipt);

      // Invalidate verse tokens query so YES/NO balances update
      await queryClient.invalidateQueries({
        queryKey: verseTokensQueryKey(asset, marketHash),
      });

      // Invalidate all token balance queries so stablecoin balance updates
      // wagmi/tempo uses query keys containing the token address
      await queryClient.invalidateQueries({
        predicate: (query) => {
          const key = query.queryKey;
          // Match wagmi token balance queries (they contain 'getBalance' or the token address)
          return (
            Array.isArray(key) &&
            key.some(
              (k) =>
                k === "getBalance" ||
                (typeof k === "object" && k !== null && "token" in k)
            )
          );
        },
      });

      return receipt;
    } catch (err) {
      console.error("[useSplit] sendTransactionSync error:", err);
      throw err;
    }
  };

  return {
    split,
    isPending,
    isSuccess,
    error,
    reset,
    contractsLoading,
  };
}
