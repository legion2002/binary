import { useState } from "react";
import { useAccount, useConnect, useDisconnect } from "wagmi";
import { FundButton } from "./FundButton";

export function WalletConnect() {
  const { address, isConnected } = useAccount();
  const { connect, connectors, isPending } = useConnect();
  const { disconnect } = useDisconnect();
  const [copied, setCopied] = useState(false);

  const copyAddress = async () => {
    if (address) {
      await navigator.clipboard.writeText(address);
      setCopied(true);
      setTimeout(() => setCopied(false), 1500);
    }
  };

  const handleConnect = (isSignUp: boolean) => {
    const webAuthnConnector = connectors.find((c) => c.id === "webAuthn");
    
    if (webAuthnConnector) {
      // Access key is configured at the connector level in wagmi config
      const capabilities = isSignUp 
        ? { type: 'sign-up' as const, label: `Binary ${new Date().toLocaleDateString()}` }
        : { type: 'sign-in' as const, selectAccount: true };
      
      connect(
        { 
          connector: webAuthnConnector,
          capabilities,
        } as Parameters<typeof connect>[0],
        {
          onError: (err) => {
            console.error("[WalletConnect] Connect error:", err);
          },
        }
      );
    }
  };

  const formatAddress = (addr: string) => {
    return `${addr.slice(0, 6)}...${addr.slice(-4)}`;
  };

  if (isConnected && address) {
    return (
      <div className="flex gap-2 items-center">
        <span 
          className="text-sm cursor-pointer hover:opacity-70"
          onClick={copyAddress}
          title="Click to copy address"
        >
          {copied ? "Copied!" : formatAddress(address)}
        </span>
        <FundButton />
        <button
          className="wallet-btn"
          onClick={() => disconnect()}
        >
          Sign Out
        </button>
      </div>
    );
  }

  return (
    <div className="flex gap-2">
      <button
        className="wallet-btn"
        onClick={() => handleConnect(false)}
        disabled={isPending}
      >
        {isPending ? "..." : "Log In"}
      </button>
      <button
        className="wallet-btn"
        onClick={() => handleConnect(true)}
        disabled={isPending}
      >
        {isPending ? "..." : "Sign Up"}
      </button>
    </div>
  );
}
