import { type Address, parseUnits, formatUnits } from "viem";
import { Hooks } from "wagmi/tempo";

// AlphaUSD - primary trading currency (static precompile address)
const USD_TOKEN = "0x20C0000000000000000000000000000000000001" as Address;
const ONE_TOKEN = parseUnits("1", 6);

interface PriceQuotesResult {
  yesPrice: number | null;
  noPrice: number | null;
  yesProbability: number;
  noProbability: number;
  isLoading: boolean;
}

export function usePriceQuotes(
  yesTokenAddress: string | undefined,
  noTokenAddress: string | undefined
): PriceQuotesResult {
  const { data: yesBuyQuote, isLoading: yesLoading } = Hooks.dex.useBuyQuote({
    tokenIn: USD_TOKEN,
    tokenOut: (yesTokenAddress ?? USD_TOKEN) as Address,
    amountOut: ONE_TOKEN,
    query: {
      enabled: !!yesTokenAddress,
    },
  });

  const { data: noBuyQuote, isLoading: noLoading } = Hooks.dex.useBuyQuote({
    tokenIn: USD_TOKEN,
    tokenOut: (noTokenAddress ?? USD_TOKEN) as Address,
    amountOut: ONE_TOKEN,
    query: {
      enabled: !!noTokenAddress,
    },
  });

  const yesPrice = yesBuyQuote ? parseFloat(formatUnits(yesBuyQuote, 6)) : null;
  const noPrice = noBuyQuote ? parseFloat(formatUnits(noBuyQuote, 6)) : null;

  let yesProbability = 50;
  let noProbability = 50;

  if (yesPrice !== null && noPrice !== null && yesPrice + noPrice > 0) {
    yesProbability = Math.round((yesPrice / (yesPrice + noPrice)) * 100);
    noProbability = 100 - yesProbability;
  } else if (yesPrice !== null && yesPrice > 0 && yesPrice <= 1) {
    yesProbability = Math.round(yesPrice * 100);
    noProbability = 100 - yesProbability;
  } else if (noPrice !== null && noPrice > 0 && noPrice <= 1) {
    noProbability = Math.round(noPrice * 100);
    yesProbability = 100 - noProbability;
  }

  return {
    yesPrice,
    noPrice,
    yesProbability,
    noProbability,
    isLoading: yesLoading || noLoading,
  };
}
