import { useParams, useNavigate } from 'react-router-dom';
import { useState } from 'react';
import { useAccount } from 'wagmi';
import { MOCK_MARKETS } from '../config/contracts';
import { VersePanel } from './VersePanel';
import { SplitCombine } from './SplitCombine';

export function MarketDetail() {
  const { marketHash } = useParams<{ marketHash: string }>();
  const navigate = useNavigate();
  const { isConnected } = useAccount();
  const [activeAsset, setActiveAsset] = useState<'ETH' | 'USDC'>('ETH');

  // In production, fetch from contract
  const market = MOCK_MARKETS.find(m => m.marketHash === marketHash);

  if (!market) {
    return (
      <div className="text-center py-12">
        <h2 className="text-xl font-semibold mb-4">Market not found</h2>
        <button
          onClick={() => navigate('/')}
          className="text-blue-600 hover:text-blue-700"
        >
          ← Back to markets
        </button>
      </div>
    );
  }

  return (
    <div>
      <button
        onClick={() => navigate('/')}
        className="text-blue-600 hover:text-blue-700 mb-4"
      >
        ← Back to markets
      </button>

      <div className="bg-white rounded-lg shadow-lg p-6 mb-6">
        <h2 className="text-2xl font-bold mb-2">{market.question}</h2>
        <p className="text-gray-600">
          Resolution: {new Date(market.resolutionDeadline * 1000).toLocaleDateString()}
        </p>
      </div>

      {!isConnected && (
        <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-4 mb-6">
          <p className="text-yellow-800">Please connect your wallet to trade</p>
        </div>
      )}

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 relative">
        <VersePanel
          verse="YES"
          marketHash={market.marketHash}
          activeAsset={activeAsset}
          onAssetChange={setActiveAsset}
        />
        
        <div className="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 z-10 hidden lg:block">
          <SplitCombine
            marketHash={market.marketHash}
            activeAsset={activeAsset}
          />
        </div>

        <VersePanel
          verse="NO"
          marketHash={market.marketHash}
          activeAsset={activeAsset}
          onAssetChange={setActiveAsset}
        />
      </div>

      <div className="lg:hidden mt-6">
        <SplitCombine
          marketHash={market.marketHash}
          activeAsset={activeAsset}
        />
      </div>
    </div>
  );
}