import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "../test/test-utils";
import { FundButton } from "./FundButton";

const mockFund = vi.fn();

vi.mock("wagmi", async () => {
  const actual = await vi.importActual("wagmi");
  return {
    ...actual,
    useAccount: vi.fn(() => ({
      address: "0x1234567890abcdef1234567890abcdef12345678",
      isConnected: true,
    })),
  };
});

vi.mock("../hooks/useStablecoinBalances", () => ({
  useStablecoinBalances: () => ({
    balances: [
      { address: "0x20C0000000000000000000000000000000000001", symbol: "αUSD", name: "Alpha USD", balance: 1000000n, formattedBalance: "1.00" },
      { address: "0x20C0000000000000000000000000000000000000", symbol: "pUSD", name: "Path USD", balance: 2000000n, formattedBalance: "2.00" },
    ],
    isLoading: false,
    refetch: vi.fn(),
  }),
  STABLECOINS: [
    { address: "0x20C0000000000000000000000000000000000001", symbol: "αUSD", name: "Alpha USD" },
    { address: "0x20C0000000000000000000000000000000000000", symbol: "pUSD", name: "Path USD" },
  ],
}));

vi.mock("../hooks/useFaucet", () => ({
  useFaucet: () => ({
    fund: mockFund,
    isPending: false,
    error: null,
  }),
}));

import { useAccount } from "wagmi";

describe("FundButton", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders Fund button when connected", () => {
    render(<FundButton />);
    expect(screen.getByTestId("fund-button")).toBeInTheDocument();
    expect(screen.getByText("Fund")).toBeInTheDocument();
  });

  it("does not render when not connected", () => {
    vi.mocked(useAccount).mockReturnValue({
      address: undefined,
      isConnected: false,
    } as ReturnType<typeof useAccount>);

    const { container } = render(<FundButton />);
    expect(container).toBeEmptyDOMElement();
  });

  it("opens dropdown when clicked", () => {
    vi.mocked(useAccount).mockReturnValue({
      address: "0x1234567890abcdef1234567890abcdef12345678",
      isConnected: true,
    } as unknown as ReturnType<typeof useAccount>);

    render(<FundButton />);
    fireEvent.click(screen.getByTestId("fund-button"));
    expect(screen.getByTestId("fund-dropdown")).toBeInTheDocument();
  });

  it("shows stablecoin options in dropdown", () => {
    vi.mocked(useAccount).mockReturnValue({
      address: "0x1234567890abcdef1234567890abcdef12345678",
      isConnected: true,
    } as unknown as ReturnType<typeof useAccount>);

    render(<FundButton />);
    fireEvent.click(screen.getByTestId("fund-button"));
    expect(screen.getByTestId("fund-token-αUSD")).toBeInTheDocument();
    expect(screen.getByTestId("fund-token-pUSD")).toBeInTheDocument();
  });

  it("shows amount input and presets", () => {
    vi.mocked(useAccount).mockReturnValue({
      address: "0x1234567890abcdef1234567890abcdef12345678",
      isConnected: true,
    } as unknown as ReturnType<typeof useAccount>);

    render(<FundButton />);
    fireEvent.click(screen.getByTestId("fund-button"));
    expect(screen.getByTestId("fund-amount-input")).toBeInTheDocument();
    expect(screen.getByText("$100")).toBeInTheDocument();
    expect(screen.getByText("$500")).toBeInTheDocument();
    expect(screen.getByText("$1000")).toBeInTheDocument();
    expect(screen.getByText("$5000")).toBeInTheDocument();
  });

  it("calls fund with token and amount when button clicked", async () => {
    vi.mocked(useAccount).mockReturnValue({
      address: "0x1234567890abcdef1234567890abcdef12345678",
      isConnected: true,
    } as unknown as ReturnType<typeof useAccount>);

    render(<FundButton />);
    fireEvent.click(screen.getByTestId("fund-button"));
    fireEvent.click(screen.getByTestId("fund-action-button"));
    
    expect(mockFund).toHaveBeenCalledWith({
      token: "0x20C0000000000000000000000000000000000001",
      amount: 1000000000n, // 1000 * 10^6
    });
  });

  it("allows selecting a different token", async () => {
    vi.mocked(useAccount).mockReturnValue({
      address: "0x1234567890abcdef1234567890abcdef12345678",
      isConnected: true,
    } as unknown as ReturnType<typeof useAccount>);

    render(<FundButton />);
    fireEvent.click(screen.getByTestId("fund-button"));
    fireEvent.click(screen.getByTestId("fund-token-pUSD"));
    fireEvent.click(screen.getByTestId("fund-action-button"));
    
    expect(mockFund).toHaveBeenCalledWith({
      token: "0x20C0000000000000000000000000000000000000",
      amount: 1000000000n,
    });
  });

  it("allows changing amount with preset buttons", async () => {
    vi.mocked(useAccount).mockReturnValue({
      address: "0x1234567890abcdef1234567890abcdef12345678",
      isConnected: true,
    } as unknown as ReturnType<typeof useAccount>);

    render(<FundButton />);
    fireEvent.click(screen.getByTestId("fund-button"));
    fireEvent.click(screen.getByText("$500"));
    fireEvent.click(screen.getByTestId("fund-action-button"));
    
    expect(mockFund).toHaveBeenCalledWith({
      token: "0x20C0000000000000000000000000000000000001",
      amount: 500000000n, // 500 * 10^6
    });
  });
});
