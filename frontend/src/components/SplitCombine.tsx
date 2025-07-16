import { useState } from 'react';
import { useAccount, useWriteContract, useWaitForTransactionReceipt } from 'wagmi';
import { parseUnits } from 'viem';
import { MULTIVERSE_ABI, CONTRACTS } from '../config/contracts';

interface SplitCombineProps {
  marketHash: string;
  activeAsset: 'ETH' | 'USDC';
}

export function SplitCombine({ marketHash, activeAsset }: SplitCombineProps) {
  const { address, isConnected } = useAccount();
  const [amount, setAmount] = useState('');
  const [mode, setMode] = useState<'split' | 'combine'>('split');

  const { writeContract, data: hash, isPending } = useWriteContract();
  const { isLoading: isConfirming } = useWaitForTransactionReceipt({ hash });

  const assetAddress = activeAsset === 'ETH' ? CONTRACTS.WETH : CONTRACTS.USDC;
  const decimals = activeAsset === 'ETH' ? 18 : 6;

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
    <div className="bg-white rounded-lg shadow-lg p-6 w-64">
      <h3 className="text-lg font-bold mb-4 text-center">Split / Combine</h3>
      
      <div className="flex gap-2 mb-4">
        <button
          onClick={() => setMode('split')}
          className={`flex-1 py-2 px-3 rounded text-sm font-medium transition-colors ${
            mode === 'split'
              ? 'bg-blue-600 text-white'
              : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
          }`}
        >
          Split
        </button>
        <button
          onClick={() => setMode('combine')}
          className={`flex-1 py-2 px-3 rounded text-sm font-medium transition-colors ${
            mode === 'combine'
              ? 'bg-blue-600 text-white'
              : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
          }`}
        >
          Combine
        </button>
      </div>

      <div className="mb-4">
        <label className="block text-sm font-medium text-gray-700 mb-1">
          Amount ({activeAsset})
        </label>
        <input
          type="number"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
          placeholder="0.0"
          className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
          disabled={!isConnected || isLoading}
        />
      </div>

      <div className="text-xs text-gray-600 mb-4">
        {mode === 'split' ? (
          <p>
            Deposit {amount || '0'} {activeAsset} to receive:
            <br />• {amount || '0'} YES_{activeAsset}
            <br />• {amount || '0'} NO_{activeAsset}
          </p>
        ) : (
          <p>
            Combine {amount || '0'} YES_{activeAsset} + {amount || '0'} NO_{activeAsset}
            <br />to receive {amount || '0'} {activeAsset}
          </p>
        )}
      </div>

      <button
        onClick={handleAction}
        disabled={!isConnected || !amount || isLoading}
        className="w-full py-2 px-4 bg-blue-600 text-white rounded-lg font-medium hover:bg-blue-700 transition-colors disabled:bg-gray-300 disabled:cursor-not-allowed"
      >
        {isLoading ? 'Processing...' : mode === 'split' ? 'Split' : 'Combine'}
      </button>
    </div>
  );
}