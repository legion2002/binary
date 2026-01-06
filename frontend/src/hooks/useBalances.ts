import type { Address } from "viem";
import { useAccount } from "wagmi";
import { Hooks } from "wagmi/tempo";

/**
 * Hook to get a token balance using wagmi/tempo
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
  return Hooks.token.useGetBalance({
    account: account!,
    token,
    query: {
      enabled: enabled && !!account,
    },
  });
}

export function useBalances(
  _address: string | undefined,
  _marketHash: string,
  verse: "YES" | "NO"
) {
  const { address } = useAccount();

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

  // Use wagmi/tempo token balance hooks
  const { data: usdBalanceData, isLoading: usdLoading } = useTokenBalance({
    account: address,
    token: usdVerseAddress,
    enabled: !!address,
  });

  const { data: usdcBalanceData, isLoading: usdcLoading } = useTokenBalance({
    account: address,
    token: usdcVerseAddress,
    enabled: !!address,
  });

  // Tempo uses 6 decimals for native currency (USD)
  // Token balances should be formatted based on their decimals
  // Guard against invalid/garbage data from non-existent tokens
  const parseBalance = (data: bigint | undefined): number => {
    if (!data) return 0;
    const num = Number(data) / 1e6;
    // Guard against NaN, Infinity, or unreasonably large numbers
    if (!Number.isFinite(num) || num > 1e15 || num < 0) return 0;
    return num;
  };

  const usdBalance = parseBalance(usdBalanceData);
  const usdcBalance = parseBalance(usdcBalanceData);

  return {
    // Keep ethBalance name for backward compatibility but it's now USD
    ethBalance: usdBalance,
    usdcBalance,
    loading: usdLoading || usdcLoading,
  };
}
