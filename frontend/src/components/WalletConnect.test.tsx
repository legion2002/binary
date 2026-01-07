import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "../test/test-utils";
import { WalletConnect } from "./WalletConnect";

const mockConnect = vi.fn();
const mockDisconnect = vi.fn();

vi.mock("wagmi", async () => {
  const actual = await vi.importActual("wagmi");
  return {
    ...actual,
    useAccount: vi.fn(() => ({
      address: undefined,
      isConnected: false,
    })),
    useConnect: vi.fn(() => ({
      connect: mockConnect,
      connectors: [{ id: "webAuthn" }],
      isPending: false,
    })),
    useDisconnect: vi.fn(() => ({
      disconnect: mockDisconnect,
    })),
  };
});

import { useAccount, useConnect } from "wagmi";

describe("WalletConnect", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders Log In and Sign Up buttons when not connected", () => {
    render(<WalletConnect />);
    expect(screen.getByRole("button", { name: "Log In" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Sign Up" })).toBeInTheDocument();
  });

  it("Log In button calls connect with sign-in capability", () => {
    render(<WalletConnect />);
    const loginButton = screen.getByRole("button", { name: "Log In" });
    fireEvent.click(loginButton);
    expect(mockConnect).toHaveBeenCalledWith(
      expect.objectContaining({
        connector: { id: "webAuthn" },
        capabilities: expect.objectContaining({ type: "sign-in" }),
      }),
      expect.objectContaining({ onError: expect.any(Function) })
    );
  });

  it("Sign Up button calls connect with sign-up capability", () => {
    render(<WalletConnect />);
    const signUpButton = screen.getByRole("button", { name: "Sign Up" });
    fireEvent.click(signUpButton);
    expect(mockConnect).toHaveBeenCalledWith(
      expect.objectContaining({
        connector: { id: "webAuthn" },
        capabilities: expect.objectContaining({ type: "sign-up" }),
      }),
      expect.objectContaining({ onError: expect.any(Function) })
    );
  });

  it("shows ... and disables buttons when pending", () => {
    vi.mocked(useAccount).mockReturnValue({
      address: undefined,
      isConnected: false,
    } as ReturnType<typeof useAccount>);
    vi.mocked(useConnect).mockReturnValue({
      connect: mockConnect,
      connectors: [{ id: "webAuthn" }],
      isPending: true,
    } as unknown as ReturnType<typeof useConnect>);

    render(<WalletConnect />);
    const buttons = screen.getAllByRole("button");
    expect(buttons).toHaveLength(2);
    buttons.forEach((button) => {
      expect(button).toBeDisabled();
      expect(button).toHaveTextContent("...");
    });
  });

  it("shows address and Sign Out button when connected", () => {
    vi.mocked(useAccount).mockReturnValue({
      address: "0x1234567890abcdef1234567890abcdef12345678",
      isConnected: true,
    } as unknown as ReturnType<typeof useAccount>);
    vi.mocked(useConnect).mockReturnValue({
      connect: mockConnect,
      connectors: [{ id: "webAuthn" }],
      isPending: false,
    } as unknown as ReturnType<typeof useConnect>);

    render(<WalletConnect />);
    expect(screen.getByText("0x1234...5678")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Sign Out" })).toBeInTheDocument();
  });

  it("Sign Out button calls disconnect", () => {
    vi.mocked(useAccount).mockReturnValue({
      address: "0x1234567890abcdef1234567890abcdef12345678",
      isConnected: true,
    } as unknown as ReturnType<typeof useAccount>);
    vi.mocked(useConnect).mockReturnValue({
      connect: mockConnect,
      connectors: [{ id: "webAuthn" }],
      isPending: false,
    } as unknown as ReturnType<typeof useConnect>);

    render(<WalletConnect />);
    const signOutButton = screen.getByRole("button", { name: "Sign Out" });
    fireEvent.click(signOutButton);
    expect(mockDisconnect).toHaveBeenCalled();
  });
});
