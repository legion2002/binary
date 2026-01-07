import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "../test/test-utils";
import { TradePanel } from "./TradePanel";

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
}));

vi.mock("../hooks/useContractWrite", () => ({
  useWriteContract: vi.fn(() => ({
    writeContract: mockWriteContract,
    isPending: false,
  })),
}));

import { useAccount } from "wagmi";

const defaultProps = {
  marketHash: "0x123",
  yesTokenAddress: "0xyes",
  noTokenAddress: "0xno",
  yesPrice: 0.6,
  noPrice: 0.4,
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
    expect(screen.getByRole("button", { name: "Buy YES" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Buy NO" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Split" })).toBeInTheDocument();
  });

  it("switching tabs updates the active state", () => {
    render(<TradePanel {...defaultProps} />);

    const buyYesTab = screen.getByRole("button", { name: "Buy YES" });
    const buyNoTab = screen.getByRole("button", { name: "Buy NO" });
    const splitTab = screen.getByRole("button", { name: "Split" });

    expect(buyYesTab).toHaveClass("active");
    expect(buyNoTab).not.toHaveClass("active");
    expect(splitTab).not.toHaveClass("active");

    fireEvent.click(buyNoTab);
    expect(buyYesTab).not.toHaveClass("active");
    expect(buyNoTab).toHaveClass("active");

    fireEvent.click(splitTab);
    expect(splitTab).toHaveClass("active");
    expect(buyNoTab).not.toHaveClass("active");
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

  it("button disabled when not connected", () => {
    render(<TradePanel {...defaultProps} />);
    const tradeButton = screen.getByRole("button", { name: "Connect Wallet" });
    expect(tradeButton).toBeDisabled();
  });

  it("button disabled when amount is empty or zero", () => {
    vi.mocked(useAccount).mockReturnValue({
      isConnected: true,
    } as ReturnType<typeof useAccount>);

    render(<TradePanel {...defaultProps} />);
    const tradeButton = screen.getByRole("button", { name: "Buy YES Tokens" });
    expect(tradeButton).toBeDisabled();

    const input = screen.getByPlaceholderText("0.00");
    fireEvent.change(input, { target: { value: "0" } });
    expect(tradeButton).toBeDisabled();
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
});
