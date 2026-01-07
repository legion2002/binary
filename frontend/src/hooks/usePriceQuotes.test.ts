import { describe, it, expect, vi, beforeEach } from "vitest";
import { renderHook } from "@testing-library/react";
import { usePriceQuotes } from "./usePriceQuotes";
import { parseUnits } from "viem";

vi.mock("wagmi/tempo", () => ({
  Hooks: {
    dex: {
      useBuyQuote: vi.fn(),
    },
  },
}));

import { Hooks } from "wagmi/tempo";

const mockUseBuyQuote = Hooks.dex.useBuyQuote as ReturnType<typeof vi.fn>;

describe("usePriceQuotes", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("returns 50/50 probability when no prices available", () => {
    mockUseBuyQuote.mockReturnValue({ data: undefined, isLoading: false });

    const { result } = renderHook(() =>
      usePriceQuotes("0xYesToken", "0xNoToken")
    );

    expect(result.current.yesProbability).toBe(50);
    expect(result.current.noProbability).toBe(50);
    expect(result.current.yesPrice).toBeNull();
    expect(result.current.noPrice).toBeNull();
  });

  it("returns correct probability when both prices available", () => {
    mockUseBuyQuote
      .mockReturnValueOnce({
        data: parseUnits("0.7", 6),
        isLoading: false,
      })
      .mockReturnValueOnce({
        data: parseUnits("0.3", 6),
        isLoading: false,
      });

    const { result } = renderHook(() =>
      usePriceQuotes("0xYesToken", "0xNoToken")
    );

    expect(result.current.yesPrice).toBe(0.7);
    expect(result.current.noPrice).toBe(0.3);
    expect(result.current.yesProbability).toBe(70);
    expect(result.current.noProbability).toBe(30);
  });

  it("returns isLoading true when either quote is loading", () => {
    mockUseBuyQuote
      .mockReturnValueOnce({ data: undefined, isLoading: true })
      .mockReturnValueOnce({ data: undefined, isLoading: false });

    const { result } = renderHook(() =>
      usePriceQuotes("0xYesToken", "0xNoToken")
    );

    expect(result.current.isLoading).toBe(true);
  });

  it("handles edge case when only yesPrice is available", () => {
    mockUseBuyQuote
      .mockReturnValueOnce({
        data: parseUnits("0.6", 6),
        isLoading: false,
      })
      .mockReturnValueOnce({ data: undefined, isLoading: false });

    const { result } = renderHook(() =>
      usePriceQuotes("0xYesToken", "0xNoToken")
    );

    expect(result.current.yesPrice).toBe(0.6);
    expect(result.current.noPrice).toBeNull();
    expect(result.current.yesProbability).toBe(60);
    expect(result.current.noProbability).toBe(40);
  });

  it("handles edge case when only noPrice is available", () => {
    mockUseBuyQuote
      .mockReturnValueOnce({ data: undefined, isLoading: false })
      .mockReturnValueOnce({
        data: parseUnits("0.4", 6),
        isLoading: false,
      });

    const { result } = renderHook(() =>
      usePriceQuotes("0xYesToken", "0xNoToken")
    );

    expect(result.current.yesPrice).toBeNull();
    expect(result.current.noPrice).toBe(0.4);
    expect(result.current.yesProbability).toBe(60);
    expect(result.current.noProbability).toBe(40);
  });
});
