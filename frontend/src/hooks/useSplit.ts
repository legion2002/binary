import { type Address, encodeFunctionData } from "viem";
import { useSendTransaction } from "wagmi";
import { CONTRACTS, MULTIVERSE_ABI, TIP20_ABI } from "../config/contracts";

interface SplitParams {
  asset: Address;
  amount: bigint;
  marketHash: string;
}

export function useSplit() {
  const { sendTransaction, isPending, isSuccess, error, reset } = useSendTransaction();

  const split = async ({ asset, amount, marketHash }: SplitParams) => {
    const approveData = encodeFunctionData({
      abi: TIP20_ABI,
      functionName: "approve",
      args: [CONTRACTS.MULTIVERSE as Address, amount],
    });

    const splitData = encodeFunctionData({
      abi: MULTIVERSE_ABI,
      functionName: "split",
      args: [asset, amount, marketHash as `0x${string}`],
    });

    return sendTransaction({
      calls: [
        {
          to: asset,
          data: approveData,
        },
        {
          to: CONTRACTS.MULTIVERSE as Address,
          data: splitData,
        },
      ],
      feePayer: true,
    } as Parameters<typeof sendTransaction>[0]);
  };

  return {
    split,
    isPending,
    isSuccess,
    error,
    reset,
  };
}
