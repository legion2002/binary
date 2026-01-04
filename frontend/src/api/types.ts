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
}

export interface ErrorResponse {
  error: string;
}
