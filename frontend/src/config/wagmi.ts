import { createConfig, createStorage, http } from "wagmi";
import { tempoTestnet } from "wagmi/chains";
import { webAuthn, KeyManager } from "wagmi/tempo";
import { getDeployment } from "./deployments";

// RPC URL - defaults to local devnet, can be overridden via env or CLI
const RPC_URL = import.meta.env.VITE_RPC_URL || "http://localhost:9545";

// Fee payer URL - defaults to local, uses testnet sponsor when on testnet
const isTestnet = RPC_URL.includes("moderato.tempo.xyz");
const FEE_PAYER_URL =
  import.meta.env.VITE_FEE_PAYER_URL ||
  (isTestnet ? "https://sponsor.moderato.tempo.xyz" : undefined);

// Determine chain ID based on RPC URL
export const CHAIN_ID = isTestnet ? 42431 : 1337;

// Load deployment addresses for this chain
export const deployment = getDeployment(CHAIN_ID);

// Token addresses on Tempo
// AlphaUSD - the stablecoin users trade with
export const USD_TOKEN =
  "0x20c0000000000000000000000000000000000001" as const;

// PATH_USD - the root quote token (validators accept this for fees)
export const PATH_USD =
  "0x20c0000000000000000000000000000000000000" as const;

// Determine if we're connecting to local devnet
const isLocalDevnet =
  RPC_URL.includes("localhost") || RPC_URL.includes("127.0.0.1");

// Fee token to use - on local devnet, use PATH_USD directly (no Fee AMM liquidity)
// On testnet, use AlphaUSD (Fee AMM converts it)
export const FEE_TOKEN = isLocalDevnet ? PATH_USD : USD_TOKEN;

// Create local devnet chain by extending tempoTestnet to inherit all Tempo formatters/serializers
// This is critical for Tempo-specific features like `calls` batching and `feeToken`
const localDevnet = {
  ...tempoTestnet,
  id: 1337,
  name: "Tempo Local",
  rpcUrls: {
    default: { http: [RPC_URL] },
  },
} as const;

const testnetWithFeeToken = {
  ...tempoTestnet,
  id: 42431, // Moderato chain ID (tempoTestnet defaults to Andantino 42429)
  name: "Tempo Testnet (Moderato)",
  rpcUrls: {
    default: { http: [RPC_URL] },
  },
} as const;

// Pick the right chain based on RPC URL
export const tempoChain = isLocalDevnet ? localDevnet : testnetWithFeeToken;

// Access key configuration - enables signing without passkey prompts
// Expires in 24 hours, no spending limits
const accessKeyConfig = {
  expiry: Math.floor(Date.now() / 1000) + 24 * 60 * 60, // 24 hours from now
  limits: null, // No spending limits
  strict: false, // Don't disconnect if access key is expired, just re-prompt
};

// Wagmi config with native Tempo support via webAuthn connector
export const wagmiConfig = isLocalDevnet
  ? createConfig({
      chains: [localDevnet],
      connectors: [
        webAuthn({
          keyManager: KeyManager.localStorage(),
          grantAccessKey: accessKeyConfig,
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
          grantAccessKey: accessKeyConfig,
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
