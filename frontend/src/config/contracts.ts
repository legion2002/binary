import { MULTIVERSE_ABI, ERC20_ABI } from "./abis";

// Re-export ABIs for backward compatibility
export { MULTIVERSE_ABI, ERC20_ABI };

// Contract addresses for Tempo Testnet (Andantino)
// Tempo uses USD as native currency with 6 decimals
export const CONTRACTS = {
  MULTIVERSE: "0x0000000000000000000000000000000000000000", // TODO: Replace with deployed address on Tempo
  // Tempo native stablecoins - addresses from Tempo precompiles
  USD: "0x20c0000000000000000000000000000000000001", // Native USD on Tempo
  USDC: "0x20c0000000000000000000000000000000000002", // USDC on Tempo (if available)
  USDT: "0x20c0000000000000000000000000000000000003", // USDT on Tempo (if available)
  // Tempo DEX is built-in, no external router/factory needed
} as const;

// Market data is now fetched from the backend API
// See src/api/client.ts and src/api/types.ts
