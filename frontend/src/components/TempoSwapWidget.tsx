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

  const isYes = verse === 'YES';

  return (
    <div className="w-full bg-dark-tertiary rounded-lg overflow-hidden border border-dark">
      <div className="px-4 py-3 bg-dark border-b border-dark">
        <div className="flex items-center justify-between">
          <h4 className="text-sm font-semibold text-secondary">
            Swap {verse} Tokens
          </h4>
          <div className="flex items-center gap-2">
            <span className="text-xs text-muted">Powered by</span>
            <span className="text-xs font-semibold text-accent-blue">Tempo DEX</span>
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
                ? 'bg-accent-green text-white'
                : 'bg-dark-hover text-secondary hover:text-primary'
            }`}
          >
            Buy {verse}
          </button>
          <button
            onClick={() => setIsBuying(false)}
            className={`flex-1 py-2 px-4 rounded-lg text-sm font-medium transition-all ${
              !isBuying
                ? 'bg-accent-red text-white'
                : 'bg-dark-hover text-secondary hover:text-primary'
            }`}
          >
            Sell {verse}
          </button>
        </div>

        {/* Amount Input */}
        <div>
          <label className="block text-sm font-medium text-secondary mb-1">
            {isBuying ? `${verse} Tokens to Buy` : `${verse} Tokens to Sell`}
          </label>
          <div className="relative">
            <input
              type="number"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              placeholder="0.00"
              className="input-dark pr-20 text-lg font-mono"
              disabled={!isConnected || isPending}
            />
            <div className="absolute right-3 top-1/2 transform -translate-y-1/2">
              <span className={`text-sm font-medium px-2 py-1 rounded ${
                isYes ? 'bg-accent-green-dim text-accent-green' : 'bg-accent-red-dim text-accent-red'
              }`}>
                {verse}
              </span>
            </div>
          </div>
        </div>

        {/* Quote Display */}
        {amount && parseFloat(amount) > 0 && (
          <div className="bg-dark rounded-lg p-3 border border-dark">
            <div className="flex justify-between items-center">
              <span className="text-sm text-secondary">
                {isBuying ? 'You pay (USD)' : 'You receive (USD)'}
              </span>
              <span className="text-lg font-mono font-semibold text-primary">
                {isQuoteLoading ? (
                  <span className="animate-pulse-dark">Loading...</span>
                ) : (
                  `$${formatQuote(isBuying ? buyQuote : sellQuote)}`
                )}
              </span>
            </div>
            <p className="text-xs text-muted mt-1">
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
              ? 'btn-success'
              : 'btn-danger'
          } disabled:bg-dark-hover disabled:text-muted disabled:cursor-not-allowed`}
        >
          {isPending ? (
            <span className="flex items-center justify-center">
              <div className="w-4 h-4 spinner-dark mr-2"></div>
              Processing...
            </span>
          ) : !isConnected ? (
            'Sign In to Trade'
          ) : (
            isBuying ? `Buy ${verse} Tokens` : `Sell ${verse} Tokens`
          )}
        </button>
      </div>

      <div className="px-4 py-2 bg-dark border-t border-dark">
        <p className="text-xs text-muted text-center">
          Trading on Tempo Testnet • Gasless with Fee Sponsorship
        </p>
      </div>
    </div>
  );
}
