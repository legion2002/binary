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
        <span className="text-sm text-gray-600">{formatAddress(address)}</span>
        <button
          onClick={signOut}
          className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200 transition-colors"
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
          className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-lg hover:bg-blue-700 transition-colors disabled:bg-gray-400 disabled:cursor-not-allowed"
        >
          {isLoading ? "Loading..." : "Sign Up"}
        </button>
        <button
          onClick={signIn}
          disabled={isLoading || !hasSaved}
          className="px-4 py-2 text-sm font-medium text-blue-600 bg-blue-100 rounded-lg hover:bg-blue-200 transition-colors disabled:text-gray-400 disabled:bg-gray-100 disabled:cursor-not-allowed"
          title={!hasSaved ? "No saved credential. Sign up first." : undefined}
        >
          Sign In
        </button>
      </div>
      {error && (
        <span className="text-xs text-red-500">{error.message}</span>
      )}
    </div>
  );
}
