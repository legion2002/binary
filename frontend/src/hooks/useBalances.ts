import { useQuery } from "@tanstack/react-query";
import { usePublicClient } from "wagmi";
import { Actions, tempoActions } from "viem/tempo";
import type { Address } from "viem";

/**
 * Hook to get a token balance using viem/tempo
 */
export function useTokenBalance({
  account,
  token,
  enabled = true,
}: {
  account: Address | undefined;
  token: Address;
  enabled?: boolean;
}) {
  const publicClient = usePublicClient();

  return useQuery({
    queryKey: ["tempo", "token", "balance", account, token],
    queryFn: async () => {
      if (!publicClient) throw new Error("No public client");
      if (!account) throw new Error("No account");
      const client = publicClient.extend(tempoActions());
      return Actions.token.getBalance(client, { account, token });
    },
    enabled: enabled && !!publicClient && !!account,
  });
}

export function useBalances(
  address: string | undefined,
  _marketHash: string,
  verse: "YES" | "NO"
) {
  // TODO: Get actual verse token addresses from contract
  // These should be fetched from the Multiverse contract based on marketHash
  const mockYesUsdAddress =
    "0x0000000000000000000000000000000000000001" as Address;
  const mockNoUsdAddress =
    "0x0000000000000000000000000000000000000002" as Address;
  const mockYesUsdcAddress =
    "0x0000000000000000000000000000000000000003" as Address;
  const mockNoUsdcAddress =
    "0x0000000000000000000000000000000000000004" as Address;

  const usdVerseAddress =
    verse === "YES" ? mockYesUsdAddress : mockNoUsdAddress;
  const usdcVerseAddress =
    verse === "YES" ? mockYesUsdcAddress : mockNoUsdcAddress;

  // Use custom token balance hook with viem/tempo
  const { data: usdBalanceData, isLoading: usdLoading } = useTokenBalance({
    account: address as Address | undefined,
    token: usdVerseAddress,
    enabled: !!address,
  });

  const { data: usdcBalanceData, isLoading: usdcLoading } = useTokenBalance({
    account: address as Address | undefined,
    token: usdcVerseAddress,
    enabled: !!address,
  });

  // Tempo uses 6 decimals for native currency (USD)
  // Token balances should be formatted based on their decimals
  const usdBalance = usdBalanceData ? Number(usdBalanceData) / 1e6 : 0;
  const usdcBalance = usdcBalanceData ? Number(usdcBalanceData) / 1e6 : 0;

  return {
    // Keep ethBalance name for backward compatibility but it's now USD
    ethBalance: usdBalance,
    usdcBalance,
    loading: usdLoading || usdcLoading,
  };
}
