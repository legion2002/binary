import { type Address, encodeFunctionData, zeroAddress } from "viem";
import { useSendTransactionSync, usePublicClient } from "wagmi";
import { CONTRACTS, MULTIVERSE_ABI, TIP20_ABI } from "../config/contracts";

interface SplitParams {
  asset: Address;
  amount: bigint;
  marketHash: string;
}

export function useSplit() {
  const { mutateAsync: sendTransactionSync, isPending, isSuccess, error, reset } = useSendTransactionSync();
  const publicClient = usePublicClient();

  const split = async ({ asset, amount, marketHash }: SplitParams) => {
    console.log("[useSplit] split called with:", { asset, amount: amount.toString(), marketHash });

    const marketHashBytes = marketHash as `0x${string}`;

    // Check if verse tokens already exist for this asset
    let needsCreate = false;
    if (publicClient) {
      try {
        const [yesVerse] = await publicClient.readContract({
          address: CONTRACTS.MULTIVERSE as Address,
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
      args: [CONTRACTS.MULTIVERSE as Address, amount],
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
        to: CONTRACTS.MULTIVERSE as Address,
        data: createData,
      });
    }

    calls.push({
      to: CONTRACTS.MULTIVERSE as Address,
      data: splitData,
    });

    console.log("[useSplit] Sending calls:", calls);

    try {
      // Use wagmi's useSendTransactionSync with Tempo properties (calls, feeToken)
      // Use feeToken instead of feePayer for local devnet without fee payer relay
      const receipt = await sendTransactionSync({
        calls,
        feeToken: CONTRACTS.PATH_USD,
      } as Parameters<typeof sendTransactionSync>[0]);
      
      console.log("[useSplit] Transaction receipt:", receipt);
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
  };
}
