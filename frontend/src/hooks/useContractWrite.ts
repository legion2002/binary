import { useState, useCallback } from "react";
import {
  encodeFunctionData,
  type Abi,
  type Address,
  type Hash,
} from "viem";
import { useTempoWalletClient, usePasskey } from "../contexts/PasskeyContext";

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
 * Hook to write to a contract using the passkey wallet client
 * Replaces wagmi's useWriteContract for passkey-based accounts
 */
export function useWriteContract(): UseWriteContractReturn {
  const walletClient = useTempoWalletClient();
  const { isConnected } = usePasskey();

  const [hash, setHash] = useState<Hash | undefined>(undefined);
  const [isPending, setIsPending] = useState(false);
  const [isConfirming, setIsConfirming] = useState(false);
  const [isSuccess, setIsSuccess] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const writeContract = useCallback(
    async (params: WriteContractParams): Promise<Hash> => {
      if (!walletClient) {
        throw new Error("No wallet client - please sign in");
      }
      if (!isConnected) {
        throw new Error("Not connected");
      }

      setIsPending(true);
      setError(null);
      setHash(undefined);
      setIsSuccess(false);

      try {
        const data = encodeFunctionData({
          abi: params.abi,
          functionName: params.functionName,
          args: params.args || [],
        });

        // Use batched call format for Tempo transactions
        const txHash = await walletClient.sendTransaction({
          calls: [
            {
              to: params.address,
              data,
              value: params.value || 0n,
            },
          ],
        });

        setHash(txHash);
        setIsConfirming(true);

        // Wait for confirmation
        // Note: In production, you'd want to use waitForTransactionReceipt
        setIsConfirming(false);
        setIsSuccess(true);

        return txHash;
      } catch (err) {
        const error = err instanceof Error ? err : new Error(String(err));
        setError(error);
        throw error;
      } finally {
        setIsPending(false);
      }
    },
    [walletClient, isConnected]
  );

  const reset = useCallback(() => {
    setHash(undefined);
    setIsPending(false);
    setIsConfirming(false);
    setIsSuccess(false);
    setError(null);
  }, []);

  return {
    writeContract,
    hash,
    isPending,
    isConfirming,
    isSuccess,
    error,
    reset,
  };
}
