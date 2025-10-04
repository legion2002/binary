import { Link } from 'react-router-dom';
import { useQuery } from '@tanstack/react-query';
import { fetchMarkets } from '../api/client';

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
        <div className="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
        <p className="mt-4 text-gray-600">Loading markets...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="text-center py-12">
        <p className="text-red-600 mb-2">Failed to load markets</p>
        <p className="text-sm text-gray-600">{(error as Error).message}</p>
      </div>
    );
  }

  if (!data || data.markets.length === 0) {
    return (
      <div className="text-center py-12">
        <p className="text-gray-600">No markets available</p>
      </div>
    );
  }

  return (
    <div>
      <h2 className="text-2xl font-bold mb-6">Active Markets</h2>
      <div className="text-sm text-gray-600 mb-4">
        Showing {data.count} of {data.total} markets
      </div>
      <div className="grid gap-4">
        {data.markets.map((market) => (
          <Link
            key={market.marketHash}
            to={`/market/${market.marketHash}`}
            className="block bg-white rounded-lg shadow hover:shadow-md transition-shadow p-6"
          >
            <h3 className="text-lg font-semibold mb-2">
              {market.question || 'Question not available'}
            </h3>
            <div className="flex justify-between items-center text-sm text-gray-600">
              <span>Resolution: {formatDeadline(market.resolutionDeadline)}</span>
              <span className="text-blue-600">View Market →</span>
            </div>
          </Link>
        ))}
      </div>
    </div>
  );
}