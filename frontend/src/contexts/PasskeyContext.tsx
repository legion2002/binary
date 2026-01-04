import {
  createContext,
  useContext,
  useMemo,
  type ReactNode,
} from "react";
import {
  createWalletClient,
  createPublicClient,
  http,
  type WalletClient,
  type PublicClient,
  type Address,
} from "viem";
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
};

type TempoWalletClient = WalletClient & ReturnType<typeof tempoActions>;
type TempoPublicClient = PublicClient & ReturnType<typeof tempoActions>;

interface PasskeyContextValue {
  // Account state
  account: ReturnType<typeof usePasskeyAccount>["account"];
  address: Address | undefined;
  isConnected: boolean;

  // Clients
  walletClient: TempoWalletClient | null;
  publicClient: TempoPublicClient;

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

    return createWalletClient({
      account,
      chain: tempoChain,
      transport: withFeePayer({
        defaultTransport: http(),
        relayTransport: http(feePayerUrl),
      }),
    }).extend(tempoActions()) as TempoWalletClient;
  }, [account, feePayerUrl]);

  // Public client for read operations (always available)
  const publicClient = useMemo(
    () =>
      createPublicClient({
        chain: tempoChain,
        transport: http(),
      }).extend(tempoActions()) as TempoPublicClient,
    []
  );

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
export function useTempoWalletClient() {
  const { walletClient } = usePasskey();
  return walletClient;
}

// Convenience hook for just the public client
export function useTempoPublicClient() {
  const { publicClient } = usePasskey();
  return publicClient;
}
