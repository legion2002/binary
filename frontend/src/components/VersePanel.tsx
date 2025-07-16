import { useState } from 'react';
import { useAccount } from 'wagmi';
import { TradePanel } from './TradePanel';
import { useBalances } from '../hooks/useBalances';
import { usePriceFeed } from '../hooks/usePriceFeed';
import { Tooltip } from './Tooltip';

interface VersePanelProps {
  verse: 'YES' | 'NO';
  marketHash: string;
}

export function VersePanel({ verse, marketHash }: VersePanelProps) {
  const { address } = useAccount();
  const [tradeType, setTradeType] = useState<'buy' | 'sell'>('buy');
  
  const { ethBalance, usdcBalance, loading: balanceLoading } = useBalances(
    address,
    marketHash,
    verse
  );
  
  const { price, loading: priceLoading } = usePriceFeed(marketHash, verse);

  const isYes = verse === 'YES';
  
  // Format large numbers properly
  const formatBalance = (balance: number, decimals: number = 6) => {
    if (balance === 0) return '0.000000';
    if (balance < 0.000001) return '<0.000001';
    return balance.toFixed(decimals);
  };

  return (
    <div className="bg-gradient-to-br from-gray-50 to-slate-50 rounded-xl p-6 border-2 border-gray-200 shadow-lg">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h3 className="text-2xl font-bold text-gray-800">
            {verse} Verse
          </h3>
        </div>
        <Tooltip content={`${verse} tokens pay out 1 ETH each if the market resolves ${verse}`}>
          <svg className="w-5 h-5 text-gray-400 cursor-help" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </Tooltip>
      </div>
      
      <div className="bg-white rounded-lg p-4 mb-4 shadow-sm">
        <div className="flex items-center justify-between mb-3">
          <h4 className="font-semibold text-gray-700">Your Holdings</h4>
          <Tooltip content="These tokens can be traded or redeemed for ETH when the market resolves">
            <svg className="w-4 h-4 text-gray-400 cursor-help" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </Tooltip>
        </div>
        {balanceLoading ? (
          <div className="animate-pulse space-y-2">
            <div className="h-10 bg-gray-200 rounded"></div>
            <div className="h-10 bg-gray-200 rounded"></div>
          </div>
        ) : (
          <div className="space-y-2">
            <div className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
              <div className="flex items-center gap-2">
                <div className="w-6 h-6 bg-blue-100 rounded-full flex items-center justify-center">
                  <span className="text-xs font-bold text-blue-600">Ξ</span>
                </div>
                <span className="font-medium text-gray-700">{verse} ETH</span>
              </div>
              <span className="font-mono font-semibold text-gray-900">{formatBalance(ethBalance)}</span>
            </div>
            <div className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
              <div className="flex items-center gap-2">
                <div className="w-6 h-6 bg-green-100 rounded-full flex items-center justify-center">
                  <span className="text-xs font-bold text-green-600">$</span>
                </div>
                <span className="font-medium text-gray-700">{verse} USDC</span>
              </div>
              <span className="font-mono font-semibold text-gray-900">{formatBalance(usdcBalance, 2)}</span>
            </div>
          </div>
        )}
      </div>

      <div className="bg-white rounded-lg p-4 mb-4 shadow-sm">
        <div className="flex items-center justify-between mb-3">
          <h4 className="font-semibold text-gray-700">ETH Price if {verse}</h4>
          <Tooltip content={`The price to buy 1 ETH worth of ${verse} tokens that pay out if the market resolves ${verse}`}>
            <svg className="w-4 h-4 text-gray-400 cursor-help" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </Tooltip>
        </div>
        {priceLoading ? (
          <div className="animate-pulse space-y-2">
            <div className="h-10 bg-gray-200 rounded w-3/4"></div>
            <div className="h-4 bg-gray-200 rounded w-full"></div>
          </div>
        ) : price > 0 ? (
          <div>
            <div className="flex items-baseline gap-2">
              <span className="text-3xl font-bold text-gray-900">${(price / 100).toFixed(2)}</span>
              <span className="text-sm text-gray-500">per ETH</span>
            </div>
            <p className="text-sm text-gray-600 mt-2">
              Pay ${(price / 100).toFixed(2)} now to receive 1 ETH if {verse} wins
            </p>
          </div>
        ) : (
          <div className="text-gray-500 text-sm">
            Price feed unavailable
          </div>
        )}
      </div>

      <div className="bg-white/80 backdrop-blur rounded-lg p-4 border border-white/50">
        <div className="flex gap-4 mb-4">
          <button
            onClick={() => setTradeType('buy')}
            className={`flex-1 py-4 px-6 rounded-md font-semibold text-lg transition-all ${
              tradeType === 'buy'
                ? 'bg-green-600 hover:bg-green-700 text-white shadow-sm ring-2 ring-green-600'
                : 'bg-white hover:bg-gray-50 text-gray-700 border border-gray-300'
            }`}
          >
            Buy ETH
          </button>
          <button
            onClick={() => setTradeType('sell')}
            className={`flex-1 py-4 px-6 rounded-md font-semibold text-lg transition-all ${
              tradeType === 'sell'
                ? 'bg-red-600 hover:bg-red-700 text-white shadow-sm ring-2 ring-red-600'
                : 'bg-white hover:bg-gray-50 text-gray-700 border border-gray-300'
            }`}
          >
            Sell ETH
          </button>
        </div>

        <TradePanel
          verse={verse}
          marketHash={marketHash}
          tradeType={tradeType}
        />
      </div>
    </div>
  );
}