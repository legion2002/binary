# Binary

**Price feeds from the future.**

- *What will ETH trade at if FOCIL is included in the Glamsterdam hardfork?*  
- *How will GPU prices diverge if Trump is (or isn't) re-elected?*

Binary is a lean, highly-optimized prediction-market framework that spawns self-contained economies around every yes/no question.


## Market

A **market** is simply an oracle answering a binary question.

- **Will FOCIL be included in the Glamsterdam hardfork?**  
- **Will Trump win the next U.S. presidential election?**


## Splitter

1. Escrow **1 USDC** in a smart contract.  
2. Receive **1 YES_USDC** and **1 NO_USDC**.  
3. **YES_USDC** pays **1 USDC** if the oracle says **YES**, otherwise **0**.  
4. **NO_USDC** does the opposite.

Holding **1 USDC ≡ 1 YES_USDC + 1 NO_USDC**.  
The same mechanic works for any other asset in the same market —e.g., split **1 ETH** into **YES_ETH** and **NO_ETH**.


## Economy

Every economy—onchain or offchain—reduces to three building blocks:

1. **Assets:** goods and services.  
2. **Trade:** venues to exchange them.  
3. **Credit:** mechanisms to lend, borrow, or lever up.

#### YES-Economy

- Assets: **YES_USDC**, **YES_ETH**  
- AMM: **YES_ETH / YES_USDC**  
- Lending: borrow **YES_ETH** against **YES_USDC**

#### NO-Economy

- Assets: **NO_USDC**, **NO_ETH**  
- AMM: **NO_ETH / NO_USDC**  
- Lending: borrow **NO_ETH** against **NO_USDC**

## Price

"**Price**" is distilled collective intelligence of all of humanity - a live forecast of future value.  

Binary markets allow us to use the concept of price, to extract valuable information from the future.

Each possible answer spawns its own self-contained economy. The price feeds inside these separate "verses" reveal what every asset is worth under that specific outcome.

Example:

- **Market**: *Will FOCIL be included in the Glamsterdam hardfork?*  
- **YES_ETH / YES_USDC** quotes ETH's expected value *if the fix happens*.  
- **NO_ETH / NO_USDC** quotes ETH's value *if it doesn't*.

Comparing the two feeds gives the Ethereum community a clear, quantitative signal about which path the future favors—and where to focus effort today.

---

## Quick Start

```bash
# Install dependencies
make install

# Start full local environment (Tempo + contracts + backend + frontend)
make dev

# Run integration tests
make test
```

**Available Commands:**

| Command | Description |
|---------|-------------|
| `make dev` | Start full local stack (Tempo + contracts + backend + frontend) |
| `make test` | Run integration tests against full stack |
| `make test-unit` | Run unit tests only (fast, no Docker) |
| `make build` | Build for production |
| `make install` | Install all dependencies |

---

## Deployment

Binary is deployed on **Tempo** - a Layer 1 blockchain optimized for payments with native TIP-20 token support.

### Network: Tempo Testnet (Andantino)

| Property | Value |
|----------|-------|
| Chain ID | `42429` |
| RPC URL | `https://rpc.testnet.tempo.xyz` |
| Explorer | [explore.tempo.xyz](https://explore.tempo.xyz) |

### Prerequisites

1. **Get testnet funds**: Request PATH_USD from the [Tempo Faucet](https://docs.tempo.xyz/quickstart/faucet)
2. **Set private key**: Add `PRIVATE_KEY=...` to `.env`

### Deploy to Testnet

```bash
bun run deploy:testnet
```

Or manually with forge:

```bash
cd contracts
forge script script/DeployAndSeedMarkets.s.sol:DeployAndSeedMarkets \
    --rpc-url https://rpc.testnet.tempo.xyz \
    --broadcast \
    --private-key $PRIVATE_KEY \
    -vvv
```

### Tempo Precompile Addresses

| Contract | Address |
|----------|---------|
| TIP-20 Factory | `0x20Fc000000000000000000000000000000000000` |
| PATH_USD | `0x20C0000000000000000000000000000000000000` |
| Stablecoin DEX | `0xDEc0000000000000000000000000000000000000` |

### Tempo-Specific Notes

- **No native gas token** - Fees are paid in TIP-20 tokens (defaults to PATH_USD)
- **Fast finality** - ~0.5 second blocks with deterministic finality
- **TIP-20 tokens** - Binary creates YES/NO verse tokens via the TIP-20 Factory

---

## Local Development

The local dev environment uses Docker to run a Tempo node and automatically deploys contracts.

```bash
make dev
```

This will:
1. Start a local Tempo node (Docker)
2. Configure fee token to PathUSD
3. Deploy contracts with seeded markets
4. Start the backend server
5. Start the frontend dev server

### Local Node Details

| Property | Value |
|----------|-------|
| RPC URL | `http://localhost:9545` |
| Chain ID | `42429` |
| Test Private Key | `0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80` |
| Test Account | `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266` |

---

## Project Structure

```
binary/
├── backend/           # Rust/Axum API server
│   ├── src/           # Backend source code
│   └── tests/         # Integration tests
├── frontend/          # React/Vite app
│   └── src/           # Frontend source code
├── contracts/         # Solidity contracts (Foundry project)
│   ├── src/           # Contract source files
│   │   ├── MultiVerse.sol    # Core prediction market contract
│   │   └── TrustedOracle.sol # Oracle implementation
│   ├── test/          # Contract tests
│   ├── script/        # Forge deploy scripts
│   └── lib/           # Forge dependencies (forge-std, solady)
├── script/
│   ├── env.ts         # Central orchestrator
│   ├── deploy-tempo.ts # Testnet deploy script
│   └── lib/           # Shared utilities
├── Makefile           # Unified commands
├── package.json       # Root package
└── docker-compose.yml # Tempo node config
```

---

## API Documentation

See [backend/API.md](backend/API.md) for full API documentation.

### Quick Reference

```bash
# Get all markets
curl http://127.0.0.1:3000/markets

# Get single market
curl http://127.0.0.1:3000/markets/{marketHash}

# Create market (admin)
curl -X POST http://127.0.0.1:3000/admin/markets/open \
  -H 'Authorization: Bearer YOUR_API_KEY' \
  -H 'Content-Type: application/json' \
  -d '{"question": "Will X happen?", "resolutionDeadline": 1767225600}'
```
