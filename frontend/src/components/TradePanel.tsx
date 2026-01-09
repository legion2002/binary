import { useState } from "react";
import { parseUnits, formatUnits, type Address } from "viem";
import { Tick } from "viem/tempo";
import { useAccount } from "wagmi";
import { useBuyQuote, useBuySync, useSellQuote, useSellSync, usePlaceOrderSync } from "../hooks/useTempoDex";
import { useSplit } from "../hooks/useSplit";

type TradeMode = "trade-yes" | "trade-no" | "split";
type ActionType = "buy" | "sell";
type OrderType = "market" | "limit";

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
  const { isConnected } = useAccount();
  const [mode, setMode] = useState<TradeMode>("trade-yes");
  const [action, setAction] = useState<ActionType>("buy");
  const [orderType, setOrderType] = useState<OrderType>("market");
  const [amount, setAmount] = useState("");
  const [limitPrice, setLimitPrice] = useState("");
  const [tradeError, setTradeError] = useState<string | null>(null);

  const { mutate: buy, isPending: isBuyPending, error: buyError } = useBuySync();
  const { mutate: sell, isPending: isSellPending, error: sellError } = useSellSync();
  const { mutate: placeOrder, isPending: isPlacePending, error: placeError } = usePlaceOrderSync();
  const { split, isPending: isSplitPending } = useSplit();

  const displayError = tradeError || (buyError?.message) || (sellError?.message) || (placeError?.message);

  const parsedAmount = amount ? parseUnits(amount, 6) : 0n;
  const exceedsBalance = parsedAmount > selectedBalance;

  const currentToken = mode === "trade-yes" ? yesTokenAddress : noTokenAddress;

  // Buy quotes: how much USD to pay for X tokens
  const { data: buyQuote, isLoading: buyQuoteLoading } = useBuyQuote({
    tokenIn: selectedAsset,
    tokenOut: (currentToken ?? selectedAsset) as Address,
    amountOut: parsedAmount,
    query: {
      enabled: mode !== "split" && action === "buy" && orderType === "market" && !!currentToken && parsedAmount > 0n,
    },
  });

  // Sell quotes: how much USD received for selling X tokens
  const { data: sellQuote, isLoading: sellQuoteLoading } = useSellQuote({
    tokenIn: (currentToken ?? selectedAsset) as Address,
    tokenOut: selectedAsset,
    amountIn: parsedAmount,
    query: {
      enabled: mode !== "split" && action === "sell" && orderType === "market" && !!currentToken && parsedAmount > 0n,
    },
  });

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

    if (orderType === "limit") {
      if (!limitPrice || parseFloat(limitPrice) <= 0) return;
      
      const tick = Tick.fromPrice(limitPrice);
      placeOrder({
        token: currentToken as Address,
        amount: parsedAmount,
        tick,
        type: action,
      });
    } else {
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
    }
  };

  const handleMaxClick = () => {
    const maxAmount = Number(selectedBalance) / 1e6;
    setAmount(maxAmount.toString());
  };

  const isPending = isBuyPending || isSellPending || isSplitPending || isPlacePending;
  const isQuoteLoading = buyQuoteLoading || sellQuoteLoading;

  const getQuoteDisplay = () => {
    if (mode === "split" || orderType === "limit") return null;
    const quote = action === "buy" ? buyQuote : sellQuote;
    if (!quote) return null;
    return formatUnits(quote, 6);
  };

  const quoteValue = getQuoteDisplay();
  const tokenLabel = mode === "trade-yes" ? "YES" : "NO";
  const currentPrice = mode === "trade-yes" ? yesPrice : noPrice;

  const isTradeDisabled = !isConnected || !amount || isPending || 
    parseFloat(amount || "0") <= 0 || exceedsBalance ||
    (orderType === "limit" && (!limitPrice || parseFloat(limitPrice) <= 0));

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
      </div>

      <div className="input-group">
        <div className="input-label-row">
          <label className="input-label">
            {mode === "split" ? "Amount to split" : `${tokenLabel} tokens to ${action}`}
          </label>
          <div className="input-actions">
            {mode !== "split" && (
              <div className="order-type-toggle" data-testid="order-type-toggle">
                <button
                  className={`order-type-btn ${orderType === "market" ? "active" : ""}`}
                  onClick={() => setOrderType("market")}
                  data-testid="order-type-market"
                >
                  Market
                </button>
                <button
                  className={`order-type-btn ${orderType === "limit" ? "active" : ""}`}
                  onClick={() => setOrderType("limit")}
                  data-testid="order-type-limit"
                >
                  Limit
                </button>
              </div>
            )}
            <button
              type="button"
              className="btn-max"
              onClick={handleMaxClick}
              disabled={!isConnected || isPending || selectedBalance === 0n}
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
        {displayError && (
          <div className="input-error-message">{displayError}</div>
        )}
      </div>

      {mode !== "split" && orderType === "limit" && (
        <div className="input-group">
          <label className="input-label">Price per token</label>
          <input
            type="number"
            className="input-field"
            value={limitPrice}
            onChange={(e) => setLimitPrice(e.target.value)}
            placeholder="0.00"
            step="0.01"
            min="0.01"
            max="0.99"
            disabled={!isConnected || isPending}
            data-testid="limit-price-input"
          />
          <div className="input-hint">Enter price between $0.01 and $0.99</div>
        </div>
      )}

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

      {mode !== "split" && orderType === "market" && amount && parseFloat(amount) > 0 && (
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

      {mode !== "split" && orderType === "limit" && amount && parseFloat(amount) > 0 && limitPrice && parseFloat(limitPrice) > 0 && (
        <div className="quote-box">
          <div className="quote-row">
            <span className="quote-label">
              {action === "buy" ? "Total cost" : "Total received"}
            </span>
            <span className="quote-value">
              ${(parseFloat(amount) * parseFloat(limitPrice)).toFixed(2)}
            </span>
          </div>
          <div className="quote-row mt-2">
            <span className="quote-label">Limit price</span>
            <span className="quote-value">${limitPrice}</span>
          </div>
        </div>
      )}

      {mode === "split" ? (
        <button
          className="btn btn-primary"
          onClick={handleTrade}
          disabled={!isConnected || !amount || isPending || parseFloat(amount || "0") <= 0 || exceedsBalance}
        >
          {isPending ? (
            <span className="inline-flex items-center gap-2">
              <span className="spinner" /> Processing...
            </span>
          ) : !isConnected ? (
            "Connect Wallet"
          ) : (
            "Split USD"
          )}
        </button>
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
