import { useReadContracts } from 'wagmi';
import { ERC20_ABI, CONTRACTS, MULTIVERSE_ABI } from '../config/contracts';

export function useBalances(
  address: string | undefined,
  marketHash: string,
  verse: 'YES' | 'NO'
) {
  // TODO: Get actual verse token addresses from contract
  const mockYesEthAddress = '0x0000000000000000000000000000000000000001';
  const mockNoEthAddress = '0x0000000000000000000000000000000000000002';
  const mockYesUsdcAddress = '0x0000000000000000000000000000000000000003';
  const mockNoUsdcAddress = '0x0000000000000000000000000000000000000004';

  const ethVerseAddress = verse === 'YES' ? mockYesEthAddress : mockNoEthAddress;
  const usdcVerseAddress = verse === 'YES' ? mockYesUsdcAddress : mockNoUsdcAddress;

  const { data, isLoading } = useReadContracts({
    contracts: [
      {
        address: ethVerseAddress as `0x${string}`,
        abi: ERC20_ABI,
        functionName: 'balanceOf',
        args: address ? [address as `0x${string}`] : undefined,
      },
      {
        address: usdcVerseAddress as `0x${string}`,
        abi: ERC20_ABI,
        functionName: 'balanceOf',
        args: address ? [address as `0x${string}`] : undefined,
      },
    ],
    query: {
      enabled: !!address,
    },
  });

  const ethBalance = data?.[0]?.result ? Number(data[0].result) / 1e18 : 0;
  const usdcBalance = data?.[1]?.result ? Number(data[1].result) / 1e6 : 0;

  return {
    ethBalance,
    usdcBalance,
    loading: isLoading,
  };
}