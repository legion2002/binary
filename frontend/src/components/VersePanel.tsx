import { useAccount } from 'wagmi';
import { useQuery } from '@tanstack/react-query';
import { fetchMarket } from '../api/client';
import { UniswapSwapWidget } from './UniswapSwapWidget';
import { useBalances } from '../hooks/useBalances';
import { CONTRACTS } from '../config/contracts';
import { Tooltip } from './Tooltip';

interface VersePanelProps {
  verse: 'YES' | 'NO';
  marketHash: string;
}

export function VersePanel({ verse, marketHash }: VersePanelProps) {
  const { address } = useAccount();

  // Fetch market data including token addresses
  const { data: market, isLoading: marketLoading } = useQuery({
    queryKey: ['market', marketHash],
    queryFn: () => fetchMarket(marketHash),
    enabled: !!marketHash,
  });

  // Get balances
  const { ethBalance, usdcBalance, loading: balanceLoading } = useBalances(
    address,
    marketHash,
    verse
  );

  // Get token addresses from the market data
  const verseToken = market?.verseTokens.find(t => t.asset.toLowerCase() === CONTRACTS.WETH.toLowerCase());
  const tokenAddress = verse === 'YES' ? verseToken?.yesVerse : verseToken?.noVerse;

  // Check for V4 pools
  const relevantPools = market?.v4Pools?.filter(pool =>
    pool.poolType.includes(verse)
  ) || [];

  // Format balance display
  const formatBalance = (balance: number, decimals: number = 6) => {
    if (balance === 0) return '0.000000';
    if (balance < 0.000001) return '<0.000001';
    return balance.toFixed(decimals);
  };

  // Loading state
  if (marketLoading) {
    return (
      <div className="bg-gradient-to-br from-gray-50 to-slate-50 rounded-xl p-6 border-2 border-gray-200 shadow-lg">
        <div className="animate-pulse space-y-4">
          <div className="h-8 bg-gray-200 rounded w-1/3"></div>
          <div className="h-20 bg-gray-200 rounded"></div>
          <div className="h-96 bg-gray-200 rounded"></div>
        </div>
      </div>
    );
  }

  return (
    <div className="bg-gradient-to-br from-gray-50 to-slate-50 rounded-xl p-6 border-2 border-gray-200 shadow-lg">
      {/* Header */}
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-3">
          <h3 className="text-2xl font-bold text-gray-800">
            {verse} Verse
          </h3>
          <span className={`px-3 py-1 text-sm font-medium rounded-full ${
            verse === 'YES'
              ? 'bg-green-100 text-green-700'
              : 'bg-red-100 text-red-700'
          }`}>
            {verse}
          </span>
        </div>
        <Tooltip content={`${verse} tokens pay out 1 ETH each if the market resolves ${verse}`}>
          <svg className="w-5 h-5 text-gray-400 cursor-help" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </Tooltip>
      </div>

      {/* Compact Holdings Display */}
      <div className="bg-white rounded-lg p-4 mb-4 shadow-sm">
        <div className="flex items-center justify-between mb-2">
          <h4 className="font-semibold text-gray-700 text-sm">Your Holdings</h4>
          <Tooltip content="Your current token balances">
            <svg className="w-4 h-4 text-gray-400 cursor-help" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </Tooltip>
        </div>

        {balanceLoading ? (
          <div className="animate-pulse space-y-2">
            <div className="h-8 bg-gray-200 rounded"></div>
            <div className="h-8 bg-gray-200 rounded"></div>
          </div>
        ) : (
          <div className="grid grid-cols-2 gap-2">
            <div className="bg-gray-50 rounded-lg p-2">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-1">
                  <div className="w-5 h-5 bg-blue-100 rounded-full flex items-center justify-center">
                    <span className="text-xs font-bold text-blue-600">Ξ</span>
                  </div>
                  <span className="text-xs font-medium text-gray-600">{verse} ETH</span>
                </div>
                <span className="font-mono text-sm font-semibold text-gray-900">
                  {formatBalance(ethBalance)}
                </span>
              </div>
            </div>
            <div className="bg-gray-50 rounded-lg p-2">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-1">
                  <div className="w-5 h-5 bg-green-100 rounded-full flex items-center justify-center">
                    <span className="text-xs font-bold text-green-600">$</span>
                  </div>
                  <span className="text-xs font-medium text-gray-600">{verse} USDC</span>
                </div>
                <span className="font-mono text-sm font-semibold text-gray-900">
                  {formatBalance(usdcBalance, 2)}
                </span>
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Professional Trading Interface */}
      <div className="space-y-4">
        {/* V4 Pool Information */}
        {relevantPools.length > 0 ? (
          <div className="bg-white rounded-lg p-4">
            <div className="flex items-center justify-between mb-3">
              <h4 className="text-sm font-semibold text-gray-700">Uniswap V4 Pools</h4>
              <span className="px-2 py-1 text-xs bg-purple-100 text-purple-700 rounded-full">V4</span>
            </div>
            <div className="space-y-2">
              {relevantPools.map(pool => (
                <div key={pool.poolId} className="bg-gray-50 rounded-lg p-3 text-xs">
                  <div className="font-medium text-gray-700 mb-1">{pool.poolType}</div>
                  <div className="text-gray-500 space-y-1">
                    <div>Pool ID: <span className="font-mono">{pool.poolId.slice(0, 10)}...</span></div>
                    <div>Fee: {pool.fee / 10000}% | Tick Spacing: {pool.tickSpacing}</div>
                  </div>
                </div>
              ))}
            </div>
            <p className="text-xs text-gray-400 mt-3">
              Note: V4 pools are active on-chain. Chart visualization coming soon.
            </p>
          </div>
        ) : (
          <div className="bg-white rounded-lg p-8 text-center">
            <svg className="w-12 h-12 text-gray-300 mx-auto mb-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
            </svg>
            <p className="text-sm text-gray-500 mb-1">V4 pools unavailable</p>
            <p className="text-xs text-gray-400">Waiting for pool deployment</p>
          </div>
        )}

        {/* Swap Widget */}
        {tokenAddress ? (
          <UniswapSwapWidget tokenAddress={tokenAddress} verse={verse} />
        ) : (
          <div className="bg-white rounded-lg p-8 text-center">
            <svg className="w-12 h-12 text-gray-300 mx-auto mb-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4" />
            </svg>
            <p className="text-sm text-gray-500 mb-1">Swap interface unavailable</p>
            <p className="text-xs text-gray-400">Waiting for token deployment</p>
          </div>
        )}
      </div>
    </div>
  );
}