import { createConfig, createStorage, http } from "wagmi";
import { tempoTestnet } from "wagmi/chains";
import { webAuthn, KeyManager } from "wagmi/tempo";
import type { Chain } from "viem";

// RPC URL - defaults to local devnet, can be overridden via env or CLI
const RPC_URL = import.meta.env.VITE_RPC_URL || "http://localhost:9545";

// Fee payer URL - defaults to local, uses testnet sponsor when on testnet
const isTestnet = RPC_URL.includes("testnet.tempo.xyz");
const FEE_PAYER_URL =
  import.meta.env.VITE_FEE_PAYER_URL ||
  (isTestnet ? "https://sponsor.testnet.tempo.xyz" : undefined);

// USD token address on Tempo (native stablecoin) - AlphaUSD
export const USD_TOKEN =
  "0x20c0000000000000000000000000000000000001" as const;

// PATH_USD token address - used as fee token on local devnet
const PATH_USD = "0x20C0000000000000000000000000000000000000" as const;

// Determine if we're connecting to local devnet
const isLocalDevnet =
  RPC_URL.includes("localhost") || RPC_URL.includes("127.0.0.1");

// Create the chain config for local devnet with Tempo properties
const localDevnet = {
  id: 1337,
  name: "Tempo Local",
  nativeCurrency: tempoTestnet.nativeCurrency,
  rpcUrls: {
    default: { http: [RPC_URL] },
  },
  blockExplorers: tempoTestnet.blockExplorers,
  // Tempo-specific: default fee token for transactions
  feeToken: PATH_USD,
} as const satisfies Chain & { feeToken: `0x${string}` };

const testnetWithFeeToken = {
  ...tempoTestnet,
  rpcUrls: {
    default: { http: [RPC_URL] },
  },
} as const;

// Pick the right chain based on RPC URL
export const tempoChain = isLocalDevnet ? localDevnet : testnetWithFeeToken;

// Wagmi config with native Tempo support via webAuthn connector
export const wagmiConfig = isLocalDevnet
  ? createConfig({
      chains: [localDevnet],
      connectors: [
        webAuthn({
          keyManager: KeyManager.localStorage(),
        }),
      ],
      storage: createStorage({ storage: localStorage }),
      multiInjectedProviderDiscovery: false,
      transports: {
        [localDevnet.id]: http(RPC_URL),
      },
    })
  : createConfig({
      chains: [testnetWithFeeToken],
      connectors: [
        webAuthn({
          keyManager: KeyManager.localStorage(),
        }),
      ],
      storage: createStorage({ storage: localStorage }),
      multiInjectedProviderDiscovery: false,
      transports: {
        [testnetWithFeeToken.id]: http(RPC_URL),
      },
    });

// Export config values for use in other components
export const TEMPO_RPC_URL = RPC_URL;
export const TEMPO_FEE_PAYER_URL = FEE_PAYER_URL;
