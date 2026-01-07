import { MULTIVERSE_ABI, TIP20_ABI } from "./abis";

// Re-export ABIs
export { MULTIVERSE_ABI, TIP20_ABI };

// Contract addresses for Tempo Testnet (Andantino)
export const CONTRACTS = {
  // Binary Protocol Contracts (deployed)
  MULTIVERSE: "0xA51c1fc2f0D1a1b8494Ed1FE312d7C3a78Ed91C0",
  ORACLE: "0xB7f8BC63BbcaD18155201308C8f3540b07f84F5e",
  
  // Tempo System Precompiles
  PATH_USD: "0x20C0000000000000000000000000000000000000",  // pathUSD (tokenId 0)
  ALPHA_USD: "0x20C0000000000000000000000000000000000001", // alphaUSD (tokenId 1)
  BETA_USD: "0x20C0000000000000000000000000000000000002",  // betaUSD (tokenId 2)
  THETA_USD: "0x20C0000000000000000000000000000000000003", // thetaUSD (tokenId 3)
  TIP20_FACTORY: "0x20Fc000000000000000000000000000000000000",
  FEE_MANAGER: "0xFeEc000000000000000000000000000000000000",
  STABLECOIN_DEX: "0xDec0000000000000000000000000000000000000",
  
  // Stablecoin aliases (USD = alphaUSD for trading)
  USD: "0x20C0000000000000000000000000000000000001",   // alphaUSD - primary trading currency
  USDC: "0x20C0000000000000000000000000000000000002",  // betaUSD - alternate stablecoin
} as const;

// Market data is now fetched from the backend API
// See src/api/client.ts and src/api/types.ts
