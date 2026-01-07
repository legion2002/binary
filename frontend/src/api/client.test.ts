import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { fetchMarkets, fetchMarket } from "./client";

describe("API Client", () => {
  const originalFetch = globalThis.fetch;

  beforeEach(() => {
    globalThis.fetch = vi.fn();
  });

  afterEach(() => {
    globalThis.fetch = originalFetch;
  });

  describe("fetchMarkets", () => {
    it("makes correct API call with pagination params", async () => {
      const mockResponse = {
        markets: [{ id: "1", question: "Test market?" }],
        total: 1,
      };

      vi.mocked(fetch).mockResolvedValue({
        ok: true,
        json: () => Promise.resolve(mockResponse),
      } as Response);

      const result = await fetchMarkets(10, 5);

      expect(fetch).toHaveBeenCalledWith(
        "http://127.0.0.1:3000/markets?limit=10&offset=5"
      );
      expect(result).toEqual(mockResponse);
    });

    it("throws on non-ok response", async () => {
      vi.mocked(fetch).mockResolvedValue({
        ok: false,
        statusText: "Internal Server Error",
      } as Response);

      await expect(fetchMarkets()).rejects.toThrow(
        "Failed to fetch markets: Internal Server Error"
      );
    });
  });

  describe("fetchMarket", () => {
    it("fetches single market by hash", async () => {
      const mockMarket = {
        market_hash: "0xabc123",
        question: "Will it rain?",
      };

      vi.mocked(fetch).mockResolvedValue({
        ok: true,
        json: () => Promise.resolve(mockMarket),
      } as Response);

      const result = await fetchMarket("0xabc123");

      expect(fetch).toHaveBeenCalledWith(
        "http://127.0.0.1:3000/markets/0xabc123"
      );
      expect(result).toEqual(mockMarket);
    });

    it('throws "Market not found" on 404', async () => {
      vi.mocked(fetch).mockResolvedValue({
        ok: false,
        status: 404,
        statusText: "Not Found",
      } as Response);

      await expect(fetchMarket("0xnonexistent")).rejects.toThrow(
        "Market not found"
      );
    });
  });
});
