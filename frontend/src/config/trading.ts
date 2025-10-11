// Trading widget configuration
export const TRADING_CONFIG = {
  // Uniswap Swap Widget Configuration
  uniswap: {
    baseUrl: 'https://app.uniswap.org/#/swap',
    chain: 'base-sepolia',
    height: {
      desktop: '600px',
      tablet: '550px',
      mobile: '500px'
    },
    defaultSlippage: 0.5,
    exactIn: false, // We want exact output (buying YES/NO tokens)
  },

  // Price Chart Configuration
  charts: {
    provider: 'dextools' as 'dextools' | 'geckoterminal',
    dextools: {
      baseUrl: 'https://www.dextools.io/widget-chart/en/base/pe-light',
      theme: 'light',
      chartType: 1, // 1 = candlestick, 2 = line
      chartResolution: 30, // 30 minute candles
      drawingToolbars: false,
    },
    geckoterminal: {
      baseUrl: 'https://www.geckoterminal.com/base/pools',
      embed: true,
    },
    height: {
      desktop: '400px',
      tablet: '350px',
      mobile: '300px'
    }
  },

  // Responsive breakpoints
  breakpoints: {
    mobile: 640,
    tablet: 768,
    desktop: 1024
  }
};

// Helper function to get device-specific height
export function getDeviceHeight(config: { desktop: string; tablet: string; mobile: string }): string {
  if (typeof window === 'undefined') return config.desktop;

  const width = window.innerWidth;
  if (width < TRADING_CONFIG.breakpoints.mobile) return config.mobile;
  if (width < TRADING_CONFIG.breakpoints.desktop) return config.tablet;
  return config.desktop;
}

// Helper function to build Uniswap URL
export function buildUniswapUrl(tokenAddress: string): string {
  return `${TRADING_CONFIG.uniswap.baseUrl}?outputCurrency=${tokenAddress}&chain=${TRADING_CONFIG.uniswap.chain}`;
}

// Helper function to build chart URL
export function buildChartUrl(pairAddress: string): string {
  const { provider, dextools, geckoterminal } = TRADING_CONFIG.charts;

  if (provider === 'dextools') {
    const params = new URLSearchParams({
      theme: dextools.theme,
      chartType: dextools.chartType.toString(),
      chartResolution: dextools.chartResolution.toString(),
      drawingToolbars: dextools.drawingToolbars.toString()
    });
    return `${dextools.baseUrl}/${pairAddress}?${params.toString()}`;
  } else {
    // GeckoTerminal format
    return `${geckoterminal.baseUrl}/${pairAddress}${geckoterminal.embed ? '?embed=1&info=0' : ''}`;
  }
}