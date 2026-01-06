import { createConfig, createStorage, http } from "wagmi";
import { tempoTestnet } from "wagmi/chains";
import { webAuthn, KeyManager } from "wagmi/tempo";

// USD token address on Tempo (native stablecoin)
export const USD_TOKEN =
  "0x20c0000000000000000000000000000000000001" as const;

// Create Tempo chain with fee token configured
export const tempoChain = {
  ...tempoTestnet,
  feeToken: USD_TOKEN,
};

// Fee payer service URL for gasless transactions
const FEE_PAYER_URL = "https://sponsor.testnet.tempo.xyz";

// Wagmi config with native Tempo support via webAuthn connector
export const wagmiConfig = createConfig({
  chains: [tempoChain],
  connectors: [
    webAuthn({
      keyManager: KeyManager.localStorage(),
    }),
  ],
  storage: createStorage({ storage: localStorage }),
  multiInjectedProviderDiscovery: false,
  transports: {
    [tempoChain.id]: http(),
  },
});

// Export fee payer URL for transaction sponsorship
export const TEMPO_FEE_PAYER_URL = FEE_PAYER_URL;
