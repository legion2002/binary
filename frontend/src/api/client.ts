import type {
  ConfigResponse,
  GetMarketsResponse,
  MarketDetailResponse,
  VerseTokenResponse,
} from './types';

const API_URL = import.meta.env.VITE_API_URL || 'http://127.0.0.1:3000';

// Cached config - fetched once at startup
let cachedConfig: ConfigResponse | null = null;
let configPromise: Promise<ConfigResponse> | null = null;

/**
 * Fetch backend config (contract addresses, etc.)
 * Results are cached for the lifetime of the app
 */
export async function fetchConfig(): Promise<ConfigResponse> {
  // Return cached config if available
  if (cachedConfig) {
    return cachedConfig;
  }

  // If a fetch is in progress, wait for it
  if (configPromise) {
    return configPromise;
  }

  // Start the fetch
  configPromise = (async () => {
    const response = await fetch(`${API_URL}/config`);

    if (!response.ok) {
      throw new Error(`Failed to fetch config: ${response.statusText}`);
    }

    const config = await response.json() as ConfigResponse;
    cachedConfig = config;
    return config;
  })();

  return configPromise;
}

/**
 * Get cached config synchronously (returns null if not yet fetched)
 */
export function getCachedConfig(): ConfigResponse | null {
  return cachedConfig;
}

/**
 * Fetch all markets with pagination
 */
export async function fetchMarkets(
  limit = 50,
  offset = 0
): Promise<GetMarketsResponse> {
  const response = await fetch(
    `${API_URL}/markets?limit=${limit}&offset=${offset}`
  );

  if (!response.ok) {
    throw new Error(`Failed to fetch markets: ${response.statusText}`);
  }

  return response.json();
}

/**
 * Fetch a single market by its hash
 */
export async function fetchMarket(
  marketHash: string
): Promise<MarketDetailResponse> {
  const response = await fetch(`${API_URL}/markets/${marketHash}`);

  if (!response.ok) {
    if (response.status === 404) {
      throw new Error('Market not found');
    }
    throw new Error(`Failed to fetch market: ${response.statusText}`);
  }

  return response.json();
}

/**
 * Fetch verse tokens for a specific market
 */
export async function fetchVerseTokens(
  marketHash: string
): Promise<VerseTokenResponse[]> {
  const response = await fetch(`${API_URL}/markets/${marketHash}/verse-tokens`);

  if (!response.ok) {
    if (response.status === 404) {
      throw new Error('Market not found');
    }
    throw new Error(`Failed to fetch verse tokens: ${response.statusText}`);
  }

  return response.json();
}
