import { useEffect, useState } from 'react';
import { buildUniswapUrl, getDeviceHeight, TRADING_CONFIG } from '../config/trading';

interface UniswapSwapWidgetProps {
  tokenAddress: string;
  verse: 'YES' | 'NO';
}

export function UniswapSwapWidget({ tokenAddress, verse }: UniswapSwapWidgetProps) {
  const [height, setHeight] = useState(getDeviceHeight(TRADING_CONFIG.uniswap.height));

  // Update height on window resize
  useEffect(() => {
    const handleResize = () => {
      setHeight(getDeviceHeight(TRADING_CONFIG.uniswap.height));
    };

    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, []);

  const swapUrl = buildUniswapUrl(tokenAddress);

  return (
    <div className="w-full bg-white rounded-lg overflow-hidden shadow-sm">
      <div className="px-4 py-3 bg-gradient-to-r from-purple-50 to-blue-50 border-b border-gray-200">
        <div className="flex items-center justify-between">
          <h4 className="text-sm font-semibold text-gray-700">
            Swap {verse} Tokens
          </h4>
          <div className="flex items-center gap-2">
            <span className="text-xs text-gray-500">Powered by</span>
            <svg width="24" height="24" viewBox="0 0 24 24" className="text-pink-500">
              <path fill="currentColor" d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm3.53 6.47L13 11l2.53 2.53c.29.29.29.77 0 1.06-.15.15-.34.22-.53.22s-.38-.07-.53-.22L12 12.06l-2.47 2.47c-.15.15-.34.22-.53.22s-.38-.07-.53-.22c-.29-.29-.29-.77 0-1.06L11 11 8.47 8.47c-.29-.29-.29-.77 0-1.06.29-.29.77-.29 1.06 0L12 9.94l2.47-2.47c.29-.29.77-.29 1.06 0 .29.29.29.77 0 1.06z"/>
            </svg>
          </div>
        </div>
      </div>

      <iframe
        src={swapUrl}
        height={height}
        width="100%"
        style={{
          border: 'none',
          display: 'block',
          minHeight: '500px'
        }}
        title={`Uniswap Swap Widget - ${verse} Tokens`}
        sandbox="allow-scripts allow-same-origin allow-popups allow-forms allow-modals allow-storage-access-by-user-activation"
      />

      <div className="px-4 py-2 bg-gray-50 border-t border-gray-100">
        <p className="text-xs text-gray-500 text-center">
          Trading on Base Sepolia • Slippage: {TRADING_CONFIG.uniswap.defaultSlippage}%
        </p>
      </div>
    </div>
  );
}