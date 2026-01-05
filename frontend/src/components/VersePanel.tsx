import { useQuery } from '@tanstack/react-query';
import { fetchMarket } from '../api/client';
import { TempoSwapWidget } from './TempoSwapWidget';
import { useBalances } from '../hooks/useBalances';
import { CONTRACTS } from '../config/contracts';
import { Tooltip } from './Tooltip';
import { usePasskey } from '../contexts/PasskeyContext';

interface VersePanelProps {
  verse: 'YES' | 'NO';
  marketHash: string;
}

export function VersePanel({ verse, marketHash }: VersePanelProps) {
  const { address } = usePasskey();

  // Fetch market data including token addresses
  const { data: market, isLoading: marketLoading } = useQuery({
    queryKey: ['market', marketHash],
    queryFn: () => fetchMarket(marketHash),
    enabled: !!marketHash,
  });

  // Get balances using viem/tempo
  const { ethBalance: usdBalance, usdcBalance, loading: balanceLoading } = useBalances(
    address,
    marketHash,
    verse
  );

  // Get token addresses from the market data
  // On Tempo, we use USD as the primary asset
  const verseToken = market?.verseTokens.find(t => t.asset.toLowerCase() === CONTRACTS.USD.toLowerCase());
  const tokenAddress = verse === 'YES' ? verseToken?.yesVerse : verseToken?.noVerse;

  // Format balance display with safety checks
  const formatBalance = (balance: number, decimals: number = 6) => {
    // Guard against NaN, Infinity, or unreasonably large numbers
    if (!Number.isFinite(balance) || balance > 1e15 || balance < 0) return '0.000000';
    if (balance === 0) return '0.000000';
    if (balance < 0.000001) return '<0.000001';
    return balance.toFixed(decimals);
  };

  const isYes = verse === 'YES';
  const accentClass = isYes ? 'accent-green' : 'accent-red';
  const borderClass = isYes ? 'border-l-accent-green' : 'border-l-accent-red';
  const dimClass = isYes ? 'bg-accent-green-dim' : 'bg-accent-red-dim';

  // Loading state
  if (marketLoading) {
    return (
      <div className={`card-dark ${borderClass} p-6`}>
        <div className="animate-pulse-dark space-y-4">
          <div className="h-8 bg-dark-tertiary rounded w-1/3"></div>
          <div className="h-20 bg-dark-tertiary rounded"></div>
          <div className="h-96 bg-dark-tertiary rounded"></div>
        </div>
      </div>
    );
  }

  return (
    <div className={`card-dark ${borderClass} p-6`}>
      {/* Header */}
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-3">
          <h3 className="text-2xl font-bold text-primary">
            {verse}
          </h3>
          <span className={`px-3 py-1 text-sm font-medium rounded-full ${dimClass} text-${accentClass}`}>
            {verse}
          </span>
        </div>
        <Tooltip content={`${verse} tokens pay out $1 USD each if the market resolves ${verse}`}>
          <svg className="w-5 h-5 text-muted cursor-help" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </Tooltip>
      </div>

      {/* Compact Holdings Display */}
      <div className="bg-dark-tertiary rounded-lg p-4 mb-4">
        <div className="flex items-center justify-between mb-2">
          <h4 className="font-semibold text-secondary text-sm">Your Holdings</h4>
          <Tooltip content="Your current token balances">
            <svg className="w-4 h-4 text-muted cursor-help" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </Tooltip>
        </div>

        {balanceLoading ? (
          <div className="animate-pulse-dark space-y-2">
            <div className="h-8 bg-dark-hover rounded"></div>
            <div className="h-8 bg-dark-hover rounded"></div>
          </div>
        ) : (
          <div className="grid grid-cols-2 gap-2">
            <div className="bg-dark rounded-lg p-2 border border-dark">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-1">
                  <div className={`w-5 h-5 ${dimClass} rounded-full flex items-center justify-center`}>
                    <span className={`text-xs font-bold text-${accentClass}`}>$</span>
                  </div>
                  <span className="text-xs font-medium text-muted">{verse} USD</span>
                </div>
                <span className="font-mono text-sm font-semibold text-primary">
                  {formatBalance(usdBalance)}
                </span>
              </div>
            </div>
            <div className="bg-dark rounded-lg p-2 border border-dark">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-1">
                  <div className="w-5 h-5 bg-accent-blue-dim rounded-full flex items-center justify-center">
                    <span className="text-xs font-bold text-accent-blue">$</span>
                  </div>
                  <span className="text-xs font-medium text-muted">{verse} USDC</span>
                </div>
                <span className="font-mono text-sm font-semibold text-primary">
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
        <div className="bg-dark-tertiary rounded-lg p-4">
          <div className="flex items-center justify-between mb-3">
            <h4 className="text-sm font-semibold text-secondary">Tempo DEX</h4>
            <span className="px-2 py-1 text-xs bg-accent-blue-dim text-accent-blue rounded-full">Native</span>
          </div>
          <p className="text-xs text-muted">
            Trade directly on Tempo's built-in orderbook DEX for fast, low-cost swaps.
          </p>
        </div>

        {/* Swap Widget */}
        {tokenAddress ? (
          <TempoSwapWidget tokenAddress={tokenAddress} verse={verse} />
        ) : (
          <div className="bg-dark-tertiary rounded-lg p-8 text-center">
            <svg className="w-12 h-12 text-muted mx-auto mb-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4" />
            </svg>
            <p className="text-sm text-secondary mb-1">Swap interface unavailable</p>
            <p className="text-xs text-muted">Waiting for token deployment on Tempo</p>
          </div>
        )}
      </div>
    </div>
  );
}
