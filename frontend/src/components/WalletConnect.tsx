import { useAccount, useConnect, useDisconnect } from "wagmi";

export function WalletConnect() {
  const { address, isConnected } = useAccount();
  const { connect, connectors, isPending, error } = useConnect();
  const { disconnect } = useDisconnect();

  const formatAddress = (addr: string) => {
    return `${addr.slice(0, 6)}...${addr.slice(-4)}`;
  };

  // Get the webAuthn connector
  const webAuthnConnector = connectors.find((c) => c.id === "webauthn");

  if (isConnected && address) {
    return (
      <div className="flex items-center gap-4">
        <span className="text-sm text-secondary font-mono">
          {formatAddress(address)}
        </span>
        <button
          onClick={() => disconnect()}
          className="px-4 py-2 text-sm font-medium text-secondary bg-dark-tertiary rounded-lg hover:bg-dark-hover hover:text-primary transition-all border border-dark"
        >
          Sign Out
        </button>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-end gap-2">
      <div className="flex gap-2">
        <button
          onClick={() => webAuthnConnector && connect({ connector: webAuthnConnector })}
          disabled={isPending || !webAuthnConnector}
          className="btn-primary text-sm"
        >
          {isPending ? "Loading..." : "Connect with Passkey"}
        </button>
      </div>
      {error && (
        <span className="text-xs text-accent-red">{error.message}</span>
      )}
    </div>
  );
}
