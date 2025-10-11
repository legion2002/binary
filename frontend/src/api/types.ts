// API Response Types matching backend Rust types

export interface MarketResponse {
  marketHash: string;
  questionHash: string;
  question?: string;
  resolutionDeadline: number;
  oracle: string;
  blockNumber: number;
}

export interface GetMarketsResponse {
  markets: MarketResponse[];
  count: number;
  total: number;
}

export interface VerseTokenResponse {
  asset: string;
  yesVerse: string;
  noVerse: string;
  transactionHash?: string;
}

export interface V4PoolInfo {
  poolType: string;
  poolId: string;
  token0: string;
  token1: string;
  fee: number;
  tickSpacing: number;
  liquidity?: string | null;
  sqrtPriceX96?: string | null;
  tick?: number | null;
}

export interface MarketDetailResponse {
  marketHash: string;
  questionHash: string;
  question?: string;
  resolutionDeadline: number;
  oracle: string;
  blockNumber: number;
  verseTokens: VerseTokenResponse[];
  v4Pools: V4PoolInfo[];
}

export interface ErrorResponse {
  error: string;
}
