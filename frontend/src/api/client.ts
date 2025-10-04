import type {
  GetMarketsResponse,
  MarketDetailResponse,
  VerseTokenResponse,
} from './types';

const API_URL = import.meta.env.VITE_API_URL || 'http://127.0.0.1:3000';

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
