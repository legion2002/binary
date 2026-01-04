import { http, createConfig, createStorage } from "wagmi";
import { tempoTestnet } from "viem/chains";
import { webAuthn, KeyManager } from "tempo.ts/wagmi"; // WebAuthn connector not yet in native wagmi
import type { Chain } from "viem";

// USD token address on Tempo (native stablecoin)
const USD_TOKEN = "0x20c0000000000000000000000000000000000001" as const;

// Create Tempo chain with fee token configured
const tempo = {
  ...tempoTestnet,
  feeToken: USD_TOKEN,
} as Chain & { feeToken: typeof USD_TOKEN };

export const wagmiConfig = createConfig({
  chains: [tempo],
  connectors: [
    webAuthn({
      keyManager: KeyManager.localStorage(),
    }),
  ],
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
