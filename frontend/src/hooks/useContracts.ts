import type { Address } from "viem";
import { useConfig } from "./useConfig";
import { MULTIVERSE_ABI, TIP20_ABI } from "../config/abis";
import { deployment } from "../config/wagmi";

// Static contract addresses (Tempo system precompiles - never change)
const STATIC_CONTRACTS = {
  PATH_USD: "0x20C0000000000000000000000000000000000000" as Address,
  ALPHA_USD: "0x20C0000000000000000000000000000000000001" as Address,
  BETA_USD: "0x20C0000000000000000000000000000000000002" as Address,
  THETA_USD: "0x20C0000000000000000000000000000000000003" as Address,
  TIP20_FACTORY: "0x20Fc000000000000000000000000000000000000" as Address,
  FEE_MANAGER: "0xFeEc000000000000000000000000000000000000" as Address,
  STABLECOIN_DEX: "0xDec0000000000000000000000000000000000000" as Address,
  USD: "0x20C0000000000000000000000000000000000001" as Address,
  USDC: "0x20C0000000000000000000000000000000000002" as Address,
  UNIV2_ROUTER: deployment.uniV2Router,
  UNIV2_FACTORY: deployment.uniV2Factory,
} as const;

export interface Contracts {
  MULTIVERSE: Address;
  ORACLE: Address;
  PATH_USD: Address;
  ALPHA_USD: Address;
  BETA_USD: Address;
  THETA_USD: Address;
  TIP20_FACTORY: Address;
  FEE_MANAGER: Address;
  STABLECOIN_DEX: Address;
  USD: Address;
  USDC: Address;
  UNIV2_ROUTER: Address;
  UNIV2_FACTORY: Address;
}

/**
 * Hook to get contract addresses
 * Dynamic addresses (MULTIVERSE, ORACLE) are fetched from backend
 * Static addresses (stablecoins, precompiles) are hardcoded
 */
export function useContracts(): {
  contracts: Contracts | null;
  isLoading: boolean;
  error: Error | null;
} {
  const { data: config, isLoading, error } = useConfig();

  if (!config) {
    return { contracts: null, isLoading, error: error as Error | null };
  }

  const contracts: Contracts = {
    MULTIVERSE: config.multiverseAddress as Address,
    ORACLE: config.oracleAddress as Address,
    ...STATIC_CONTRACTS,
  };

  return { contracts, isLoading: false, error: null };
}

// Re-export ABIs for convenience
export { MULTIVERSE_ABI, TIP20_ABI };
