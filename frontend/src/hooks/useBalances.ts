import { Hooks } from 'tempo.ts/wagmi';

export function useBalances(
  address: string | undefined,
  _marketHash: string,
  verse: 'YES' | 'NO'
) {
  // TODO: Get actual verse token addresses from contract
  // These should be fetched from the Multiverse contract based on marketHash
  const mockYesUsdAddress = '0x0000000000000000000000000000000000000001' as `0x${string}`;
  const mockNoUsdAddress = '0x0000000000000000000000000000000000000002' as `0x${string}`;
  const mockYesUsdcAddress = '0x0000000000000000000000000000000000000003' as `0x${string}`;
  const mockNoUsdcAddress = '0x0000000000000000000000000000000000000004' as `0x${string}`;

  const usdVerseAddress = verse === 'YES' ? mockYesUsdAddress : mockNoUsdAddress;
  const usdcVerseAddress = verse === 'YES' ? mockYesUsdcAddress : mockNoUsdcAddress;

  // Use tempo.ts token balance hooks
  const { data: usdBalanceData, isLoading: usdLoading } = Hooks.token.useGetBalance({
    account: address as `0x${string}`,
    token: usdVerseAddress,
    query: {
      enabled: !!address,
    },
  });

  const { data: usdcBalanceData, isLoading: usdcLoading } = Hooks.token.useGetBalance({
    account: address as `0x${string}`,
    token: usdcVerseAddress,
    query: {
      enabled: !!address,
    },
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
