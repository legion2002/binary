import { http, createConfig, createStorage } from "wagmi";
import { tempoTestnet } from "viem/chains";

// USD token address on Tempo (native stablecoin)
const USD_TOKEN = "0x20c0000000000000000000000000000000000001" as const;

// Create Tempo chain with fee token configured
const tempo = {
  ...tempoTestnet,
  feeToken: USD_TOKEN,
};

// Simplified wagmi config - passkey handling is now in PasskeyContext
// This config is kept for compatibility with existing wagmi hooks if needed
export const wagmiConfig = createConfig({
  chains: [tempo],
  connectors: [], // No connectors - using direct passkey via PasskeyContext
  storage: createStorage({ storage: localStorage }),
  multiInjectedProviderDiscovery: false,
  transports: {
    [tempo.id]: http(),
  },
});

// Export the chain for use with viem clients
export const tempoChain = tempo;

// Export the fee token for use in other components
export const TEMPO_FEE_TOKEN = USD_TOKEN;
