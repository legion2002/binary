// API Response Types matching backend Rust types

export type Resolution = 'UNRESOLVED' | 'YES' | 'NO' | 'EVEN';

export interface MarketResponse {
  marketHash: string;
  questionHash: string;
  question?: string;
  resolutionDeadline: number;
  oracle: string;
  blockNumber: number;
  // Probability derived from orderbook prices (0.0 - 1.0)
  // Note: null in list view for performance, populated in detail view
  yesProbability?: number | null;
  noProbability?: number | null;
  // Resolution status from on-chain contract
  resolution?: Resolution | null;
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

export interface OrderbookInfo {
  pairType: string;
  pairKey: string;
  baseToken: string;
  quoteToken: string;
  bestBidTick?: number | null;
  bestAskTick?: number | null;
  bestBidPrice?: string | null;
  bestAskPrice?: string | null;
  midPrice?: string | null;
  spreadBps?: number | null;
}

export interface MarketDetailResponse {
  marketHash: string;
  questionHash: string;
  question?: string;
  resolutionDeadline: number;
  oracle: string;
  blockNumber: number;
  verseTokens: VerseTokenResponse[];
  orderbooks: OrderbookInfo[];
  // Probability derived from live orderbook prices (0.0 - 1.0)
  yesProbability?: number | null;
  noProbability?: number | null;
  // Resolution status from on-chain contract
  resolution?: Resolution | null;
}

export interface ErrorResponse {
  error: string;
}

export interface ConfigResponse {
  multiverseAddress: string;
  oracleAddress: string;
}
