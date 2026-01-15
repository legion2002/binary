import { useState } from "react";
import { parseUnits, formatUnits, type Address } from "viem";
import { useAccount } from "wagmi";
import { useBuyQuote, useBuySync, useSellQuote, useSellSync } from "../hooks/useTempoDex";
import { useSplit } from "../hooks/useSplit";
import { useMarketLiquidity, useAddLiquidityToPair, useRemoveLiquidityFromPair } from "../hooks/useLiquidity";
import { useTokenBalance } from "../hooks/useBalances";
import { formatError } from "../utils/errorMessages";

type TradeMode = "trade-yes" | "trade-no" | "split" | "lp";
type ActionType = "buy" | "sell";
type LPActionType = "add" | "remove";
type LPTokenType = "yes" | "no";

interface TradePanelProps {
  marketHash: string;
  yesTokenAddress: string | undefined;
  noTokenAddress: string | undefined;
  yesPrice: number | null;
  noPrice: number | null;
  selectedAsset: Address;
  selectedBalance: bigint;
}

export function TradePanel({
  marketHash,
  yesTokenAddress,
  noTokenAddress,
  yesPrice,
  noPrice,
  selectedAsset,
  selectedBalance,
}: TradePanelProps) {
  const { address, isConnected } = useAccount();
  const [mode, setMode] = useState<TradeMode>("trade-yes");
  const [action, setAction] = useState<ActionType>("buy");
  const [lpAction, setLpAction] = useState<LPActionType>("add");
  const [lpToken, setLpToken] = useState<LPTokenType>("yes");
  const [amount, setAmount] = useState("");
  const [tradeError, setTradeError] = useState<string | null>(null);

  const { mutate: buy, isPending: isBuyPending, error: buyError } = useBuySync();
  const { mutate: sell, isPending: isSellPending, error: sellError } = useSellSync();
  const { split, isPending: isSplitPending } = useSplit();
  const { addLiquidity: addLiquidityToPair, isPending: isAddLpPending } = useAddLiquidityToPair();
  const { removeLiquidity: removeLiquidityFromPair, isPending: isRemoveLpPending } = useRemoveLiquidityFromPair();

  // Get LP info for both YES/PATH_USD and NO/PATH_USD pairs
  const { yesPair, noPair } = useMarketLiquidity({
    yesTokenAddress: yesTokenAddress as Address | undefined,
    noTokenAddress: noTokenAddress as Address | undefined,
    account: address,
    enabled: !!yesTokenAddress && !!noTokenAddress && !!address,
  });

  // Current selected pair info
  const currentPair = lpToken === "yes" ? yesPair : noPair;
  const currentTokenAddress = lpToken === "yes" ? yesTokenAddress : noTokenAddress;

  // Fetch YES/NO token balances for sell validation
  const { data: yesTokenBalance } = useTokenBalance({
    account: address,
    token: (yesTokenAddress ?? "0x0000000000000000000000000000000000000000") as Address,
    enabled: !!yesTokenAddress && !!address,
  });
  const { data: noTokenBalance } = useTokenBalance({
    account: address,
    token: (noTokenAddress ?? "0x0000000000000000000000000000000000000000") as Address,
    enabled: !!noTokenAddress && !!address,
  });

  // LP tokens use 18 decimals, verse tokens use 6
  const parsedAmount = mode === "lp" && lpAction === "remove" 
    ? parseUnits(amount || "0", 18) 
    : parseUnits(amount || "0", 6);
  
  // For buying: check USD balance. For selling: check token balance. For LP remove: check LP balance.
  const currentTokenBalance = mode === "trade-yes" ? (yesTokenBalance ?? 0n) : (noTokenBalance ?? 0n);
  const lpTokenBalance = lpToken === "yes" ? (yesTokenBalance ?? 0n) : (noTokenBalance ?? 0n);
  const relevantBalance = mode === "lp" 
    ? lpAction === "remove" 
      ? currentPair.balance 
      : lpTokenBalance // For adding LP, check the verse token balance
    : action === "sell" 
      ? currentTokenBalance 
      : selectedBalance;
  const exceedsBalance = parsedAmount > relevantBalance;

  const currentToken = mode === "trade-yes" ? yesTokenAddress : noTokenAddress;

  // Buy quotes: how much USD to pay for X tokens
  const { data: buyQuote, isLoading: buyQuoteLoading, error: buyQuoteError } = useBuyQuote({
    tokenIn: selectedAsset,
    tokenOut: (currentToken ?? selectedAsset) as Address,
    amountOut: parsedAmount,
    query: {
      enabled: mode !== "split" && action === "buy" && !!currentToken && parsedAmount > 0n,
    },
  });

  // Sell quotes: how much USD received for selling X tokens
  const { data: sellQuote, isLoading: sellQuoteLoading, error: sellQuoteError } = useSellQuote({
    tokenIn: (currentToken ?? selectedAsset) as Address,
    tokenOut: selectedAsset,
    amountIn: parsedAmount,
    query: {
      enabled: mode !== "split" && action === "sell" && !!currentToken && parsedAmount > 0n,
    },
  });

  // Check for quote errors (usually means insufficient liquidity)
  const quoteError = action === "buy" ? buyQuoteError : sellQuoteError;

  // Format errors for user-friendly display (prioritize quote errors for better UX)
  const rawError = quoteError || tradeError || buyError || sellError;
  const formattedError = rawError ? formatError(rawError) : null;

  const handleTrade = async () => {
    if (!amount || !isConnected || exceedsBalance) {
      return;
    }

    // Clear any previous error
    setTradeError(null);

    if (mode === "split") {
      try {
        await split({
          asset: selectedAsset,
          amount: parsedAmount,
          marketHash,
        });
      } catch (err) {
        console.error("[TradePanel] Split error:", err);
        setTradeError(err instanceof Error ? err.message : "Split failed");
      }
      return;
    }

    if (!currentToken) return;

    if (action === "buy") {
      buy({
        tokenIn: selectedAsset,
        tokenOut: currentToken as Address,
        amountOut: parsedAmount,
        maxAmountIn: buyQuote ? (buyQuote * 105n) / 100n : parsedAmount * 2n,
      });
    } else {
      sell({
        tokenIn: currentToken as Address,
        tokenOut: selectedAsset,
        amountIn: parsedAmount,
        minAmountOut: sellQuote ? (sellQuote * 95n) / 100n : 0n,
      });
    }
  };

  const handleMaxClick = () => {
    if (mode === "lp" && lpAction === "remove") {
      const maxAmount = Number(currentPair.balance) / 1e18;
      setAmount(maxAmount.toString());
    } else {
      const maxAmount = Number(relevantBalance) / 1e6;
      setAmount(maxAmount.toString());
    }
  };

  // Calculate the required USD tokens based on verse token input and current reserves
  // Also calculate the expected LP tokens to receive
  const calculateLpAmounts = (): { amountToken: bigint; amountUsd: bigint; estimatedLp: bigint } => {
    const { reserveToken, reserveUsd, totalSupply } = currentPair;
    // If no reserves (new pool), use 1:1 ratio
    if (!reserveToken || !reserveUsd || reserveToken === 0n || reserveUsd === 0n || !totalSupply) {
      // For new pools: LP = sqrt(amountToken * amountUsd) - 1000 (minimum liquidity)
      // Approximate as just the token amount for display
      return { amountToken: parsedAmount, amountUsd: parsedAmount, estimatedLp: parsedAmount };
    }
    // Calculate optimal USD amount based on current ratio
    // amountUsd = amountToken * reserveUsd / reserveToken
    const amountToken = parsedAmount;
    const amountUsd = (parsedAmount * reserveUsd) / reserveToken;
    
    // Calculate LP tokens: min(amountToken * totalSupply / reserveToken, amountUsd * totalSupply / reserveUsd)
    const lpFromToken = (amountToken * totalSupply) / reserveToken;
    const lpFromUsd = (amountUsd * totalSupply) / reserveUsd;
    const estimatedLp = lpFromToken < lpFromUsd ? lpFromToken : lpFromUsd;
    
    return { amountToken, amountUsd, estimatedLp };
  };

  const lpAmounts = mode === "lp" && lpAction === "add" ? calculateLpAmounts() : null;

  const handleProvideLiquidity = async () => {
    if (!currentTokenAddress || !amount || !isConnected) return;
    setTradeError(null);
    
    const { amountToken, amountUsd } = calculateLpAmounts();
    
    try {
      await addLiquidityToPair({
        token: currentTokenAddress as Address,
        amountToken,
        amountUsd,
      });
      setAmount("");
    } catch (err) {
      console.error("[TradePanel] Add liquidity error:", err);
      setTradeError(err instanceof Error ? err.message : "Add liquidity failed");
    }
  };

  const handleRemoveLiquidity = async () => {
    if (!currentTokenAddress || !amount || !isConnected || !currentPair.pairAddress) return;
    setTradeError(null);
    try {
      await removeLiquidityFromPair({
        token: currentTokenAddress as Address,
        pairAddress: currentPair.pairAddress,
        liquidity: parsedAmount,
      });
      setAmount("");
    } catch (err) {
      console.error("[TradePanel] Remove liquidity error:", err);
      setTradeError(err instanceof Error ? err.message : "Remove liquidity failed");
    }
  };

  const isPending = isBuyPending || isSellPending || isSplitPending || isAddLpPending || isRemoveLpPending;
  const isQuoteLoading = buyQuoteLoading || sellQuoteLoading;

  const getQuoteDisplay = () => {
    if (mode === "split") return null;
    const quote = action === "buy" ? buyQuote : sellQuote;
    if (!quote) return null;
    return formatUnits(quote, 6);
  };

  const quoteValue = getQuoteDisplay();
  const tokenLabel = mode === "trade-yes" ? "YES" : "NO";
  const currentPrice = mode === "trade-yes" ? yesPrice : noPrice;

  const isTradeDisabled = !isConnected || !amount || isPending || 
    parseFloat(amount || "0") <= 0 || exceedsBalance;

  return (
    <div className="trade-section" data-testid="trade-panel">
      <div className="trade-tabs" data-testid="trade-tabs">
        <button
          className={`trade-tab ${mode === "trade-yes" ? "active trade-yes" : ""}`}
          onClick={() => setMode("trade-yes")}
          data-testid="tab-trade-yes"
        >
          Trade YES
        </button>
        <button
          className={`trade-tab ${mode === "trade-no" ? "active trade-no" : ""}`}
          onClick={() => setMode("trade-no")}
          data-testid="tab-trade-no"
        >
          Trade NO
        </button>
        <button
          className={`trade-tab ${mode === "split" ? "active" : ""}`}
          onClick={() => setMode("split")}
          data-testid="tab-split"
        >
          Split
        </button>
        <button
          className={`trade-tab ${mode === "lp" ? "active" : ""}`}
          onClick={() => setMode("lp")}
          data-testid="tab-lp"
        >
          LP
        </button>
      </div>

      {mode === "lp" && (
        <>
          <div className="lp-token-toggle">
            <button
              className={`lp-toggle-btn ${lpToken === "yes" ? "active yes" : ""}`}
              onClick={() => setLpToken("yes")}
            >
              YES/USD Pool
            </button>
            <button
              className={`lp-toggle-btn ${lpToken === "no" ? "active no" : ""}`}
              onClick={() => setLpToken("no")}
            >
              NO/USD Pool
            </button>
          </div>
          <div className="lp-action-toggle">
            <button
              className={`lp-toggle-btn ${lpAction === "add" ? "active" : ""}`}
              onClick={() => setLpAction("add")}
            >
              Add Liquidity
            </button>
            <button
              className={`lp-toggle-btn ${lpAction === "remove" ? "active" : ""}`}
              onClick={() => setLpAction("remove")}
            >
              Remove Liquidity
            </button>
          </div>
        </>
      )}

      {mode === "lp" && (
        <div className="lp-balance-info">
          <div className="lp-balance-row">
            <span>{lpToken.toUpperCase()}/USD LP Position</span>
            <span>{Number(formatUnits(currentPair.balance, 18)).toFixed(6)} LP</span>
          </div>
          {currentPair.underlyingToken !== undefined && currentPair.underlyingUsd !== undefined && currentPair.balance > 0n && (
            <div className="lp-balance-row">
              <span>Underlying</span>
              <span>{formatUnits(currentPair.underlyingToken, 6)} {lpToken.toUpperCase()} + {formatUnits(currentPair.underlyingUsd, 6)} USD</span>
            </div>
          )}
          {currentPair.reserveToken && currentPair.reserveUsd && currentPair.reserveToken > 0n && (
            <div className="lp-balance-row" style={{ fontSize: '0.6875rem', color: 'var(--text-muted)' }}>
              <span>Pool price</span>
              <span>1 {lpToken.toUpperCase()} = ${(Number(currentPair.reserveUsd) / Number(currentPair.reserveToken)).toFixed(4)}</span>
            </div>
          )}
        </div>
      )}

      <div className="input-group">
        <div className="input-label-row">
          <label className="input-label">
            {mode === "split" 
              ? "Amount to split" 
              : mode === "lp"
                ? lpAction === "add" 
                  ? `${lpToken.toUpperCase()} tokens (USD calculated from price)` 
                  : "LP tokens to remove"
                : `${tokenLabel} tokens to ${action}`}
          </label>
          <div className="input-actions">
            <button
              type="button"
              className="btn-max"
              onClick={handleMaxClick}
              disabled={!isConnected || isPending || (mode === "lp" && lpAction === "remove" ? currentPair.balance === 0n : relevantBalance === 0n)}
            >
              Max
            </button>
          </div>
        </div>
        <input
          type="number"
          className={`input-field ${exceedsBalance ? "input-error" : ""}`}
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
          placeholder="0.00"
          disabled={!isConnected || isPending}
        />
        {exceedsBalance && (
          <div className="input-error-message">Insufficient balance</div>
        )}
        {formattedError && (
          <div className="trade-error">
            <div className="trade-error-title">{formattedError.title}</div>
            <div className="trade-error-message">{formattedError.message}</div>
          </div>
        )}
      </div>

      {mode === "split" && amount && parseFloat(amount) > 0 && (
        <div className="split-info">
          <div className="split-row">
            <span>You deposit</span>
            <span>${amount} USD</span>
          </div>
          <div className="split-arrow">↓</div>
          <div className="split-row">
            <span>You receive</span>
            <span>{amount} YES + {amount} NO</span>
          </div>
        </div>
      )}

      {mode === "lp" && lpAction === "add" && amount && parseFloat(amount) > 0 && lpAmounts && (
        <div className="split-info">
          <div className="split-row">
            <span>You provide</span>
            <span>{formatUnits(lpAmounts.amountToken, 6)} {lpToken.toUpperCase()} + {formatUnits(lpAmounts.amountUsd, 6)} USD</span>
          </div>
          <div className="split-arrow">↓</div>
          <div className="split-row">
            <span>You receive</span>
            <span>~{Number(formatUnits(lpAmounts.estimatedLp, 18)).toFixed(6)} LP tokens</span>
          </div>
        </div>
      )}

      {(mode === "trade-yes" || mode === "trade-no") && amount && parseFloat(amount) > 0 && (
        <div className="quote-box">
          <div className="quote-row">
            <span className="quote-label">
              {action === "buy" ? "You pay" : "You receive"}
            </span>
            <span className="quote-value">
              {isQuoteLoading ? "..." : quoteValue ? `$${quoteValue}` : "—"}
            </span>
          </div>
          <div className="quote-row mt-2">
            <span className="quote-label">Price per token</span>
            <span className="quote-value">
              ${currentPrice?.toFixed(2) ?? "—"}
            </span>
          </div>
        </div>
      )}

      {mode === "split" ? (
        <div className="split-action-buttons">
          <button
            className="btn btn-primary"
            onClick={handleTrade}
            disabled={!isConnected || !amount || isPending || parseFloat(amount || "0") <= 0 || exceedsBalance}
          >
            {isSplitPending ? (
              <span className="inline-flex items-center gap-2">
                <span className="spinner" /> Processing...
              </span>
            ) : !isConnected ? (
              "Connect Wallet"
            ) : (
              "Split USD"
            )}
          </button>
        </div>
      ) : mode === "lp" ? (
        <div className="lp-action-buttons">
          {lpAction === "add" ? (
            <button
              className="btn btn-primary"
              onClick={handleProvideLiquidity}
              disabled={!isConnected || !amount || isPending || parseFloat(amount || "0") <= 0 || !currentTokenAddress || exceedsBalance}
            >
              {isAddLpPending ? (
                <span className="inline-flex items-center gap-2">
                  <span className="spinner" /> Processing...
                </span>
              ) : !isConnected ? (
                "Connect Wallet"
              ) : (
                `Add ${lpToken.toUpperCase()}/USD Liquidity`
              )}
            </button>
          ) : (
            <button
              className="btn btn-red"
              onClick={handleRemoveLiquidity}
              disabled={!isConnected || !amount || isPending || parseFloat(amount || "0") <= 0 || !currentPair.pairAddress || exceedsBalance}
            >
              {isRemoveLpPending ? (
                <span className="inline-flex items-center gap-2">
                  <span className="spinner" /> Processing...
                </span>
              ) : !isConnected ? (
                "Connect Wallet"
              ) : (
                `Remove ${lpToken.toUpperCase()}/USD Liquidity`
              )}
            </button>
          )}
        </div>
      ) : (
        <div className="trade-action-buttons">
          <button
            className={`btn btn-green`}
            onClick={() => {
              setAction("buy");
              if (action === "buy") handleTrade();
            }}
            disabled={!isConnected || (action === "buy" && isTradeDisabled)}
            data-testid="btn-buy"
          >
            {isPending && action === "buy" ? (
              <span className="inline-flex items-center gap-2">
                <span className="spinner" /> Processing...
              </span>
            ) : (
              "Buy"
            )}
          </button>
          <button
            className={`btn btn-red`}
            onClick={() => {
              setAction("sell");
              if (action === "sell") handleTrade();
            }}
            disabled={!isConnected || (action === "sell" && isTradeDisabled)}
            data-testid="btn-sell"
          >
            {isPending && action === "sell" ? (
              <span className="inline-flex items-center gap-2">
                <span className="spinner" /> Processing...
              </span>
            ) : (
              "Sell"
            )}
          </button>
        </div>
      )}
    </div>
  );
}
