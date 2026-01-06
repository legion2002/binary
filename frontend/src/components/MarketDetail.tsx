import { useParams, useNavigate } from 'react-router-dom';
import { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { useAccount } from 'wagmi';
import { fetchMarket } from '../api/client';
import { VersePanel } from './VersePanel';
import { SplitCombine } from './SplitCombine';
import { Tooltip } from './Tooltip';
import { calculateProbabilityFromOrderbooks, formatProbability } from '../utils/probability';

export function MarketDetail() {
  const { marketHash } = useParams<{ marketHash: string }>();
  const navigate = useNavigate();
  const { isConnected } = useAccount();
  const [showHowItWorks, setShowHowItWorks] = useState(false);

  // Fetch market data from API
  const { data: market, isLoading, error } = useQuery({
    queryKey: ['market', marketHash],
    queryFn: () => fetchMarket(marketHash!),
    enabled: !!marketHash,
  });

  // Use API probability if available, otherwise calculate from orderbooks
  const probability = market
    ? market.yesProbability != null && market.noProbability != null
      ? { yesProbability: market.yesProbability, noProbability: market.noProbability, source: 'API' }
      : calculateProbabilityFromOrderbooks(market.orderbooks)
    : null;

  if (isLoading) {
    return (
      <div className="text-center py-12">
        <div className="inline-block w-8 h-8 spinner-dark"></div>
        <p className="mt-4 text-secondary">Loading market...</p>
      </div>
    );
  }

  if (error || !market) {
    return (
      <div className="text-center py-12">
        <h2 className="text-xl font-semibold mb-4 text-primary">Market not found</h2>
        <p className="text-sm text-muted mb-4">
          {error ? (error as Error).message : 'The requested market does not exist'}
        </p>
        <button
          onClick={() => navigate('/')}
          className="text-accent-purple hover:text-primary transition-colors"
        >
          ← Back to markets
        </button>
      </div>
    );
  }

  const yesPercent = probability ? Math.round(probability.yesProbability * 100) : 50;
  const noPercent = probability ? Math.round(probability.noProbability * 100) : 50;

  return (
    <div>
      {/* Back navigation */}
      <button
        onClick={() => navigate('/')}
        className="mb-4 inline-flex items-center gap-1 text-sm text-muted hover:text-primary cursor-pointer transition-colors"
      >
        ← Back to markets
      </button>

      {/* Market Question Header */}
      <div className="card-dark border-l-accent-purple p-6 mb-6">
        <div className="flex items-start justify-between">
          <div>
            <h2 className="text-2xl font-bold mb-2 text-primary">
              {market.question || 'Question not available'}
            </h2>
            <p className="text-sm text-muted">
              Resolves: {new Date(market.resolutionDeadline * 1000).toLocaleDateString()}
              {market.resolution && market.resolution !== 'UNRESOLVED' && (
                <span className={`ml-3 px-2 py-1 rounded text-xs font-medium ${
                  market.resolution === 'YES' ? 'bg-accent-green-dim text-accent-green' :
                  market.resolution === 'NO' ? 'bg-accent-red-dim text-accent-red' :
                  'bg-accent-purple-dim text-accent-purple'
                }`}>
                  Resolved: {market.resolution}
                </span>
              )}
            </p>
          </div>
          <Tooltip content="This is a prediction market where you can trade on the outcome">
            <svg className="w-5 h-5 text-muted cursor-help" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </Tooltip>
        </div>
      </div>

      {/* Market Probability Section */}
      {probability && (
        <div className="card-dark p-6 mb-6">
          <h4 className="font-semibold mb-4 text-secondary">Market Probability</h4>

          {/* Probability bar */}
          <div className="probability-bar mb-4" style={{ height: '12px' }}>
            <div
              className="probability-bar-yes"
              style={{ width: `${yesPercent}%` }}
            />
            <div
              className="probability-bar-no"
              style={{ width: `${noPercent}%` }}
            />
          </div>

          <div className="flex gap-4 justify-center">
            <div className="flex-1 max-w-xs bg-accent-green-dim rounded-lg p-4 border border-accent-green border-opacity-30">
              <h5 className="font-semibold text-accent-green mb-2 text-center">YES</h5>
              <p className="text-3xl font-bold text-accent-green text-center">
                {formatProbability(probability.yesProbability)}
              </p>
            </div>
            <div className="flex-1 max-w-xs bg-accent-red-dim rounded-lg p-4 border border-accent-red border-opacity-30">
              <h5 className="font-semibold text-accent-red mb-2 text-center">NO</h5>
              <p className="text-3xl font-bold text-accent-red text-center">
                {formatProbability(probability.noProbability)}
              </p>
            </div>
          </div>
        </div>
      )}

      {/* How it works section */}
      <div className="mb-6">
        <button
          onClick={() => setShowHowItWorks(!showHowItWorks)}
          className="inline-flex items-center gap-1 text-sm text-accent-purple hover:text-primary cursor-pointer transition-colors"
        >
          {showHowItWorks ? '▼' : '▶'} How it works
        </button>

        {showHowItWorks && (
          <div className="card-dark p-6 mt-3">
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
              <div className="bg-dark-tertiary rounded-lg p-4">
                <h4 className="font-semibold mb-2 flex items-center gap-2">
                  <span className="text-accent-green">1.</span>
                  <span className="text-primary">Trade on Tempo DEX</span>
                </h4>
                <p className="text-muted">
                  Purchase YES tokens if you believe the outcome will happen, or NO tokens if you think it won't.
                </p>
              </div>
              <div className="bg-dark-tertiary rounded-lg p-4">
                <h4 className="font-semibold mb-2 flex items-center gap-2">
                  <span className="text-accent-blue">2.</span>
                  <span className="text-primary">Tempo Orderbook</span>
                </h4>
                <p className="text-muted">
                  Trade on Tempo's native orderbook for efficient price discovery between YES/NO tokens.
                </p>
              </div>
              <div className="bg-dark-tertiary rounded-lg p-4">
                <h4 className="font-semibold mb-2 flex items-center gap-2">
                  <span className="text-accent-purple">3.</span>
                  <span className="text-primary">Win or Arbitrage</span>
                </h4>
                <p className="text-muted">
                  When resolved, winning tokens redeem for $1 USD each. Use split/combine for arbitrage.
                </p>
              </div>
            </div>
          </div>
        )}
      </div>

      {!isConnected && (
        <div className="text-center mb-6">
          <p className="text-sm text-muted mb-2">Connect your wallet to start trading</p>
          <div className="inline-flex items-center gap-2 px-4 py-2 bg-dark-tertiary rounded-full border border-dark">
            <div className="w-2 h-2 bg-accent-purple rounded-full animate-pulse"></div>
            <span className="text-xs text-secondary">Wallet not connected</span>
          </div>
        </div>
      )}

      {/* Main Trading Layout */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 relative">
        <VersePanel
          verse="YES"
          marketHash={marketHash!}
        />

        {/* Split/Combine Tool - Center on desktop, bottom on mobile */}
        <div className="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 z-10 hidden lg:block">
          <SplitCombine
            marketHash={marketHash!}
            activeAsset="USD"
          />
        </div>

        <VersePanel
          verse="NO"
          marketHash={marketHash!}
        />
      </div>

      {/* Split/Combine for mobile */}
      <div className="lg:hidden mt-6 flex justify-center">
        <SplitCombine
          marketHash={marketHash!}
          activeAsset="USD"
        />
      </div>
    </div>
  );
}
