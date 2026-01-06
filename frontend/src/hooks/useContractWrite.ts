import { useCallback } from "react";
import { type Abi, type Address, type Hash } from "viem";
import {
  useAccount,
  useWriteContract as useWagmiWriteContract,
} from "wagmi";

interface WriteContractParams {
  address: Address;
  abi: Abi;
  functionName: string;
  args?: unknown[];
  value?: bigint;
}

interface UseWriteContractReturn {
  writeContract: (params: WriteContractParams) => Promise<Hash>;
  hash: Hash | undefined;
  isPending: boolean;
  isConfirming: boolean;
  isSuccess: boolean;
  error: Error | null;
  reset: () => void;
}

/**
 * Hook to write to a contract using wagmi's native writeContract
 */
export function useWriteContract(): UseWriteContractReturn {
  const { isConnected } = useAccount();
  const {
    writeContractAsync,
    data: hash,
    isPending,
    isSuccess,
    error,
    reset,
  } = useWagmiWriteContract();

  const writeContract = useCallback(
    async (params: WriteContractParams): Promise<Hash> => {
      if (!isConnected) {
        throw new Error("Not connected");
      }

      return writeContractAsync({
        address: params.address,
        abi: params.abi,
        functionName: params.functionName,
        args: params.args || [],
        value: params.value,
      });
    },
    [isConnected, writeContractAsync]
  );

  return {
    writeContract,
    hash,
    isPending,
    isConfirming: false,
    isSuccess,
    error: error as Error | null,
    reset,
  };
}
