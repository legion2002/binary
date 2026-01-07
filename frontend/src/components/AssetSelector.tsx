import { useState, useRef, useEffect } from "react";
import type { Address } from "viem";
import {
  useStablecoinBalances,
  type StablecoinBalance,
} from "../hooks/useStablecoinBalances";

interface AssetSelectorProps {
  selectedAsset: Address;
  onAssetChange: (asset: Address, balance: bigint) => void;
  availableAssets?: Address[];
  disabled?: boolean;
}

export function AssetSelector({
  selectedAsset,
  onAssetChange,
  availableAssets,
  disabled = false,
}: AssetSelectorProps) {
  const [isOpen, setIsOpen] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);
  const { balances, isLoading } = useStablecoinBalances();

  const filteredBalances = availableAssets
    ? balances.filter((b) =>
        availableAssets.some(
          (a) => a.toLowerCase() === b.address.toLowerCase()
        )
      )
    : balances;

  const selectedCoin = filteredBalances.find(
    (b) => b.address.toLowerCase() === selectedAsset.toLowerCase()
  );

  // Sync balance with parent when it loads or changes
  useEffect(() => {
    if (selectedCoin && !isLoading) {
      onAssetChange(selectedCoin.address, selectedCoin.balance);
    }
  }, [selectedCoin?.balance, isLoading]);

  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (
        dropdownRef.current &&
        !dropdownRef.current.contains(event.target as Node)
      ) {
        setIsOpen(false);
      }
    }
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  const handleSelect = (coin: StablecoinBalance) => {
    onAssetChange(coin.address, coin.balance);
    setIsOpen(false);
  };

  return (
    <div className="asset-selector" ref={dropdownRef}>
      <button
        type="button"
        className="asset-selector-trigger"
        onClick={() => !disabled && setIsOpen(!isOpen)}
        disabled={disabled}
        data-testid="asset-selector-trigger"
      >
        <div className="asset-selector-content">
          <span className="asset-symbol">{selectedCoin?.symbol ?? "—"}</span>
          <span className="asset-balance">
            {isLoading ? "..." : `$${selectedCoin?.formattedBalance ?? "0.00"}`}
          </span>
        </div>
        <span className={`asset-chevron ${isOpen ? "open" : ""}`}>▼</span>
      </button>

      {isOpen && (
        <div className="asset-dropdown" data-testid="asset-dropdown">
          {balances.map((coin) => (
            <button
              key={coin.address}
              type="button"
              className={`asset-option ${
                coin.address.toLowerCase() === selectedAsset.toLowerCase()
                  ? "selected"
                  : ""
              }`}
              onClick={() => handleSelect(coin)}
              data-testid={`asset-option-${coin.symbol}`}
            >
              <div className="asset-option-left">
                <span className="asset-option-symbol">{coin.symbol}</span>
                <span className="asset-option-name">{coin.name}</span>
              </div>
              <span className="asset-option-balance">
                ${coin.formattedBalance}
              </span>
            </button>
          ))}
        </div>
      )}
    </div>
  );
}
