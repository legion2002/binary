import type { Address } from "viem";
import { useAccount } from "wagmi";
import { Hooks } from "wagmi/tempo";
import { CONTRACTS } from "../config/contracts";

export interface StablecoinInfo {
  address: Address;
  symbol: string;
  name: string;
}

export const STABLECOINS: StablecoinInfo[] = [
  { address: CONTRACTS.ALPHA_USD as Address, symbol: "αUSD", name: "Alpha USD" },
  { address: CONTRACTS.PATH_USD as Address, symbol: "pUSD", name: "Path USD" },
  { address: CONTRACTS.BETA_USD as Address, symbol: "βUSD", name: "Beta USD" },
  { address: CONTRACTS.THETA_USD as Address, symbol: "θUSD", name: "Theta USD" },
];

export interface StablecoinBalance extends StablecoinInfo {
  balance: bigint;
  formattedBalance: string;
}

export function useStablecoinBalances() {
  const { address } = useAccount();

  const alphaQuery = Hooks.token.useGetBalance({
    account: address!,
    token: CONTRACTS.ALPHA_USD as Address,
    query: { enabled: !!address },
  });

  const pathQuery = Hooks.token.useGetBalance({
    account: address!,
    token: CONTRACTS.PATH_USD as Address,
    query: { enabled: !!address },
  });

  const betaQuery = Hooks.token.useGetBalance({
    account: address!,
    token: CONTRACTS.BETA_USD as Address,
    query: { enabled: !!address },
  });

  const thetaQuery = Hooks.token.useGetBalance({
    account: address!,
    token: CONTRACTS.THETA_USD as Address,
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
