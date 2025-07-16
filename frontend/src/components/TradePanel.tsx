import { useState } from 'react';
import { useAccount } from 'wagmi';
import { parseUnits } from 'viem';

interface TradePanelProps {
  verse: 'YES' | 'NO';
  marketHash: string;
  tradeType: 'buy' | 'sell';
  activeAsset: 'ETH' | 'USDC';
  onAssetChange: (asset: 'ETH' | 'USDC') => void;
}

export function TradePanel({ 
  verse, 
  marketHash, 
  tradeType, 
  activeAsset,
  onAssetChange 
}: TradePanelProps) {
  const { isConnected } = useAccount();
  const [amount, setAmount] = useState('');

  // TODO: Implement actual trading logic with UniswapV2
  const handleTrade = async () => {
    if (!amount || !isConnected) return;
    
    const parsedAmount = parseUnits(amount, activeAsset === 'ETH' ? 18 : 6);
    console.log('Trade:', { verse, tradeType, amount: parsedAmount, activeAsset });
    
    // Trading logic will be implemented here
    alert('Trading functionality will be implemented with UniswapV2 integration');
  };

  return (
    <div>
      <div className="flex gap-2 mb-3">
        <button
          onClick={() => onAssetChange('ETH')}
          className={`flex-1 py-1 px-3 rounded text-sm font-medium transition-colors ${
            activeAsset === 'ETH'
              ? 'bg-blue-100 text-blue-700'
              : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
          }`}
        >
          ETH
        </button>
        <button
          onClick={() => onAssetChange('USDC')}
          className={`flex-1 py-1 px-3 rounded text-sm font-medium transition-colors ${
            activeAsset === 'USDC'
              ? 'bg-blue-100 text-blue-700'
              : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
          }`}
        >
          USDC
        </button>
      </div>

      <div className="mb-3">
        <label className="block text-sm font-medium text-gray-700 mb-1">
          {tradeType === 'buy' ? 'Buy' : 'Sell'} Amount
        </label>
        <input
          type="number"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
          placeholder="0.0"
          className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
          disabled={!isConnected}
        />
      </div>

      <button
        onClick={handleTrade}
        disabled={!isConnected || !amount}
        className={`w-full py-2 px-4 rounded-lg font-medium transition-colors disabled:bg-gray-300 disabled:cursor-not-allowed ${
          verse === 'YES'
            ? 'bg-green-600 text-white hover:bg-green-700'
            : 'bg-red-600 text-white hover:bg-red-700'
        }`}
      >
        {tradeType === 'buy' ? 'Buy' : 'Sell'} {verse}_{activeAsset}
      </button>
    </div>
  );
}