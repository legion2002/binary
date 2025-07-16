import { Link } from 'react-router-dom';
import { MOCK_MARKETS } from '../config/contracts';

export function MarketList() {
  // In production, this would fetch from the contract
  const markets = MOCK_MARKETS;

  const formatDeadline = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
    });
  };

  return (
    <div>
      <h2 className="text-2xl font-bold mb-6">Active Markets</h2>
      <div className="grid gap-4">
        {markets.map((market) => (
          <Link
            key={market.marketHash}
            to={`/market/${market.marketHash}`}
            className="block bg-white rounded-lg shadow hover:shadow-md transition-shadow p-6"
          >
            <h3 className="text-lg font-semibold mb-2">{market.question}</h3>
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