import { useEffect, useState } from 'react';
import { buildChartUrl, getDeviceHeight, TRADING_CONFIG } from '../config/trading';

interface PriceChartProps {
  pairAddress: string;
  verse: 'YES' | 'NO';
}

export function PriceChart({ pairAddress, verse }: PriceChartProps) {
  const [height, setHeight] = useState(getDeviceHeight(TRADING_CONFIG.charts.height));

  // Update height on window resize
  useEffect(() => {
    const handleResize = () => {
      setHeight(getDeviceHeight(TRADING_CONFIG.charts.height));
    };

    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, []);

  const chartUrl = buildChartUrl(pairAddress);
  const provider = TRADING_CONFIG.charts.provider;

  return (
    <div className="w-full bg-white rounded-lg overflow-hidden shadow-sm">
      <div className="px-4 py-3 bg-gradient-to-r from-gray-50 to-slate-50 border-b border-gray-200">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <h4 className="text-sm font-semibold text-gray-700">
              {verse}-ETH/USDC Price Chart
            </h4>
            <span className={`px-2 py-1 text-xs font-medium rounded-full ${
              verse === 'YES'
                ? 'bg-green-100 text-green-700'
                : 'bg-red-100 text-red-700'
            }`}>
              {verse}
            </span>
          </div>
          <div className="flex items-center gap-2">
            <span className="text-xs text-gray-500">
              {provider === 'dextools' ? 'DEXTools' : 'GeckoTerminal'}
            </span>
            <svg width="16" height="16" viewBox="0 0 24 24" className="text-gray-400">
              <path fill="currentColor" d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM9 17H7v-7h2v7zm4 0h-2V7h2v10zm4 0h-2v-4h2v4z"/>
            </svg>
          </div>
        </div>
      </div>

      <iframe
        src={chartUrl}
        height={height}
        width="100%"
        style={{
          border: 'none',
          display: 'block',
          minHeight: '300px'
        }}
        title={`${verse} Token Price Chart`}
        sandbox="allow-scripts allow-same-origin allow-popups"
      />

      <div className="px-4 py-2 bg-gray-50 border-t border-gray-100">
        <div className="flex items-center justify-between text-xs text-gray-500">
          <span>30m candles • Live data</span>
          <a
            href={chartUrl}
            target="_blank"
            rel="noopener noreferrer"
            className="text-blue-600 hover:text-blue-700 hover:underline flex items-center gap-1"
          >
            Open in {provider === 'dextools' ? 'DEXTools' : 'GeckoTerminal'}
            <svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor">
              <path d="M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z"/>
            </svg>
          </a>
        </div>
      </div>
    </div>
  );
}