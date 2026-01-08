import { useQuery } from "@tanstack/react-query";
import { fetchConfig } from "../api/client";

/**
 * Hook to fetch and cache backend configuration (contract addresses, etc.)
 * This should be called early in the app lifecycle to ensure config is available
 */
export function useConfig() {
  return useQuery({
    queryKey: ["config"],
    queryFn: fetchConfig,
    staleTime: Infinity, // Config never goes stale during a session
    gcTime: Infinity, // Never garbage collect
    retry: 3,
    retryDelay: 1000,
  });
}
