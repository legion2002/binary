import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "../test/test-utils";

const mockUseQuery = vi.fn();

vi.mock("@tanstack/react-query", async () => {
  const actual = await vi.importActual("@tanstack/react-query");
  return {
    ...actual,
    useQuery: (options: unknown) => mockUseQuery(options),
  };
});

vi.mock("../api/client", () => ({
  fetchMarkets: vi.fn(),
  fetchMarket: vi.fn(),
}));

vi.mock(new URL("./MarketCard.tsx", import.meta.url).pathname, async () => {
  const React = await import("react");
  return {
    MarketCard: ({ market }: { market: { question: string } }) =>
      React.createElement("div", { "data-testid": "market-card" }, market.question),
  };
});

import { MarketList } from "./MarketList";

describe("MarketList", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("shows loading spinner when loading", () => {
    mockUseQuery.mockReturnValue({
      data: undefined,
      isLoading: true,
      error: null,
    });

    render(<MarketList />);
    expect(screen.getByText("Loading markets...")).toBeInTheDocument();
    expect(document.querySelector(".spinner")).toBeInTheDocument();
  });

  it("shows empty state when no markets", () => {
    mockUseQuery.mockReturnValue({
      data: { markets: [], count: 0 },
      isLoading: false,
      error: null,
    });

    render(<MarketList />);
    expect(screen.getByText("No markets available")).toBeInTheDocument();
  });

  it("shows error message when fetch fails", () => {
    mockUseQuery.mockReturnValue({
      data: undefined,
      isLoading: false,
      error: new Error("Network error"),
    });

    render(<MarketList />);
    expect(screen.getByText("Failed to load markets")).toBeInTheDocument();
    expect(screen.getByText("Network error")).toBeInTheDocument();
  });

  it("renders market cards when data available", () => {
    mockUseQuery.mockReturnValue({
      data: {
        markets: [
          { marketHash: "0x123", question: "Will ETH reach $5000?", resolutionDeadline: 1767225600 },
          { marketHash: "0x456", question: "Will BTC reach $100k?", resolutionDeadline: 1767225600 },
        ],
        count: 2,
      },
      isLoading: false,
      error: null,
    });

    render(<MarketList />);
    expect(screen.getByText("Markets (2)")).toBeInTheDocument();
    expect(screen.getAllByTestId("market-card")).toHaveLength(2);
    expect(screen.getByText("Will ETH reach $5000?")).toBeInTheDocument();
    expect(screen.getByText("Will BTC reach $100k?")).toBeInTheDocument();
  });
});
