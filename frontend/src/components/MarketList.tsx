import { useQuery } from "@tanstack/react-query";
import { fetchMarkets } from "../api/client";
import { MarketCard } from "./MarketCard";

export function MarketList() {
  const { data, isLoading, error } = useQuery({
    queryKey: ["markets"],
    queryFn: () => fetchMarkets(50, 0),
  });

  if (isLoading) {
    return (
      <div className="loading">
        <div className="spinner" />
        <p className="mt-4">Loading markets...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="empty-state">
        <p className="text-red mb-2">Failed to load markets</p>
        <p className="text-sm text-muted">{(error as Error).message}</p>
      </div>
    );
  }

  if (!data || data.markets.length === 0) {
    return (
      <div className="empty-state" data-testid="empty-state">
        <p>No markets available</p>
      </div>
    );
  }

  return (
    <div data-testid="markets-section">
      <div className="section-title" data-testid="markets-count">
        Markets ({data.count})
      </div>
      {data.markets.map((market) => (
        <MarketCard key={market.marketHash} market={market} />
      ))}
    </div>
  );
}
