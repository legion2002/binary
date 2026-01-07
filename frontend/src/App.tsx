import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { WagmiProvider } from "wagmi";
import { wagmiConfig } from "./config/wagmi";
import { MarketList } from "./components/MarketList";
import { WalletConnect } from "./components/WalletConnect";

const queryClient = new QueryClient();

function App() {
  return (
    <WagmiProvider config={wagmiConfig}>
      <QueryClientProvider client={queryClient}>
        <div className="min-h-screen">
          <header className="header" data-testid="header">
            <div className="header-inner">
              <a href="/" className="logo" data-testid="logo">
                <span className="logo-dot" />
                Binary
              </a>
              <div className="flex items-center gap-3">
                <div className="tempo-badge" data-testid="tempo-badge">
                  on <span>Tempo</span>
                </div>
                <WalletConnect />
              </div>
            </div>
          </header>

          <main className="max-w-3xl mx-auto px-4 py-6">
            <MarketList />
          </main>
        </div>
      </QueryClientProvider>
    </WagmiProvider>
  );
}

export default App;
