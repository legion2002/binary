import { createContext, useContext, useMemo, type ReactNode } from "react";
import { createWalletClient, createPublicClient, http, type Address } from "viem";
import { tempoTestnet } from "viem/chains";
import { tempoActions, withFeePayer } from "viem/tempo";
import { usePasskeyAccount } from "../hooks/usePasskeyAccount";

// Fee payer service URL for gasless transactions
const FEE_PAYER_URL = "https://sponsor.testnet.tempo.xyz";

// USD token address on Tempo (native stablecoin)
export const USD_TOKEN = "0x20c0000000000000000000000000000000000001" as const;

// Tempo chain with fee token configured
const tempoChain = {
  ...tempoTestnet,
  feeToken: USD_TOKEN,
} as const;

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type TempoClient = any;

interface PasskeyContextValue {
  // Account state
  account: ReturnType<typeof usePasskeyAccount>["account"];
  address: Address | undefined;
  isConnected: boolean;

  // Clients
  walletClient: TempoClient | null;
  publicClient: TempoClient;

  // Actions
  signUp: () => Promise<unknown>;
  signIn: () => Promise<unknown>;
  signOut: () => void;
  clearCredential: () => void;
  hasSavedCredential: () => boolean;

  // Loading/Error state
  isLoading: boolean;
  error: Error | null;
}

const PasskeyContext = createContext<PasskeyContextValue | null>(null);

interface PasskeyProviderProps {
  children: ReactNode;
  feePayerUrl?: string;
}

export function PasskeyProvider({
  children,
  feePayerUrl = FEE_PAYER_URL,
}: PasskeyProviderProps) {
  const {
    account,
    address,
    isConnected,
    signUp,
    signIn,
    signOut,
    clearCredential,
    hasSavedCredential,
    isLoading,
    error,
  } = usePasskeyAccount();

  // Create wallet client with fee sponsorship when account is available
  const walletClient = useMemo(() => {
    if (!account) return null;

    const client = createWalletClient({
      account,
      chain: tempoChain,
      // withFeePayer takes positional args: (defaultTransport, relayTransport, options?)
      transport: withFeePayer(http(), http(feePayerUrl)),
    });

    return client.extend(tempoActions());
  }, [account, feePayerUrl]);

  // Public client for read operations (always available)
  const publicClient = useMemo(() => {
    const client = createPublicClient({
      chain: tempoChain,
      transport: http(),
    });
    return client.extend(tempoActions());
  }, []);

  const value: PasskeyContextValue = {
    account,
    address,
    isConnected,
    walletClient,
    publicClient,
    signUp,
    signIn,
    signOut,
    clearCredential,
    hasSavedCredential,
    isLoading,
    error,
  };

  return (
    <PasskeyContext.Provider value={value}>{children}</PasskeyContext.Provider>
  );
}

export function usePasskey() {
  const context = useContext(PasskeyContext);
  if (!context) {
    throw new Error("usePasskey must be used within a PasskeyProvider");
  }
  return context;
}

// Convenience hook for just the wallet client
export function useTempoWalletClient(): TempoClient | null {
  const { walletClient } = usePasskey();
  return walletClient;
}

// Convenience hook for just the public client
export function useTempoPublicClient(): TempoClient {
  const { publicClient } = usePasskey();
  return publicClient;
}
