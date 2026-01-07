import { type Address, encodeFunctionData } from "viem";
import { useSendTransaction } from "wagmi";
import { CONTRACTS, MULTIVERSE_ABI, TIP20_ABI } from "../config/contracts";

interface SplitParams {
  amount: bigint;
  marketHash: string;
}

export function useSplit() {
  const { sendTransaction, isPending, isSuccess, error, reset } = useSendTransaction();

  const split = async ({ amount, marketHash }: SplitParams) => {
    const approveData = encodeFunctionData({
      abi: TIP20_ABI,
      functionName: "approve",
      args: [CONTRACTS.MULTIVERSE as Address, amount],
    });

    const splitData = encodeFunctionData({
      abi: MULTIVERSE_ABI,
      functionName: "split",
      args: [CONTRACTS.USD as Address, amount, marketHash as `0x${string}`],
    });

    return sendTransaction({
      calls: [
        {
          to: CONTRACTS.USD as Address,
          data: approveData,
        },
        {
          to: CONTRACTS.MULTIVERSE as Address,
          data: splitData,
        },
      ],
      feePayer: true,
    } as any);
  };

  return {
    split,
    isPending,
    isSuccess,
    error,
    reset,
  };
}
