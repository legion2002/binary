export interface MarketAsset {
  name: string;
  symbol: string;
}

export interface Market {
  hash: string;
  question: string;
  resolutionDeadline: number;
  assets: MarketAsset[];
}

export interface OpenMarketRequest {
  question: string;
  resolutionDeadline: number;
  assets?: MarketAsset[];
}

export class TestApiClient {
  private serverUrl: string;
  private apiKey: string;

  constructor(
    serverUrl = process.env.SERVER_URL || "http://localhost:3001",
    apiKey = process.env.ADMIN_API_KEY || "test_api_key_12345"
  ) {
    this.serverUrl = serverUrl;
    this.apiKey = apiKey;
  }

  async openMarket(
    question: string,
    resolutionDeadline: number,
    assets?: MarketAsset[]
  ): Promise<Market> {
    const body: OpenMarketRequest = { question, resolutionDeadline };
    if (assets) {
      body.assets = assets;
    }

    const response = await fetch(`${this.serverUrl}/admin/markets/open`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${this.apiKey}`,
      },
      body: JSON.stringify(body),
    });

    if (!response.ok) {
      throw new Error(`Failed to open market: ${response.status} ${response.statusText}`);
    }

    return response.json();
  }

  async getMarkets(): Promise<Market[]> {
    const response = await fetch(`${this.serverUrl}/markets`, {
      headers: {
        "Content-Type": "application/json",
      },
    });

    if (!response.ok) {
      throw new Error(`Failed to get markets: ${response.status} ${response.statusText}`);
    }

    return response.json();
  }

  async getMarket(hash: string): Promise<Market> {
    const response = await fetch(`${this.serverUrl}/markets/${hash}`, {
      headers: {
        "Content-Type": "application/json",
      },
    });

    if (!response.ok) {
      throw new Error(`Failed to get market: ${response.status} ${response.statusText}`);
    }

    return response.json();
  }
}
