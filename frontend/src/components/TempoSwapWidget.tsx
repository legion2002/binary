import { useState } from 'react';
import { parseUnits, formatUnits, type Address } from 'viem';
import { useBuyQuote, useSellQuote, useBuySync, useSellSync } from '../hooks/useTempoDex';
import { CONTRACTS } from '../config/contracts';
import { usePasskey } from '../contexts/PasskeyContext';
import { USD_TOKEN } from '../contexts/PasskeyContext';

interface TempoSwapWidgetProps {
  tokenAddress: string;
  verse: 'YES' | 'NO';
}

export function TempoSwapWidget({ tokenAddress, verse }: TempoSwapWidgetProps) {
  const { isConnected } = usePasskey();
  const [amount, setAmount] = useState('');
  const [isBuying, setIsBuying] = useState(true);

  // Custom Tempo DEX hooks for trading (using viem/tempo)
  const { mutate: buy, isPending: isBuyPending } = useBuySync();
  const { mutate: sell, isPending: isSellPending } = useSellSync();

  // Get quote for the swap
  const { data: buyQuote, isLoading: buyQuoteLoading } = useBuyQuote({
    tokenIn: CONTRACTS.USD as Address,
    tokenOut: tokenAddress as Address,
    amountOut: amount ? parseUnits(amount, 6) : 0n,
    query: {
      enabled: isBuying && !!amount && parseFloat(amount) > 0,
    },
  });

  const { data: sellQuote, isLoading: sellQuoteLoading } = useSellQuote({
    tokenIn: tokenAddress as Address,
    tokenOut: CONTRACTS.USD as Address,
    amountIn: amount ? parseUnits(amount, 6) : 0n,
    query: {
      enabled: !isBuying && !!amount && parseFloat(amount) > 0,
    },
  });

  const handleSwap = () => {
    if (!amount || !isConnected) return;

    const parsedAmount = parseUnits(amount, 6);

    if (isBuying) {
      // Buy verse tokens with USD
      buy({
        tokenIn: CONTRACTS.USD as Address,
        tokenOut: tokenAddress as Address,
        amountOut: parsedAmount,
        maxAmountIn: buyQuote ? (buyQuote * 105n) / 100n : parsedAmount * 2n, // 5% slippage
        feeToken: USD_TOKEN as Address,
      });
    } else {
      // Sell verse tokens for USD
      sell({
        tokenIn: tokenAddress as Address,
        tokenOut: CONTRACTS.USD as Address,
        amountIn: parsedAmount,
        minAmountOut: sellQuote ? (sellQuote * 95n) / 100n : 0n, // 5% slippage
        feeToken: USD_TOKEN as Address,
      });
    }
  };

  const isPending = isBuyPending || isSellPending;
  const isQuoteLoading = buyQuoteLoading || sellQuoteLoading;

  const formatQuote = (quote: bigint | undefined) => {
    if (!quote) return '0.00';
    return formatUnits(quote, 6);
  };

  return (
    <div className="w-full bg-white rounded-lg overflow-hidden shadow-sm">
      <div className="px-4 py-3 bg-gradient-to-r from-purple-50 to-blue-50 border-b border-gray-200">
        <div className="flex items-center justify-between">
          <h4 className="text-sm font-semibold text-gray-700">
            Swap {verse} Tokens
          </h4>
          <div className="flex items-center gap-2">
            <span className="text-xs text-gray-500">Powered by</span>
            <span className="text-xs font-semibold text-blue-600">Tempo DEX</span>
          </div>
        </div>
      </div>

      <div className="p-4 space-y-4">
        {/* Buy/Sell Toggle */}
        <div className="flex gap-2">
          <button
            onClick={() => setIsBuying(true)}
            className={`flex-1 py-2 px-4 rounded-lg text-sm font-medium transition-all ${
              isBuying
                ? 'bg-green-600 text-white'
                : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
            }`}
          >
            Buy {verse}
          </button>
          <button
            onClick={() => setIsBuying(false)}
            className={`flex-1 py-2 px-4 rounded-lg text-sm font-medium transition-all ${
              !isBuying
                ? 'bg-red-600 text-white'
                : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
            }`}
          >
            Sell {verse}
          </button>
        </div>

        {/* Amount Input */}
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">
            {isBuying ? `${verse} Tokens to Buy` : `${verse} Tokens to Sell`}
          </label>
          <div className="relative">
            <input
              type="number"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              placeholder="0.00"
              className="w-full px-4 py-3 pr-20 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-lg font-mono"
              disabled={!isConnected || isPending}
            />
            <div className="absolute right-3 top-1/2 transform -translate-y-1/2">
              <span className={`text-sm font-medium px-2 py-1 rounded ${
                verse === 'YES' ? 'bg-green-100 text-green-700' : 'bg-red-100 text-red-700'
              }`}>
                {verse}
              </span>
            </div>
          </div>
        </div>

        {/* Quote Display */}
        {amount && parseFloat(amount) > 0 && (
          <div className="bg-gray-50 rounded-lg p-3">
            <div className="flex justify-between items-center">
              <span className="text-sm text-gray-600">
                {isBuying ? 'You pay (USD)' : 'You receive (USD)'}
              </span>
              <span className="text-lg font-mono font-semibold">
                {isQuoteLoading ? (
                  <span className="animate-pulse">Loading...</span>
                ) : (
                  `$${formatQuote(isBuying ? buyQuote : sellQuote)}`
                )}
              </span>
            </div>
            <p className="text-xs text-gray-400 mt-1">
              Includes 5% max slippage protection
            </p>
          </div>
        )}

        {/* Swap Button */}
        <button
          onClick={handleSwap}
          disabled={!isConnected || !amount || isPending || parseFloat(amount || '0') <= 0}
          className={`w-full py-3 px-4 rounded-lg font-medium transition-all ${
            isBuying
              ? 'bg-green-600 hover:bg-green-700 text-white'
              : 'bg-red-600 hover:bg-red-700 text-white'
          } disabled:bg-gray-300 disabled:cursor-not-allowed`}
        >
          {isPending ? (
            <span className="flex items-center justify-center">
              <svg className="animate-spin h-4 w-4 mr-2" fill="none" viewBox="0 0 24 24">
                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              Processing...
            </span>
          ) : !isConnected ? (
            'Sign In to Trade'
          ) : (
            isBuying ? `Buy ${verse} Tokens` : `Sell ${verse} Tokens`
          )}
        </button>
      </div>

      <div className="px-4 py-2 bg-gray-50 border-t border-gray-100">
        <p className="text-xs text-gray-500 text-center">
          Trading on Tempo Testnet • Gasless with Fee Sponsorship
        </p>
      </div>
    </div>
  );
}
