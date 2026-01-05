import { usePasskey } from "../contexts/PasskeyContext";

export function WalletConnect() {
  const {
    address,
    isConnected,
    signUp,
    signIn,
    signOut,
    hasSavedCredential,
    isLoading,
    error,
  } = usePasskey();

  const formatAddress = (addr: string) => {
    return `${addr.slice(0, 6)}...${addr.slice(-4)}`;
  };

  if (isConnected && address) {
    return (
      <div className="flex items-center gap-4">
        <span className="text-sm text-secondary font-mono">{formatAddress(address)}</span>
        <button
          onClick={signOut}
          className="px-4 py-2 text-sm font-medium text-secondary bg-dark-tertiary rounded-lg hover:bg-dark-hover hover:text-primary transition-all border border-dark"
        >
          Sign Out
        </button>
      </div>
    );
  }

  const hasSaved = hasSavedCredential();

  return (
    <div className="flex flex-col items-end gap-2">
      <div className="flex gap-2">
        <button
          onClick={signUp}
          disabled={isLoading}
          className="btn-primary text-sm"
        >
          {isLoading ? "Loading..." : "Sign Up"}
        </button>
        <button
          onClick={signIn}
          disabled={isLoading || !hasSaved}
          className="btn-secondary text-sm border-accent-purple text-accent-purple hover:bg-accent-purple-dim"
          title={!hasSaved ? "No saved credential. Sign up first." : undefined}
        >
          Sign In
        </button>
      </div>
      {error && (
        <span className="text-xs text-accent-red">{error.message}</span>
      )}
    </div>
  );
}
