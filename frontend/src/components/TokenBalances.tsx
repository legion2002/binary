import { formatUnits, type Address } from "viem";
import { useAccount } from "wagmi";
import { useTokenBalance } from "../hooks/useBalances";

interface TokenBalancesProps {
  yesTokenAddress: string | undefined;
  noTokenAddress: string | undefined;
  assetSymbol: string;
}

export function TokenBalances({
  yesTokenAddress,
  noTokenAddress,
  assetSymbol,
}: TokenBalancesProps) {
  const { address, isConnected } = useAccount();

  const { data: yesBalance, isLoading: yesLoading } = useTokenBalance({
    account: address,
    token: (yesTokenAddress ?? "0x0000000000000000000000000000000000000000") as Address,
    enabled: !!yesTokenAddress && !!address,
  });

  const { data: noBalance, isLoading: noLoading } = useTokenBalance({
    account: address,
    token: (noTokenAddress ?? "0x0000000000000000000000000000000000000000") as Address,
    enabled: !!noTokenAddress && !!address,
  });

  const formatBalance = (balance: bigint | undefined) => {
    if (!balance) return "0";
    const num = Number(formatUnits(balance, 6));
    if (num >= 1000) return num.toLocaleString(undefined, { maximumFractionDigits: 0 });
    if (num >= 1) return num.toLocaleString(undefined, { maximumFractionDigits: 2 });
    return num.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 });
  };

  if (!isConnected) return null;

  const isLoading = yesLoading || noLoading;

  return (
    <div className="verse-balances" data-testid="verse-balances">
      <span className="verse-balance yes">
        Yes {assetSymbol}: {isLoading ? "..." : formatBalance(yesBalance)}
      </span>
      <span className="verse-balance-divider">•</span>
      <span className="verse-balance no">
        No {assetSymbol}: {isLoading ? "..." : formatBalance(noBalance)}
      </span>
    </div>
  );
}
