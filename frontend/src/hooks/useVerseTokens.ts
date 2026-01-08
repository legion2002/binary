import { type Address, zeroAddress } from "viem";
import { useQuery } from "@tanstack/react-query";
import { usePublicClient } from "wagmi";
import { useContracts, MULTIVERSE_ABI } from "./useContracts";

export interface VerseTokens {
  yesVerse: Address;
  noVerse: Address;
  exists: boolean;
}

/**
 * Query verse token addresses directly from the Multiverse contract.
 * This is more reliable than backend data since verse tokens can be
 * created by any user via split().
 */
export function useVerseTokens(
  asset: Address | undefined,
  marketHash: string | undefined
): {
  data: VerseTokens | undefined;
  isLoading: boolean;
  error: Error | null;
  refetch: () => void;
} {
  const { contracts } = useContracts();
  const publicClient = usePublicClient();

  const { data, isLoading, error, refetch } = useQuery({
    queryKey: ["verseTokens", asset, marketHash],
    queryFn: async (): Promise<VerseTokens> => {
      if (!publicClient || !contracts || !asset || !marketHash) {
        throw new Error("Missing required parameters");
      }

      const [yesVerse, noVerse] = await publicClient.readContract({
        address: contracts.MULTIVERSE,
        abi: MULTIVERSE_ABI,
        functionName: "getVerseAddress",
        args: [asset, marketHash as `0x${string}`],
      });

      return {
        yesVerse,
        noVerse,
        exists: yesVerse !== zeroAddress,
      };
    },
    enabled: !!publicClient && !!contracts && !!asset && !!marketHash,
    staleTime: 10_000, // Consider data fresh for 10 seconds
  });

  return {
    data,
    isLoading,
    error: error as Error | null,
    refetch,
  };
}

/**
 * Query key for verse tokens - exported for cache invalidation
 */
export function verseTokensQueryKey(asset: Address, marketHash: string) {
  return ["verseTokens", asset, marketHash];
}
