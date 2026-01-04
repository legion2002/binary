import { useParams, useNavigate } from 'react-router-dom';
import { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { usePasskey } from '../contexts/PasskeyContext';
import { fetchMarket } from '../api/client';
import { VersePanel } from './VersePanel';
import { SplitCombine } from './SplitCombine';
import { Tooltip } from './Tooltip';
import { calculateProbabilityFromPools, formatProbability } from '../utils/probability';

export function MarketDetail() {
  const { marketHash } = useParams<{ marketHash: string }>();
  const navigate = useNavigate();
  const { isConnected } = usePasskey();
  const [showHowItWorks, setShowHowItWorks] = useState(false);

  // Fetch market data from API
  const { data: market, isLoading, error } = useQuery({
    queryKey: ['market', marketHash],
    queryFn: () => fetchMarket(marketHash!),
    enabled: !!marketHash,
  });

  // Calculate probability from V4 pools
  const probability = market ? calculateProbabilityFromPools(market.v4Pools) : null;

  if (isLoading) {
    return (
      <div className="text-center py-12">
        <div className="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
        <p className="mt-4 text-gray-600">Loading market...</p>
      </div>
    );
  }

  if (error || !market) {
    return (
      <div className="text-center py-12">
        <h2 className="text-xl font-semibold mb-4">Market not found</h2>
        <p className="text-sm text-gray-600 mb-4">
          {error ? (error as Error).message : 'The requested market does not exist'}
        </p>
        <button
          onClick={() => navigate('/')}
          className="text-blue-600 hover:text-blue-700"
        >
          ← Back to markets
        </button>
      </div>
    );
  }

  return (
    <div>
      {/* Back navigation */}
      <a
        onClick={() => navigate('/')}
        className="mb-4 inline-flex items-center gap-1 text-sm text-gray-500 hover:text-gray-700 cursor-pointer transition-colors"
      >
        ← Back to markets
      </a>

      {/* Market Question Header */}
      <div className="bg-gradient-to-r from-blue-50 to-purple-50 rounded-xl shadow-lg p-6 mb-6 border border-blue-100">
        <div className="flex items-start justify-between">
          <div>
            <h2 className="text-2xl font-bold mb-2 text-gray-800">
              {market.question || 'Question not available'}
            </h2>
            <p className="text-sm text-gray-600">
              Deadline: {new Date(market.resolutionDeadline * 1000).toLocaleDateString()}
            </p>
          </div>
          <Tooltip content="This is a prediction market where you can trade on the outcome">
            <svg className="w-5 h-5 text-gray-400 cursor-help" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </Tooltip>
        </div>
      </div>

      {/* Market Probability Section */}
      {probability && (
        <div className="bg-white rounded-lg shadow-md p-6 mb-6">
          <h4 className="font-semibold mb-4 text-gray-700">Market Resolution Probability</h4>
          <div className="flex gap-4 justify-center">
            <div className="flex-1 max-w-xs bg-green-50 rounded-lg p-4 border border-green-200">
              <h5 className="font-semibold text-green-700 mb-2 text-center">YES</h5>
              <p className="text-3xl font-bold text-green-800 text-center">
                {formatProbability(probability.yesProbability)}
              </p>
              {probability.source && (
                <p className="text-xs text-gray-500 mt-1 text-center">from V4 pools</p>
              )}
            </div>
            <div className="flex-1 max-w-xs bg-red-50 rounded-lg p-4 border border-red-200">
              <h5 className="font-semibold text-red-700 mb-2 text-center">NO</h5>
              <p className="text-3xl font-bold text-red-800 text-center">
                {formatProbability(probability.noProbability)}
              </p>
              {probability.source && (
                <p className="text-xs text-gray-500 mt-1 text-center">from V4 pools</p>
              )}
            </div>
          </div>
          <div className="mt-4 text-sm text-gray-600 bg-gray-50 rounded-lg p-3">
            <p className="font-medium mb-1">How is this calculated?</p>
            <p>Market probabilities are derived from Uniswap V4 pool prices. The relative prices of YES and NO tokens
            in the pools reflect the market's belief about the outcome probability.</p>
            {probability.source && probability.source !== 'No price data - showing 50/50' && (
              <p className="text-xs mt-1">Source pool: {probability.source}</p>
            )}
          </div>
        </div>
      )}

      {/* How it works section */}
      <div className="mb-6">
        <a
          onClick={() => setShowHowItWorks(!showHowItWorks)}
          className="inline-flex items-center gap-1 text-sm text-blue-600 hover:text-blue-700 cursor-pointer transition-colors"
        >
          {showHowItWorks ? '▼' : '▶'} How it works
        </a>

        {showHowItWorks && (
          <div className="bg-white rounded-lg shadow-md p-6 mt-3">
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
              <div className="bg-gray-50 rounded-lg p-4">
                <h4 className="font-semibold mb-2 flex items-center gap-2">
                  <span className="text-green-600">1.</span> Trade on Tempo DEX
                </h4>
                <p className="text-gray-600">
                  Purchase YES tokens if you believe the outcome will happen, or NO tokens if you think it won't. Use Tempo's built-in DEX to swap.
                </p>
              </div>
              <div className="bg-gray-50 rounded-lg p-4">
                <h4 className="font-semibold mb-2 flex items-center gap-2">
                  <span className="text-blue-600">2.</span> Tempo Orderbook
                </h4>
                <p className="text-gray-600">
                  Trade on Tempo's native orderbook for efficient price discovery between YES/NO tokens. Place limit orders for advanced strategies.
                </p>
              </div>
              <div className="bg-gray-50 rounded-lg p-4">
                <h4 className="font-semibold mb-2 flex items-center gap-2">
                  <span className="text-purple-600">3.</span> Win or Arbitrage
                </h4>
                <p className="text-gray-600">
                  When resolved, winning tokens can be redeemed for $1 USD each. Use split/combine for arbitrage opportunities during trading.
                </p>
              </div>
            </div>
          </div>
        )}
      </div>

      {!isConnected && (
        <div className="text-center mb-6">
          <p className="text-sm text-gray-500 mb-2">Connect your wallet to start trading</p>
          <div className="inline-flex items-center gap-2 px-4 py-2 bg-gray-100 rounded-full">
            <div className="w-2 h-2 bg-gray-400 rounded-full animate-pulse"></div>
            <span className="text-xs text-gray-600">Wallet not connected</span>
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