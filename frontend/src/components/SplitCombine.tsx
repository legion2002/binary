import { useState } from 'react';
import { useAccount, useWriteContract, useWaitForTransactionReceipt } from 'wagmi';
import { parseUnits } from 'viem';
import { MULTIVERSE_ABI, CONTRACTS } from '../config/contracts';
import { Tooltip } from './Tooltip';

interface SplitCombineProps {
  marketHash: string;
  activeAsset: 'USD' | 'USDC';
}

export function SplitCombine({ marketHash, activeAsset }: SplitCombineProps) {
  const { address, isConnected } = useAccount();
  const [amount, setAmount] = useState('');
  const [mode, setMode] = useState<'split' | 'combine'>('split');

  const { writeContract, data: hash, isPending } = useWriteContract();
  const { isLoading: isConfirming } = useWaitForTransactionReceipt({ hash });

  // Tempo uses USD as native currency with 6 decimals
  const assetAddress = activeAsset === 'USD' ? CONTRACTS.USD : CONTRACTS.USDC;
  const decimals = 6; // Tempo native currency is 6 decimals

  const handleAction = async () => {
    if (!amount || !isConnected || !address) return;

    const parsedAmount = parseUnits(amount, decimals);

    try {
      if (mode === 'split') {
        await writeContract({
          address: CONTRACTS.MULTIVERSE as `0x${string}`,
          abi: MULTIVERSE_ABI,
          functionName: 'split',
          args: [assetAddress as `0x${string}`, parsedAmount, marketHash as `0x${string}`],
        });
      } else {
        await writeContract({
          address: CONTRACTS.MULTIVERSE as `0x${string}`,
          abi: MULTIVERSE_ABI,
          functionName: 'combine',
          args: [assetAddress as `0x${string}`, parsedAmount, marketHash as `0x${string}`],
        });
      }
    } catch (error) {
      console.error('Transaction failed:', error);
    }
  };

  const isLoading = isPending || isConfirming;

  return (
    <div className="bg-gradient-to-br from-purple-50 to-blue-50 rounded-xl shadow-xl p-6 w-72 border-2 border-purple-200">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-bold text-gray-800">Arbitrage Tools</h3>
        <Tooltip content="Split USD into equal YES/NO tokens or combine them back">
          <svg className="w-5 h-5 text-gray-400 cursor-help" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </Tooltip>
      </div>

      <div className="flex gap-2 mb-4">
        <button
          onClick={() => setMode('split')}
          className={`flex-1 py-2 px-3 rounded-lg text-sm font-medium transition-all ${
            mode === 'split'
              ? 'bg-purple-600 text-white shadow-lg transform scale-105'
              : 'bg-white text-gray-700 hover:bg-gray-100 border border-gray-200'
          }`}
        >
          <svg className="w-4 h-4 inline mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4" />
          </svg>
          Split
        </button>
        <button
          onClick={() => setMode('combine')}
          className={`flex-1 py-2 px-3 rounded-lg text-sm font-medium transition-all ${
            mode === 'combine'
              ? 'bg-purple-600 text-white shadow-lg transform scale-105'
              : 'bg-white text-gray-700 hover:bg-gray-100 border border-gray-200'
          }`}
        >
          <svg className="w-4 h-4 inline mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 7h-12m0 0l4-4m-4 4l4 4m4 6h12m0 0l-4 4m4-4l-4-4" />
          </svg>
          Combine
        </button>
      </div>

      <div className="mb-4">
        <label className="block text-sm font-medium text-gray-700 mb-1">
          USD Amount
        </label>
        <div className="relative">
          <input
            type="number"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            placeholder="0.00"
            className="w-full px-4 py-3 pr-12 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-500 text-lg font-mono"
            disabled={!isConnected || isLoading}
          />
          <div className="absolute right-3 top-1/2 transform -translate-y-1/2">
            <div className="w-5 h-5 bg-green-100 rounded-full flex items-center justify-center">
              <span className="text-xs font-bold text-green-600">$</span>
            </div>
          </div>
        </div>
      </div>

      <div className="bg-white/70 rounded-lg p-3 mb-4 text-xs">
        {mode === 'split' ? (
          <div>
            <p className="font-semibold text-purple-700 mb-1">You deposit:</p>
            <p className="text-gray-600 ml-2">• ${amount || '0'} USD</p>
            <p className="font-semibold text-purple-700 mt-2 mb-1">You receive:</p>
            <p className="text-gray-600 ml-2">• {amount || '0'} YES tokens</p>
            <p className="text-gray-600 ml-2">• {amount || '0'} NO tokens</p>
          </div>
        ) : (
          <div>
            <p className="font-semibold text-purple-700 mb-1">You deposit:</p>
            <p className="text-gray-600 ml-2">• {amount || '0'} YES tokens</p>
            <p className="text-gray-600 ml-2">• {amount || '0'} NO tokens</p>
            <p className="font-semibold text-purple-700 mt-2 mb-1">You receive:</p>
            <p className="text-gray-600 ml-2">• ${amount || '0'} USD</p>
          </div>
        )}
      </div>

      <button
        onClick={handleAction}
        disabled={!isConnected || !amount || isLoading}
        className="w-full py-3 px-4 bg-gradient-to-r from-purple-600 to-blue-600 text-white rounded-lg font-medium hover:from-purple-700 hover:to-blue-700 transition-all transform hover:scale-105 disabled:bg-gray-300 disabled:cursor-not-allowed disabled:transform-none"
      >
        {isLoading ? (
          <span className="flex items-center justify-center">
            <svg className="animate-spin h-4 w-4 mr-2" fill="none" viewBox="0 0 24 24">
              <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
              <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            Processing...
          </span>
        ) : (
          mode === 'split' ? 'Split USD' : 'Combine Tokens'
        )}
      </button>
    </div>
  );
}
