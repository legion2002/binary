import { Link } from 'react-router-dom';
import { useQuery } from '@tanstack/react-query';
import { fetchMarkets } from '../api/client';

// Probability bar component
function ProbabilityBar({ yes, no }: { yes: number; no: number }) {
  const yesPercent = Math.round(yes * 100);
  const noPercent = Math.round(no * 100);

  return (
    <div className="mt-3">
      <div className="probability-bar">
        <div
          className="probability-bar-yes"
          style={{ width: `${yesPercent}%` }}
        />
        <div
          className="probability-bar-no"
          style={{ width: `${noPercent}%` }}
        />
      </div>
      <div className="flex justify-between mt-2 text-sm">
        <span className="text-accent-green font-medium">YES {yesPercent}%</span>
        <span className="text-accent-red font-medium">NO {noPercent}%</span>
      </div>
    </div>
  );
}

export function MarketList() {
  const { data, isLoading, error } = useQuery({
    queryKey: ['markets'],
    queryFn: () => fetchMarkets(50, 0),
  });

  const formatDeadline = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
    });
  };

  if (isLoading) {
    return (
      <div className="text-center py-12">
        <div className="inline-block w-8 h-8 spinner-dark"></div>
        <p className="mt-4 text-secondary">Loading markets...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="text-center py-12">
        <p className="text-accent-red mb-2">Failed to load markets</p>
        <p className="text-sm text-muted">{(error as Error).message}</p>
      </div>
    );
  }

  if (!data || data.markets.length === 0) {
    return (
      <div className="text-center py-12">
        <p className="text-secondary">No markets available</p>
      </div>
    );
  }

  return (
    <div>
      <h2 className="text-2xl font-bold mb-6 text-primary">Active Markets</h2>
      <div className="text-sm text-muted mb-4">
        Showing {data.count} of {data.total} markets
      </div>
      <div className="grid gap-4">
        {data.markets.map((market) => (
          <Link
            key={market.marketHash}
            to={`/market/${market.marketHash}`}
            className="block card-dark p-6 transition-all hover:border-dark-hover"
          >
            <h3 className="text-lg font-semibold mb-2 text-primary">
              {market.question || 'Question not available'}
            </h3>

            {/* Show probability bar if data available */}
            {market.yesProbability != null && market.noProbability != null ? (
              <ProbabilityBar
                yes={market.yesProbability}
                no={market.noProbability}
              />
            ) : (
              // Default 50/50 bar for markets without probability data
              <ProbabilityBar yes={0.5} no={0.5} />
            )}

            <div className="flex justify-between items-center text-sm mt-4 pt-3 border-t border-dark">
              <span className="text-muted">
                Resolves: {formatDeadline(market.resolutionDeadline)}
              </span>
              <span className="text-accent-purple font-medium">
                View Market →
              </span>
            </div>
          </Link>
        ))}
      </div>
    </div>
  );
}
