import { type Address, parseUnits, formatUnits } from "viem";
import { useBuyQuote } from "./useTempoDex";

// Default quote token (AlphaUSD)
const DEFAULT_QUOTE_TOKEN = "0x20C0000000000000000000000000000000000001" as Address;
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
  noTokenAddress: string | undefined,
  quoteToken?: Address
): PriceQuotesResult {
  const tokenIn = quoteToken ?? DEFAULT_QUOTE_TOKEN;

  const { data: yesBuyQuote, isLoading: yesLoading } = useBuyQuote({
    tokenIn,
    tokenOut: (yesTokenAddress ?? tokenIn) as Address,
    amountOut: ONE_TOKEN,
    query: {
      enabled: !!yesTokenAddress,
    },
  });

  const { data: noBuyQuote, isLoading: noLoading } = useBuyQuote({
    tokenIn,
    tokenOut: (noTokenAddress ?? tokenIn) as Address,
    amountOut: ONE_TOKEN,
    query: {
      enabled: !!noTokenAddress,
    },
  });

  const yesPrice = yesBuyQuote ? parseFloat(formatUnits(yesBuyQuote, 6)) : null;
  const noPrice = noBuyQuote ? parseFloat(formatUnits(noBuyQuote, 6)) : null;

  // Debug logging
  if (yesTokenAddress || noTokenAddress) {
    console.log("[usePriceQuotes]", {
      tokenIn,
      yesTokenAddress,
      noTokenAddress,
      yesBuyQuote: yesBuyQuote?.toString(),
      noBuyQuote: noBuyQuote?.toString(),
      yesPrice,
      noPrice,
    });
  }

  let yesProbability = 50;
  let noProbability = 50;

  if (yesPrice !== null && noPrice !== null && yesPrice + noPrice > 0) {
    // Calculate probability with full precision (no rounding)
    yesProbability = (yesPrice / (yesPrice + noPrice)) * 100;
    noProbability = 100 - yesProbability;
  } else if (yesPrice !== null && yesPrice > 0 && yesPrice <= 1) {
    yesProbability = yesPrice * 100;
    noProbability = 100 - yesProbability;
  } else if (noPrice !== null && noPrice > 0 && noPrice <= 1) {
    noProbability = noPrice * 100;
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
