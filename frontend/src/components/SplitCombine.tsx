import { useState } from 'react';
import { parseUnits } from 'viem';
import { useAccount } from 'wagmi';
import { MULTIVERSE_ABI, CONTRACTS } from '../config/contracts';
import { Tooltip } from './Tooltip';
import { useWriteContract } from '../hooks/useContractWrite';

interface SplitCombineProps {
  marketHash: string;
  activeAsset: 'USD' | 'USDC';
}

export function SplitCombine({ marketHash, activeAsset }: SplitCombineProps) {
  const { address, isConnected } = useAccount();
  const [amount, setAmount] = useState('');
  const [mode, setMode] = useState<'split' | 'combine'>('split');

  const { writeContract, isPending, isConfirming } = useWriteContract();

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
    <div className="card-dark border-l-accent-purple p-5 w-72">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-base font-bold text-primary">Arbitrage Tools</h3>
        <Tooltip content="Split USD into equal YES/NO tokens or combine them back">
          <svg className="w-5 h-5 text-muted cursor-help" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </Tooltip>
      </div>

      <div className="flex gap-2 mb-4">
        <button
          onClick={() => setMode('split')}
          className={`flex-1 py-2 px-3 rounded-lg text-sm font-medium transition-all ${
            mode === 'split'
              ? 'bg-accent-purple text-white'
              : 'bg-dark-hover text-secondary hover:text-primary'
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
              ? 'bg-accent-purple text-white'
              : 'bg-dark-hover text-secondary hover:text-primary'
          }`}
        >
          <svg className="w-4 h-4 inline mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 7h-12m0 0l4-4m-4 4l4 4m4 6h12m0 0l-4 4m4-4l-4-4" />
          </svg>
          Combine
        </button>
      </div>

      <div className="mb-4">
        <label className="block text-sm font-medium text-secondary mb-1">
          USD Amount
        </label>
        <div className="relative">
          <input
            type="number"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            placeholder="0.00"
            className="input-dark pr-12 text-lg font-mono"
            disabled={!isConnected || isLoading}
          />
          <div className="absolute right-3 top-1/2 transform -translate-y-1/2">
            <div className="w-5 h-5 bg-accent-green-dim rounded-full flex items-center justify-center">
              <span className="text-xs font-bold text-accent-green">$</span>
            </div>
          </div>
        </div>
      </div>

      <div className="bg-dark-tertiary rounded-lg p-3 mb-4 text-xs border border-dark">
        {mode === 'split' ? (
          <div>
            <p className="font-semibold text-accent-purple mb-1">You deposit:</p>
            <p className="text-secondary ml-2">• ${amount || '0'} USD</p>
            <p className="font-semibold text-accent-purple mt-2 mb-1">You receive:</p>
            <p className="text-accent-green ml-2">• {amount || '0'} YES tokens</p>
            <p className="text-accent-red ml-2">• {amount || '0'} NO tokens</p>
          </div>
        ) : (
          <div>
            <p className="font-semibold text-accent-purple mb-1">You deposit:</p>
            <p className="text-accent-green ml-2">• {amount || '0'} YES tokens</p>
            <p className="text-accent-red ml-2">• {amount || '0'} NO tokens</p>
            <p className="font-semibold text-accent-purple mt-2 mb-1">You receive:</p>
            <p className="text-secondary ml-2">• ${amount || '0'} USD</p>
          </div>
        )}
      </div>

      <button
        onClick={handleAction}
        disabled={!isConnected || !amount || isLoading}
        className="btn-primary w-full py-3 px-4 rounded-lg font-medium disabled:bg-dark-hover disabled:text-muted disabled:cursor-not-allowed"
      >
        {isLoading ? (
          <span className="flex items-center justify-center">
            <div className="w-4 h-4 spinner-dark mr-2"></div>
            Processing...
          </span>
        ) : (
          mode === 'split' ? 'Split USD' : 'Combine Tokens'
        )}
      </button>
    </div>
  );
}
