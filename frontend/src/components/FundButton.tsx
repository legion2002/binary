import { useState, useRef, useEffect } from "react";
import type { Address } from "viem";
import { useAccount } from "wagmi";
import {
  useStablecoinBalances,
  STABLECOINS,
} from "../hooks/useStablecoinBalances";
import { useFaucet } from "../hooks/useFaucet";

const DEFAULT_AMOUNT = "1000";
const DECIMALS = 6;

export function FundButton() {
  const { isConnected } = useAccount();
  const { balances, isLoading: balancesLoading, refetch: refetchBalances } = useStablecoinBalances();
  const { fund, isPending } = useFaucet();
  const [isOpen, setIsOpen] = useState(false);
  const [selectedToken, setSelectedToken] = useState<Address>(STABLECOINS[0].address);
  const [amount, setAmount] = useState(DEFAULT_AMOUNT);
  const [showSuccess, setShowSuccess] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);

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

  const handleFund = async () => {
    try {
      const amountBigInt = BigInt(Math.floor(parseFloat(amount) * 10 ** DECIMALS));
      await fund({ token: selectedToken, amount: amountBigInt });
      // Refetch balances after successful funding
      refetchBalances();
      setShowSuccess(true);
      setTimeout(() => setShowSuccess(false), 2000);
      setIsOpen(false);
    } catch (err) {
      console.error("Fund error:", err);
    }
  };

  const selectedCoin = STABLECOINS.find(
    (c) => c.address.toLowerCase() === selectedToken.toLowerCase()
  );

  const getBalance = (address: Address) => {
    const coin = balances.find(
      (b) => b.address.toLowerCase() === address.toLowerCase()
    );
    return coin?.formattedBalance ?? "0.00";
  };

  if (!isConnected) {
    return null;
  }

  return (
    <div className="fund-button-container" ref={dropdownRef}>
      <button
        type="button"
        className="fund-btn"
        onClick={() => setIsOpen(!isOpen)}
        disabled={isPending}
        data-testid="fund-button"
      >
        {isPending ? (
          <span className="fund-btn-content">
            <span className="spinner-small" />
            Funding...
          </span>
        ) : showSuccess ? (
          <span className="fund-btn-content">✓ Funded!</span>
        ) : (
          <span className="fund-btn-content">
            <span className="fund-icon">+</span>
            Fund
          </span>
        )}
      </button>

      {isOpen && !isPending && (
        <div className="fund-dropdown" data-testid="fund-dropdown">
          <div className="fund-dropdown-section">
            <label className="fund-label">Select Token</label>
            <div className="fund-token-list">
              {STABLECOINS.map((coin) => (
                <button
                  key={coin.address}
                  type="button"
                  className={`fund-token-option ${
                    selectedToken.toLowerCase() === coin.address.toLowerCase() ? "selected" : ""
                  }`}
                  onClick={() => setSelectedToken(coin.address)}
                  data-testid={`fund-token-${coin.symbol}`}
                >
                  <div className="fund-token-left">
                    <span className="fund-token-symbol">{coin.symbol}</span>
                    <span className="fund-token-name">{coin.name}</span>
                  </div>
                  <span className="fund-token-balance">
                    {balancesLoading ? "..." : `$${getBalance(coin.address)}`}
                  </span>
                </button>
              ))}
            </div>
          </div>

          <div className="fund-dropdown-section">
            <label className="fund-label" htmlFor="fund-amount">
              Amount
            </label>
            <div className="fund-amount-input-wrapper">
              <span className="fund-amount-prefix">$</span>
              <input
                id="fund-amount"
                type="number"
                className="fund-amount-input"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                min="1"
                max="1000000"
                step="100"
                data-testid="fund-amount-input"
              />
            </div>
            <div className="fund-amount-presets">
              {["100", "500", "1000", "5000"].map((preset) => (
                <button
                  key={preset}
                  type="button"
                  className={`fund-preset-btn ${amount === preset ? "selected" : ""}`}
                  onClick={() => setAmount(preset)}
                >
                  ${preset}
                </button>
              ))}
            </div>
          </div>

          <div className="fund-dropdown-footer">
            <button
              type="button"
              className="fund-action-btn"
              onClick={handleFund}
              disabled={isPending || !amount || parseFloat(amount) <= 0}
              data-testid="fund-action-button"
            >
              Fund ${amount} {selectedCoin?.symbol ?? ""}
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
