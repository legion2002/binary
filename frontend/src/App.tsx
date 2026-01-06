import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { WagmiProvider } from "wagmi";
import { wagmiConfig } from "./config/wagmi";
import { MarketList } from "./components/MarketList";
import { MarketDetail } from "./components/MarketDetail";
import { WalletConnect } from "./components/WalletConnect";

const queryClient = new QueryClient();

function App() {
  return (
    <WagmiProvider config={wagmiConfig}>
      <QueryClientProvider client={queryClient}>
        <Router>
          <div className="min-h-screen bg-dark">
            <header className="header-glass">
              <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div className="flex justify-between items-center h-16">
                  <div className="flex items-center gap-4">
                    <a href="/" className="logo-link">
                      <span className="logo-binary">Binary</span>
                      <span className="logo-markets">Markets</span>
                    </a>
                    <span className="powered-by hidden sm:flex">
                      <span className="powered-text">on</span>
                      <span className="tempo-badge">Tempo</span>
                    </span>
                  </div>
                  <WalletConnect />
                </div>
              </div>
            </header>

            <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
              <Routes>
                <Route path="/" element={<MarketList />} />
                <Route path="/market/:marketHash" element={<MarketDetail />} />
              </Routes>
            </main>
          </div>
        </Router>
      </QueryClientProvider>
    </WagmiProvider>
  );
}

export default App;
