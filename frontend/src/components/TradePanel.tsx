import { useState } from "react";
import { parseUnits, formatUnits, type Address } from "viem";
import { useAccount } from "wagmi";
import { useBuyQuote, useBuySync } from "../hooks/useTempoDex";
import { useSplit } from "../hooks/useSplit";

type TradeMode = "buy-yes" | "buy-no" | "split";

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
  const [mode, setMode] = useState<TradeMode>("buy-yes");
  const [amount, setAmount] = useState("");

  const { mutate: buy, isPending: isBuyPending } = useBuySync();
  const { split, isPending: isSplitPending } = useSplit();

  const parsedAmount = amount ? parseUnits(amount, 6) : 0n;
  const exceedsBalance = parsedAmount > selectedBalance;

  const { data: yesBuyQuote, isLoading: yesQuoteLoading } = useBuyQuote({
    tokenIn: selectedAsset,
    tokenOut: (yesTokenAddress ?? selectedAsset) as Address,
    amountOut: parsedAmount,
    query: {
      enabled: mode === "buy-yes" && !!yesTokenAddress && parsedAmount > 0n,
    },
  });

  const { data: noBuyQuote, isLoading: noQuoteLoading } = useBuyQuote({
    tokenIn: selectedAsset,
    tokenOut: (noTokenAddress ?? selectedAsset) as Address,
    amountOut: parsedAmount,
    query: {
      enabled: mode === "buy-no" && !!noTokenAddress && parsedAmount > 0n,
    },
  });

  const handleTrade = async () => {
    console.log("[TradePanel] handleTrade called:", { mode, amount, isConnected, exceedsBalance, parsedAmount: parsedAmount.toString() });
    
    if (!amount || !isConnected || exceedsBalance) {
      console.log("[TradePanel] Early return:", { noAmount: !amount, notConnected: !isConnected, exceedsBalance });
      return;
    }

    if (mode === "buy-yes" && yesTokenAddress) {
      buy({
        tokenIn: selectedAsset,
        tokenOut: yesTokenAddress as Address,
        amountOut: parsedAmount,
        maxAmountIn: yesBuyQuote ? (yesBuyQuote * 105n) / 100n : parsedAmount * 2n,
      });
    } else if (mode === "buy-no" && noTokenAddress) {
      buy({
        tokenIn: selectedAsset,
        tokenOut: noTokenAddress as Address,
        amountOut: parsedAmount,
        maxAmountIn: noBuyQuote ? (noBuyQuote * 105n) / 100n : parsedAmount * 2n,
      });
    } else if (mode === "split") {
      console.log("[TradePanel] Calling split...");
      try {
        await split({
          asset: selectedAsset,
          amount: parsedAmount,
          marketHash,
        });
        console.log("[TradePanel] Split completed");
      } catch (err) {
        console.error("[TradePanel] Split error:", err);
      }
    }
  };

  const handleMaxClick = () => {
    const maxAmount = Number(selectedBalance) / 1e6;
    setAmount(maxAmount.toString());
  };

  const isPending = isBuyPending || isSplitPending;
  const isQuoteLoading = yesQuoteLoading || noQuoteLoading;

  const getQuoteDisplay = () => {
    if (mode === "split") {
      return null;
    }
    const quote = mode === "buy-yes" ? yesBuyQuote : noBuyQuote;
    if (!quote) return null;
    return formatUnits(quote, 6);
  };

  const quoteValue = getQuoteDisplay();

  return (
    <div className="trade-section" data-testid="trade-panel">
      <div className="trade-tabs" data-testid="trade-tabs">
        <button
          className={`trade-tab ${mode === "buy-yes" ? "active buy-yes" : ""}`}
          onClick={() => setMode("buy-yes")}
          data-testid="tab-buy-yes"
        >
          Buy YES
        </button>
        <button
          className={`trade-tab ${mode === "buy-no" ? "active buy-no" : ""}`}
          onClick={() => setMode("buy-no")}
          data-testid="tab-buy-no"
        >
          Buy NO
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
            {mode === "split" ? "Amount to split" : "Tokens to buy"}
          </label>
          <button
            type="button"
            className="btn-max"
            onClick={handleMaxClick}
            disabled={!isConnected || isPending || selectedBalance === 0n}
          >
            Max
          </button>
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

      {mode !== "split" && amount && parseFloat(amount) > 0 && (
        <div className="quote-box">
          <div className="quote-row">
            <span className="quote-label">You pay</span>
            <span className="quote-value">
              {isQuoteLoading ? "..." : quoteValue ? `$${quoteValue}` : "—"}
            </span>
          </div>
          <div className="quote-row mt-2">
            <span className="quote-label">Price per token</span>
            <span className="quote-value">
              ${mode === "buy-yes" ? (yesPrice?.toFixed(2) ?? "—") : (noPrice?.toFixed(2) ?? "—")}
            </span>
          </div>
        </div>
      )}

      <button
        className={`btn ${
          mode === "buy-yes"
            ? "btn-green"
            : mode === "buy-no"
            ? "btn-red"
            : "btn-primary"
        }`}
        onClick={handleTrade}
        disabled={!isConnected || !amount || isPending || parseFloat(amount || "0") <= 0 || exceedsBalance}
      >
        {isPending ? (
          <span className="inline-flex items-center gap-2">
            <span className="spinner" /> Processing...
          </span>
        ) : !isConnected ? (
          "Connect Wallet"
        ) : mode === "buy-yes" ? (
          "Buy YES Tokens"
        ) : mode === "buy-no" ? (
          "Buy NO Tokens"
        ) : (
          "Split USD"
        )}
      </button>
    </div>
  );
}
