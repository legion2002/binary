import { useAccount } from 'wagmi';
import { useQuery } from '@tanstack/react-query';
import { fetchMarket } from '../api/client';
import { TempoSwapWidget } from './TempoSwapWidget';
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

  // Get balances using tempo.ts token hooks
  const { ethBalance: usdBalance, usdcBalance, loading: balanceLoading } = useBalances(
    address,
    marketHash,
    verse
  );

  // Get token addresses from the market data
  // On Tempo, we use USD as the primary asset
  const verseToken = market?.verseTokens.find(t => t.asset.toLowerCase() === CONTRACTS.USD.toLowerCase());
  const tokenAddress = verse === 'YES' ? verseToken?.yesVerse : verseToken?.noVerse;

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
        <Tooltip content={`${verse} tokens pay out $1 USD each if the market resolves ${verse}`}>
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
                  <div className="w-5 h-5 bg-green-100 rounded-full flex items-center justify-center">
                    <span className="text-xs font-bold text-green-600">$</span>
                  </div>
                  <span className="text-xs font-medium text-gray-600">{verse} USD</span>
                </div>
                <span className="font-mono text-sm font-semibold text-gray-900">
                  {formatBalance(usdBalance)}
                </span>
              </div>
            </div>
            <div className="bg-gray-50 rounded-lg p-2">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-1">
                  <div className="w-5 h-5 bg-blue-100 rounded-full flex items-center justify-center">
                    <span className="text-xs font-bold text-blue-600">$</span>
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

      {/* Trading Interface */}
      <div className="space-y-4">
        {/* Tempo DEX Info */}
        <div className="bg-white rounded-lg p-4">
          <div className="flex items-center justify-between mb-3">
            <h4 className="text-sm font-semibold text-gray-700">Tempo DEX</h4>
            <span className="px-2 py-1 text-xs bg-blue-100 text-blue-700 rounded-full">Native</span>
          </div>
          <p className="text-xs text-gray-500">
            Trade directly on Tempo's built-in orderbook DEX for fast, low-cost swaps.
          </p>
        </div>

        {/* Swap Widget */}
        {tokenAddress ? (
          <TempoSwapWidget tokenAddress={tokenAddress} verse={verse} />
        ) : (
          <div className="bg-white rounded-lg p-8 text-center">
            <svg className="w-12 h-12 text-gray-300 mx-auto mb-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4" />
            </svg>
            <p className="text-sm text-gray-500 mb-1">Swap interface unavailable</p>
            <p className="text-xs text-gray-400">Waiting for token deployment on Tempo</p>
          </div>
        )}
      </div>
    </div>
  );
}
