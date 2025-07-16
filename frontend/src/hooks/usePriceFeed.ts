import { useReadContract } from 'wagmi';
import { UNISWAP_V2_PAIR_ABI } from '../config/contracts';

export function usePriceFeed(marketHash: string, verse: 'YES' | 'NO') {
  // TODO: Get actual UniswapV2 pair addresses
  const mockPairAddress = verse === 'YES' 
    ? '0x0000000000000000000000000000000000000005'
    : '0x0000000000000000000000000000000000000006';

  const { data: reserves, isLoading } = useReadContract({
    address: mockPairAddress as `0x${string}`,
    abi: UNISWAP_V2_PAIR_ABI,
    functionName: 'getReserves',
  });

  // Mock price calculation - in production, this would use actual reserves
  let price = 0;
  if (reserves) {
    // Assuming token0 is YES/NO_ETH and token1 is YES/NO_USDC
    // Price = USDC per ETH
    const ethReserve = Number(reserves[0]) / 1e18;
    const usdcReserve = Number(reserves[1]) / 1e6;
    price = ethReserve > 0 ? usdcReserve / ethReserve : 0;
  }

  // For now, return mock prices
  const mockPrice = verse === 'YES' ? 2100 : 1900;

  return {
    price: price || mockPrice,
    loading: isLoading,
  };
}