import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "../test/test-utils";
import { TradePanel } from "./TradePanel";
import type { Address } from "viem";

// Static precompile address for tests
const ALPHA_USD = "0x20C0000000000000000000000000000000000001" as Address;

const mockBuy = vi.fn();
const mockWriteContract = vi.fn();

vi.mock("wagmi", async () => {
  const actual = await vi.importActual("wagmi");
  return {
    ...actual,
    useAccount: vi.fn(() => ({
      isConnected: false,
    })),
  };
});

vi.mock("../hooks/useTempoDex", () => ({
  useBuyQuote: vi.fn(() => ({ data: undefined, isLoading: false })),
  useBuySync: vi.fn(() => ({ mutate: mockBuy, isPending: false })),
  useSellQuote: vi.fn(() => ({ data: undefined, isLoading: false })),
  useSellSync: vi.fn(() => ({ mutate: vi.fn(), isPending: false })),
  usePlaceOrderSync: vi.fn(() => ({ mutate: vi.fn(), isPending: false })),
}));

vi.mock("../hooks/useContractWrite", () => ({
  useWriteContract: vi.fn(() => ({
    writeContract: mockWriteContract,
    isPending: false,
  })),
}));

vi.mock("../hooks/useContracts", () => ({
  useContracts: vi.fn(() => ({
    contracts: {
      MULTIVERSE: "0xA51c1fc2f0D1a1b8494Ed1FE312d7C3a78Ed91C0",
      ORACLE: "0xB7f8BC63BbcaD18155201308C8f3540b07f84F5e",
      PATH_USD: "0x20C0000000000000000000000000000000000000",
      ALPHA_USD: "0x20C0000000000000000000000000000000000001",
    },
    isLoading: false,
    error: null,
  })),
  MULTIVERSE_ABI: [],
  TIP20_ABI: [],
}));

import { useAccount } from "wagmi";

const defaultProps = {
  marketHash: "0x123",
  yesTokenAddress: "0xyes",
  noTokenAddress: "0xno",
  yesPrice: 0.6,
  noPrice: 0.4,
  selectedAsset: ALPHA_USD,
  selectedBalance: 1000000000n, // 1000 USD (6 decimals)
};

describe("TradePanel", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(useAccount).mockReturnValue({
      isConnected: false,
    } as ReturnType<typeof useAccount>);
  });

  it("renders all three tabs", () => {
    render(<TradePanel {...defaultProps} />);
    expect(screen.getByRole("button", { name: "Trade YES" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Trade NO" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Split" })).toBeInTheDocument();
  });

  it("switching tabs updates the active state", () => {
    render(<TradePanel {...defaultProps} />);

    const tradeYesTab = screen.getByRole("button", { name: "Trade YES" });
    const tradeNoTab = screen.getByRole("button", { name: "Trade NO" });
    const splitTab = screen.getByRole("button", { name: "Split" });

    expect(tradeYesTab).toHaveClass("active");
    expect(tradeNoTab).not.toHaveClass("active");
    expect(splitTab).not.toHaveClass("active");

    fireEvent.click(tradeNoTab);
    expect(tradeYesTab).not.toHaveClass("active");
    expect(tradeNoTab).toHaveClass("active");

    fireEvent.click(splitTab);
    expect(splitTab).toHaveClass("active");
    expect(tradeNoTab).not.toHaveClass("active");
  });

  it("input field accepts numeric values", () => {
    vi.mocked(useAccount).mockReturnValue({
      isConnected: true,
    } as ReturnType<typeof useAccount>);

    render(<TradePanel {...defaultProps} />);
    const input = screen.getByPlaceholderText("0.00");

    fireEvent.change(input, { target: { value: "100" } });
    expect(input).toHaveValue(100);

    fireEvent.change(input, { target: { value: "50.5" } });
    expect(input).toHaveValue(50.5);
  });

  it("renders buy and sell buttons for trade tabs", () => {
    vi.mocked(useAccount).mockReturnValue({
      isConnected: true,
    } as ReturnType<typeof useAccount>);

    render(<TradePanel {...defaultProps} />);
    expect(screen.getByRole("button", { name: "Buy" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Sell" })).toBeInTheDocument();
  });

  it("buy button disabled when amount is empty or zero", () => {
    vi.mocked(useAccount).mockReturnValue({
      isConnected: true,
    } as ReturnType<typeof useAccount>);

    render(<TradePanel {...defaultProps} />);
    const buyButton = screen.getByRole("button", { name: "Buy" });
    expect(buyButton).toBeDisabled();

    const input = screen.getByPlaceholderText("0.00");
    fireEvent.change(input, { target: { value: "0" } });
    expect(buyButton).toBeDisabled();
  });

  it("Split tab shows split preview when amount entered", () => {
    vi.mocked(useAccount).mockReturnValue({
      isConnected: true,
    } as ReturnType<typeof useAccount>);

    render(<TradePanel {...defaultProps} />);

    fireEvent.click(screen.getByRole("button", { name: "Split" }));

    const input = screen.getByPlaceholderText("0.00");
    fireEvent.change(input, { target: { value: "100" } });

    expect(screen.getByText("You deposit")).toBeInTheDocument();
    expect(screen.getByText("$100 USD")).toBeInTheDocument();
    expect(screen.getByText("You receive")).toBeInTheDocument();
    expect(screen.getByText("100 YES + 100 NO")).toBeInTheDocument();
  });

  it("shows order type toggle for trade tabs", () => {
    vi.mocked(useAccount).mockReturnValue({
      isConnected: true,
    } as ReturnType<typeof useAccount>);

    render(<TradePanel {...defaultProps} />);

    expect(screen.getByTestId("order-type-toggle")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Market" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Limit" })).toBeInTheDocument();
  });

  it("shows price input when limit order is selected", () => {
    vi.mocked(useAccount).mockReturnValue({
      isConnected: true,
    } as ReturnType<typeof useAccount>);

    render(<TradePanel {...defaultProps} />);

    fireEvent.click(screen.getByRole("button", { name: "Limit" }));

    expect(screen.getByTestId("limit-price-input")).toBeInTheDocument();
  });

  it("hides order type toggle on split tab", () => {
    vi.mocked(useAccount).mockReturnValue({
      isConnected: true,
    } as ReturnType<typeof useAccount>);

    render(<TradePanel {...defaultProps} />);

    fireEvent.click(screen.getByRole("button", { name: "Split" }));

    expect(screen.queryByTestId("order-type-toggle")).not.toBeInTheDocument();
  });

  it("defaults to market order type", () => {
    vi.mocked(useAccount).mockReturnValue({
      isConnected: true,
    } as ReturnType<typeof useAccount>);

    render(<TradePanel {...defaultProps} />);

    const marketBtn = screen.getByRole("button", { name: "Market" });
    const limitBtn = screen.getByRole("button", { name: "Limit" });

    expect(marketBtn).toHaveClass("active");
    expect(limitBtn).not.toHaveClass("active");
  });

  it("switches between market and limit order types", () => {
    vi.mocked(useAccount).mockReturnValue({
      isConnected: true,
    } as ReturnType<typeof useAccount>);

    render(<TradePanel {...defaultProps} />);

    const marketBtn = screen.getByRole("button", { name: "Market" });
    const limitBtn = screen.getByRole("button", { name: "Limit" });

    fireEvent.click(limitBtn);
    expect(limitBtn).toHaveClass("active");
    expect(marketBtn).not.toHaveClass("active");

    fireEvent.click(marketBtn);
    expect(marketBtn).toHaveClass("active");
    expect(limitBtn).not.toHaveClass("active");
  });

  it("hides price input for market orders", () => {
    vi.mocked(useAccount).mockReturnValue({
      isConnected: true,
    } as ReturnType<typeof useAccount>);

    render(<TradePanel {...defaultProps} />);

    expect(screen.queryByTestId("limit-price-input")).not.toBeInTheDocument();
  });

  it("shows limit order quote when amount and price entered", () => {
    vi.mocked(useAccount).mockReturnValue({
      isConnected: true,
    } as ReturnType<typeof useAccount>);

    render(<TradePanel {...defaultProps} />);

    fireEvent.click(screen.getByRole("button", { name: "Limit" }));

    const inputs = screen.getAllByPlaceholderText("0.00");
    fireEvent.change(inputs[0], { target: { value: "100" } });

    const priceInput = screen.getByTestId("limit-price-input");
    fireEvent.change(priceInput, { target: { value: "0.50" } });

    expect(screen.getByText("Total cost")).toBeInTheDocument();
    expect(screen.getByText("$50.00")).toBeInTheDocument();
    expect(screen.getByText("Limit price")).toBeInTheDocument();
    expect(screen.getByText("$0.50")).toBeInTheDocument();
  });

  it("buy button disabled for limit order without price", () => {
    vi.mocked(useAccount).mockReturnValue({
      isConnected: true,
    } as ReturnType<typeof useAccount>);

    render(<TradePanel {...defaultProps} />);

    fireEvent.click(screen.getByRole("button", { name: "Limit" }));

    const inputs = screen.getAllByPlaceholderText("0.00");
    fireEvent.change(inputs[0], { target: { value: "100" } });

    const buyButton = screen.getByRole("button", { name: "Buy" });
    expect(buyButton).toBeDisabled();
  });

  it("buy button enabled for limit order with amount and price", () => {
    vi.mocked(useAccount).mockReturnValue({
      isConnected: true,
    } as ReturnType<typeof useAccount>);

    render(<TradePanel {...defaultProps} />);

    fireEvent.click(screen.getByRole("button", { name: "Limit" }));

    const inputs = screen.getAllByPlaceholderText("0.00");
    fireEvent.change(inputs[0], { target: { value: "100" } });

    const priceInput = screen.getByTestId("limit-price-input");
    fireEvent.change(priceInput, { target: { value: "0.50" } });

    const buyButton = screen.getByRole("button", { name: "Buy" });
    expect(buyButton).not.toBeDisabled();
  });
});
