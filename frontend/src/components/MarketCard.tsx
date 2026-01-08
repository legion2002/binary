import { useState } from "react";
import type { Address } from "viem";
import { useQuery } from "@tanstack/react-query";
import { fetchMarket } from "../api/client";
import { usePriceQuotes } from "../hooks/usePriceQuotes";
import { useVerseTokens } from "../hooks/useVerseTokens";
import { STABLECOINS } from "../hooks/useStablecoinBalances";
import { TradePanel } from "./TradePanel";
import { AssetSelector } from "./AssetSelector";
import { TokenBalances } from "./TokenBalances";
import type { MarketResponse } from "../api/types";

// Default asset for new selections (AlphaUSD - static precompile address)
const DEFAULT_ASSET = "0x20C0000000000000000000000000000000000001" as Address;

interface MarketCardProps {
  market: MarketResponse;
}

export function MarketCard({ market }: MarketCardProps) {
  const [isExpanded, setIsExpanded] = useState(false);
  const [selectedAsset, setSelectedAsset] = useState<Address>(DEFAULT_ASSET);
  const [selectedBalance, setSelectedBalance] = useState<bigint>(0n);

  const handleAssetChange = (asset: Address, balance: bigint) => {
    setSelectedAsset(asset);
    setSelectedBalance(balance);
  };

  const { data: marketDetail } = useQuery({
    queryKey: ["market", market.marketHash],
    queryFn: () => fetchMarket(market.marketHash),
    enabled: isExpanded,
  });

  // Query verse tokens directly from the contract (more reliable than backend)
  const { data: verseTokens } = useVerseTokens(
    isExpanded ? selectedAsset : undefined,
    isExpanded ? market.marketHash : undefined
  );

  const selectedStablecoin = STABLECOINS.find(
    (s) => s.address.toLowerCase() === selectedAsset.toLowerCase()
  );

  const { yesPrice, noPrice, yesProbability, noProbability, isLoading: priceLoading } =
    usePriceQuotes(verseTokens?.yesVerse, verseTokens?.noVerse);

  // Use backend's mid-price probability when expanded, fall back to list API probability
  const displayProbability = isExpanded && marketDetail?.yesProbability != null
    ? Math.round(marketDetail.yesProbability * 100)
    : (market.yesProbability != null ? Math.round(market.yesProbability * 100) : 50);

  const formatDeadline = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleDateString("en-US", {
      month: "short",
      day: "numeric",
      year: "numeric",
    });
  };

  const isResolved = market.resolution && market.resolution !== "UNRESOLVED";

  return (
    <div className={`market-card ${isExpanded ? "expanded" : ""}`} data-testid="market-card">
      <div className="market-header" onClick={() => setIsExpanded(!isExpanded)}>
        <div className="flex-1">
          <div className="market-question" data-testid="market-question">
            {market.question || "Question not available"}
          </div>
          <div className="market-meta">
            Resolves {formatDeadline(market.resolutionDeadline)}
            {isResolved && (
              <span
                className={`status-badge ml-2 resolved-${market.resolution?.toLowerCase()}`}
              >
                {market.resolution}
              </span>
            )}
          </div>
        </div>
        <div className="text-right">
          <div className="market-probability" data-testid="market-probability">{displayProbability}%</div>
        </div>
      </div>

      {isExpanded && (
        <div className="market-content" data-testid="market-content">
          <div className="expanded-probability" data-testid="price-grid">
            <div className="expanded-probability-labels">
              <div className="expanded-prob-label yes">
                <span className="expanded-prob-pct">{yesProbability}%</span>
                <span className="expanded-prob-price">
                  {priceLoading ? "..." : yesPrice != null ? `$${yesPrice.toFixed(2)}` : "$0.50"}
                </span>
              </div>
              <div className="expanded-prob-label no">
                <span className="expanded-prob-price">
                  {priceLoading ? "..." : noPrice != null ? `$${noPrice.toFixed(2)}` : "$0.50"}
                </span>
                <span className="expanded-prob-pct">{noProbability}%</span>
              </div>
            </div>
            <div className="expanded-probability-bar">
              <div
                className="probability-bar-yes"
                style={{ width: `${displayProbability}%` }}
              />
              <div
                className="probability-bar-no"
                style={{ width: `${100 - displayProbability}%` }}
              />
            </div>
          </div>

          {!isResolved && (
            <>
              <AssetSelector
                selectedAsset={selectedAsset}
                onAssetChange={handleAssetChange}
              />
              <TokenBalances
                yesTokenAddress={verseTokens?.yesVerse}
                noTokenAddress={verseTokens?.noVerse}
                assetSymbol={selectedStablecoin?.symbol ?? "USD"}
              />
              <TradePanel
                marketHash={market.marketHash}
                yesTokenAddress={verseTokens?.yesVerse}
                noTokenAddress={verseTokens?.noVerse}
                yesPrice={yesPrice}
                noPrice={noPrice}
                selectedAsset={selectedAsset}
                selectedBalance={selectedBalance}
              />
            </>
          )}

          {isResolved && (
            <div className="empty-state">
              This market has been resolved to {market.resolution}.
              {market.resolution === "YES" && " YES token holders can redeem $1 per token."}
              {market.resolution === "NO" && " NO token holders can redeem $1 per token."}
              {market.resolution === "EVEN" && " Both YES and NO holders can redeem $0.50 per token."}
            </div>
          )}
        </div>
      )}
    </div>
  );
}
