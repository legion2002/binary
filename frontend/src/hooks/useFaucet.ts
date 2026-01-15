import { useState, useCallback } from "react";
import {
  createClient,
  http,
  publicActions,
  walletActions,
  encodeFunctionData,
  type Address,
} from "viem";
import { useAccount } from "wagmi";
import { useQueryClient } from "@tanstack/react-query";
import { tempoChain, TEMPO_RPC_URL, PATH_USD } from "../config/wagmi";

const TEST_PRIVATE_KEY =
  "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" as const;

interface FundParams {
  token: Address;
  amount: bigint;
}

export function useFaucet() {
  const { address } = useAccount();
  const queryClient = useQueryClient();
  const [isPending, setIsPending] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const fund = useCallback(
    async (params: FundParams) => {
      if (!address) {
        throw new Error("No wallet connected");
      }

      setIsPending(true);
      setError(null);

      try {
        const { privateKeyToAccount } = await import("viem/accounts");
        const testAccount = privateKeyToAccount(TEST_PRIVATE_KEY);

        const client = createClient({
          account: testAccount,
          chain: tempoChain,
          transport: http(TEMPO_RPC_URL),
        })
          .extend(publicActions)
          .extend(walletActions);

        const hash = await client.sendTransaction({
          to: params.token,
          data: encodeFunctionData({
            abi: [
              {
                type: "function",
                name: "transfer",
                inputs: [
                  { name: "to", type: "address" },
                  { name: "amount", type: "uint256" },
                ],
                outputs: [{ type: "bool" }],
                stateMutability: "nonpayable",
              },
            ],
            functionName: "transfer",
            args: [address, params.amount],
          }),
          feeToken: PATH_USD,
        } as Parameters<typeof client.sendTransaction>[0]);

        await client.waitForTransactionReceipt({ hash });

        // Invalidate balance queries to refresh balances
        queryClient.invalidateQueries({ queryKey: ["token"] });

        return hash;
      } catch (err) {
        const error = err instanceof Error ? err : new Error(String(err));
        setError(error);
        throw error;
      } finally {
        setIsPending(false);
      }
    },
    [address, queryClient]
  );

  return {
    fund,
    isPending,
    error,
  };
}
