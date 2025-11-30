import { http, createConfig, createStorage } from "wagmi";
import { tempoTestnet } from "tempo.ts/chains";
import { webAuthn } from "tempo.ts/wagmi";

// USD token address on Tempo (native stablecoin)
const USD_TOKEN = "0x20c0000000000000000000000000000000000001" as const;

// Create Tempo chain with fee token configured
const tempo = tempoTestnet({ feeToken: USD_TOKEN });

export const wagmiConfig = createConfig({
  chains: [tempo],
  connectors: [webAuthn()],
  storage: createStorage({ storage: localStorage }),
  transports: {
    [tempo.id]: http(),
  },
});

// Export the fee token for use in other components
export const TEMPO_FEE_TOKEN = USD_TOKEN;
