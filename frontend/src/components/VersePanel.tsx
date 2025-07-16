import { useState } from 'react';
import { useAccount } from 'wagmi';
import { TradePanel } from './TradePanel';
import { useBalances } from '../hooks/useBalances';
import { usePriceFeed } from '../hooks/usePriceFeed';

interface VersePanelProps {
  verse: 'YES' | 'NO';
  marketHash: string;
  activeAsset: 'ETH' | 'USDC';
  onAssetChange: (asset: 'ETH' | 'USDC') => void;
}

export function VersePanel({ verse, marketHash, activeAsset, onAssetChange }: VersePanelProps) {
  const { address } = useAccount();
  const [tradeType, setTradeType] = useState<'buy' | 'sell'>('buy');
  
  const { ethBalance, usdcBalance, loading: balanceLoading } = useBalances(
    address,
    marketHash,
    verse
  );
  
  const { price, loading: priceLoading } = usePriceFeed(marketHash, verse);

  const verseColor = verse === 'YES' ? 'green' : 'red';
  const bgColor = verse === 'YES' ? 'bg-green-50' : 'bg-red-50';
  const borderColor = verse === 'YES' ? 'border-green-200' : 'border-red-200';
  const textColor = verse === 'YES' ? 'text-green-800' : 'text-red-800';

  return (
    <div className={`${bgColor} rounded-lg p-6 border ${borderColor}`}>
      <h3 className={`text-xl font-bold mb-4 ${textColor}`}>{verse} Verse</h3>
      
      <div className="bg-white rounded-lg p-4 mb-4">
        <h4 className="font-semibold mb-2">Your Balances</h4>
        {balanceLoading ? (
          <p className="text-gray-500">Loading...</p>
        ) : (
          <div className="space-y-1">
            <div className="flex justify-between">
              <span>{verse}_ETH:</span>
              <span className="font-mono">{ethBalance.toFixed(6)}</span>
            </div>
            <div className="flex justify-between">
              <span>{verse}_USDC:</span>
              <span className="font-mono">{usdcBalance.toFixed(2)}</span>
            </div>
          </div>
        )}
      </div>

      <div className="bg-white rounded-lg p-4 mb-4">
        <h4 className="font-semibold mb-2">Price Feed</h4>
        {priceLoading ? (
          <p className="text-gray-500">Loading...</p>
        ) : (
          <div>
            <p className="text-2xl font-bold">${price.toFixed(2)}</p>
            <p className="text-sm text-gray-600">{verse}_ETH / {verse}_USDC</p>
          </div>
        )}
      </div>

      <div className="bg-white rounded-lg p-4">
        <div className="flex gap-2 mb-4">
          <button
            onClick={() => setTradeType('buy')}
            className={`flex-1 py-2 px-4 rounded font-medium transition-colors ${
              tradeType === 'buy'
                ? `bg-${verseColor}-600 text-white`
                : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
            }`}
          >
            Buy
          </button>
          <button
            onClick={() => setTradeType('sell')}
            className={`flex-1 py-2 px-4 rounded font-medium transition-colors ${
              tradeType === 'sell'
                ? `bg-${verseColor}-600 text-white`
                : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
            }`}
          >
            Sell
          </button>
        </div>

        <TradePanel
          verse={verse}
          marketHash={marketHash}
          tradeType={tradeType}
          activeAsset={activeAsset}
          onAssetChange={onAssetChange}
        />
      </div>
    </div>
  );
}