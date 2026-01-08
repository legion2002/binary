import type { Address } from "viem";
import { useAccount } from "wagmi";
import { Hooks } from "wagmi/tempo";

// Static Tempo stablecoin precompile addresses (never change)
const STABLECOIN_ADDRESSES = {
  ALPHA_USD: "0x20C0000000000000000000000000000000000001" as Address,
  PATH_USD: "0x20C0000000000000000000000000000000000000" as Address,
  BETA_USD: "0x20C0000000000000000000000000000000000002" as Address,
  THETA_USD: "0x20C0000000000000000000000000000000000003" as Address,
} as const;

export interface StablecoinInfo {
  address: Address;
  symbol: string;
  name: string;
}

export const STABLECOINS: StablecoinInfo[] = [
  { address: STABLECOIN_ADDRESSES.ALPHA_USD, symbol: "αUSD", name: "Alpha USD" },
  { address: STABLECOIN_ADDRESSES.PATH_USD, symbol: "pUSD", name: "Path USD" },
  { address: STABLECOIN_ADDRESSES.BETA_USD, symbol: "βUSD", name: "Beta USD" },
  { address: STABLECOIN_ADDRESSES.THETA_USD, symbol: "θUSD", name: "Theta USD" },
];

export interface StablecoinBalance extends StablecoinInfo {
  balance: bigint;
  formattedBalance: string;
}

export function useStablecoinBalances() {
  const { address } = useAccount();

  const alphaQuery = Hooks.token.useGetBalance({
    account: address!,
    token: STABLECOIN_ADDRESSES.ALPHA_USD,
    query: { enabled: !!address },
  });

  const pathQuery = Hooks.token.useGetBalance({
    account: address!,
    token: STABLECOIN_ADDRESSES.PATH_USD,
    query: { enabled: !!address },
  });

  const betaQuery = Hooks.token.useGetBalance({
    account: address!,
    token: STABLECOIN_ADDRESSES.BETA_USD,
    query: { enabled: !!address },
  });

  const thetaQuery = Hooks.token.useGetBalance({
    account: address!,
    token: STABLECOIN_ADDRESSES.THETA_USD,
    query: { enabled: !!address },
  });

  const formatBalance = (balance: bigint | undefined): string => {
    if (!balance) return "0.00";
    const num = Number(balance) / 1e6;
    if (!Number.isFinite(num) || num < 0) return "0.00";
    return num.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 });
  };

  const balances: StablecoinBalance[] = [
    {
      ...STABLECOINS[0],
      balance: alphaQuery.data ?? 0n,
      formattedBalance: formatBalance(alphaQuery.data),
    },
    {
      ...STABLECOINS[1],
      balance: pathQuery.data ?? 0n,
      formattedBalance: formatBalance(pathQuery.data),
    },
    {
      ...STABLECOINS[2],
      balance: betaQuery.data ?? 0n,
      formattedBalance: formatBalance(betaQuery.data),
    },
    {
      ...STABLECOINS[3],
      balance: thetaQuery.data ?? 0n,
      formattedBalance: formatBalance(thetaQuery.data),
    },
  ];

  const isLoading =
    alphaQuery.isLoading || pathQuery.isLoading || betaQuery.isLoading || thetaQuery.isLoading;

  return {
    balances,
    isLoading,
  };
}
