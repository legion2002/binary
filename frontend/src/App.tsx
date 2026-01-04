import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { PasskeyProvider } from "./contexts/PasskeyContext";
import { MarketList } from "./components/MarketList";
import { MarketDetail } from "./components/MarketDetail";
import { WalletConnect } from "./components/WalletConnect";

const queryClient = new QueryClient();

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <PasskeyProvider>
        <Router>
          <div className="min-h-screen bg-gray-50">
            <header className="bg-white shadow-sm border-b">
              <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div className="flex justify-between items-center h-16">
                  <h1 className="text-xl font-semibold">Binary Markets</h1>
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
      </PasskeyProvider>
    </QueryClientProvider>
  );
}

export default App;
