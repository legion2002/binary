import { MULTIVERSE_ABI, UNISWAP_V2_PAIR_ABI, ERC20_ABI } from "./abis";

// Re-export ABIs for backward compatibility
export { MULTIVERSE_ABI, UNISWAP_V2_PAIR_ABI, ERC20_ABI };

// Contract addresses for Base Mainnet
export const CONTRACTS = {
  MULTIVERSE: "0x0000000000000000000000000000000000000000", // TODO: Replace with deployed address
  WETH: "0x4200000000000000000000000000000000000006", // WETH on Base
  USDC: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913", // USDC on Base
  UNISWAP_V2_FACTORY: "0x8909Dc15e40173Ff4699343b6eB8132c65e18eC6", // Uniswap V2 Factory on Base
  UNISWAP_V2_ROUTER: "0x4752ba5DBc23f44D87826276BF6Fd6b1C372aD24", // Uniswap V2 Router02 on Base
} as const;

// Mock markets for development - replace with actual market data
export const MOCK_MARKETS = [
  {
    questionHash:
      "0x1234567890123456789012345678901234567890123456789012345678901234",
    marketHash:
      "0xabcdef1234567890123456789012345678901234567890123456789012345678",
    question: "Will FOCIL be included in the Ethereum Glamsterdam hardfork?",
    resolutionDeadline: 1798761600, // Jan 1, 2027
    oracle: "0x0000000000000000000000000000000000000000",
  },
  {
    questionHash:
      "0x2345678901234567890123456789012345678901234567890123456789012345",
    marketHash:
      "0xbcdef12345678901234567890123456789012345678901234567890123456789",
    question: "Will ePBS be included in the Ethereum Glamsterdam hardfork?",
    resolutionDeadline: 1798761600, // Jan 1, 2027
    oracle: "0x0000000000000000000000000000000000000000",
  },
];

export type Market = (typeof MOCK_MARKETS)[0];
