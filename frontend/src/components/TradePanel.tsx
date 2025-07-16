import { useState } from 'react';
import { useAccount } from 'wagmi';
import { parseUnits } from 'viem';
import { Tooltip } from './Tooltip';
import { usePriceFeed } from '../hooks/usePriceFeed';

interface TradePanelProps {
  verse: 'YES' | 'NO';
  marketHash: string;
  tradeType: 'buy' | 'sell';
}

export function TradePanel({ 
  verse, 
  marketHash, 
  tradeType
}: TradePanelProps) {
  const { isConnected } = useAccount();
  const [amount, setAmount] = useState('');
  const { price } = usePriceFeed(marketHash, verse);
  
  // Calculate conversions
  const pricePerEth = price / 100; // Price in dollars
  const ethAmount = tradeType === 'buy' ? parseFloat(amount) / pricePerEth : parseFloat(amount);
  const usdAmount = tradeType === 'sell' ? parseFloat(amount) * pricePerEth : parseFloat(amount);

  // TODO: Implement actual trading logic with UniswapV2
  const handleTrade = async () => {
    if (!amount || !isConnected) return;
    
    if (tradeType === 'buy') {
      // When buying, amount is in USD, convert to ETH amount
      const ethToBuy = parseFloat(amount) / pricePerEth;
      console.log('Buy:', { verse, usdAmount: amount, ethAmount: ethToBuy });
    } else {
      // When selling, amount is in ETH
      console.log('Sell:', { verse, ethAmount: amount, usdAmount: parseFloat(amount) * pricePerEth });
    }
    
    // Trading logic will be implemented here
    alert('Trading functionality will be implemented with UniswapV2 integration');
  };

  return (
    <div>
      <div className="mb-4">
        <div className="flex items-center justify-between mb-2">
          <label className="block text-sm font-medium text-gray-700">
            {tradeType === 'buy' ? 'USD Amount' : 'ETH Amount'}
          </label>
          <Tooltip content={
            tradeType === 'buy' 
              ? `Amount of USD to spend buying ETH if ${verse}` 
              : `Amount of ETH to sell from your ${verse} position`
          }>
            <svg className="w-4 h-4 text-gray-400 cursor-help" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </Tooltip>
        </div>
        <div className="relative">
          <div className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-500 font-medium">
            {tradeType === 'buy' ? '$' : 'Ξ'}
          </div>
          <input
            type="number"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            placeholder="0.00"
            className={`w-full ${tradeType === 'buy' ? 'pl-8' : 'pl-10'} pr-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-lg font-mono bg-white`}
            disabled={!isConnected}
            step="0.01"
          />
        </div>
        {amount && (
          <p className="text-sm text-gray-600 mt-2">
            {tradeType === 'buy' 
              ? `You'll receive ≈ ${ethAmount.toFixed(6)} ETH if ${verse} wins`
              : `You'll receive ≈ $${usdAmount.toFixed(2)} USD`
            }
          </p>
        )}
      </div>

      <button
        onClick={handleTrade}
        disabled={!isConnected || !amount}
        className={`w-full py-4 px-6 rounded-md font-semibold text-lg transition-all disabled:bg-gray-200 disabled:text-gray-400 disabled:cursor-not-allowed ${
          tradeType === 'buy' 
            ? 'bg-green-600 hover:bg-green-700 text-white shadow-sm' 
            : 'bg-red-600 hover:bg-red-700 text-white shadow-sm'
        }`}
      >
        {tradeType === 'buy' ? 'Buy' : 'Sell'} ETH if {verse}
      </button>
      
      {!isConnected && (
        <p className="text-xs text-center text-gray-500 mt-2">
          Connect wallet to trade
        </p>
      )}
    </div>
  );
}