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

export interface MarketDetailResponse {
  marketHash: string;
  questionHash: string;
  question?: string;
  resolutionDeadline: number;
  oracle: string;
  blockNumber: number;
  verseTokens: VerseTokenResponse[];
}

export interface ErrorResponse {
  error: string;
}
