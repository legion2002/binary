import { useAccount, useConnect, useDisconnect } from 'wagmi';

export function WalletConnect() {
  const { address, isConnected } = useAccount();
  const { connect, connectors } = useConnect();
  const { disconnect } = useDisconnect();

  const formatAddress = (addr: string) => {
    return `${addr.slice(0, 6)}...${addr.slice(-4)}`;
  };

  if (isConnected && address) {
    return (
      <div className="flex items-center gap-4">
        <span className="text-sm text-gray-600">{formatAddress(address)}</span>
        <button
          onClick={() => disconnect()}
          className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200 transition-colors"
        >
          Disconnect
        </button>
      </div>
    );
  }

  // Porto should be the only connector we configured
  const connector = connectors[0];
  
  if (!connector) {
    return (
      <button
        disabled
        className="px-4 py-2 text-sm font-medium text-gray-400 bg-gray-200 rounded-lg cursor-not-allowed"
      >
        No wallet available
      </button>
    );
  }

  return (
    <button
      onClick={() => connect({ connector })}
      className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-lg hover:bg-blue-700 transition-colors"
    >
      Connect with Porto
    </button>
  );
}