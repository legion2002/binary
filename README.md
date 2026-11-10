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
| `make dev` | Start full local stack (Tempo + UniV2 + contracts + backend + frontend) |
| `make test` | Run integration tests against full stack |
| `make test-unit` | Run unit tests only (fast, no Docker) |
| `make testnet` | Deploy and run full stack on Tempo testnet |
| `make testnet-liquidity` | Add UniV2 liquidity to testnet markets |
| `make build` | Build for production |
| `make install` | Install all dependencies |

---

## Deployment

Binary is deployed on **Tempo** - a Layer 1 blockchain optimized for payments with native TIP-20 token support.

### Network: Tempo Testnet (Moderato)

| Property | Value |
|----------|-------|
| Chain ID | `42431` |
| RPC URL | `https://rpc.moderato.tempo.xyz` |
| Explorer | [explore.tempo.xyz](https://explore.tempo.xyz) |

### Prerequisites

1. **Get testnet funds**: Request PATH_USD from the [Tempo Faucet](https://docs.tempo.xyz/quickstart/faucet)
2. **Set private key**: Add `PRIVATE_KEY=...` to `.env`

### Deploy to Testnet

```bash
bun run testnet
```

Or manually with forge (contracts only, markets created via admin API):

```bash
cd contracts
forge script script/Deploy.s.sol:Deploy \
    --rpc-url https://rpc.moderato.tempo.xyz \
    --broadcast \
    --private-key $PRIVATE_KEY \
    -vvv
```

### Contract Addresses

| Contract | Address |
|----------|---------|
| TIP-20 Factory | `0x20Fc000000000000000000000000000000000000` |
| PATH_USD | `0x20C0000000000000000000000000000000000000` |
| UniV2 Factory | Deployed dynamically (see `.univ2-config.json`) |
| UniV2 Router | Deployed dynamically (see `.univ2-config.json`) |

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
3. Deploy contracts (MultiVerse + TrustedOracle + UniV2 Factory/Router)
4. Start the backend server
5. Create seed markets via admin API
6. Add UniV2 liquidity for market trading pairs
7. Start the frontend dev server

### Local Node Details

| Property | Value |
|----------|-------|
| RPC URL | `http://localhost:9545` |
| Chain ID | `1337` |
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
│   ├── env.ts         # Local dev orchestrator
│   ├── env-testnet.ts # Testnet orchestrator
│   └── lib/           # Shared utilities
├── Makefile           # Unified commands
├── package.json       # Root package
└── docker-compose.yml # Tempo node config
```

---

## Oracle: Witan council

Markets can resolve from a decision of Witan, a council of AI models that decides a question and signs the result. The council oracle sits next to the trusted oracle: set `MARKET_ORACLE=council` on the backend to open new markets with it.

### How it works

1. **Market creation.** `POST /admin/markets/open` picks a random 16-byte question salt and computes the salted question hash `sha256(JCS({"question": q, "salt": salt}))` (RFC 8785 canonical JSON). It opens the market on the MultiVerse with `CouncilOracle` as its oracle and calls `CouncilOracle.registerMarket(marketHash, councilHash, questionHash)`, which pins the market to one council config (`COUNCIL_HASH`) and that question, once. The salt and hashes are stored and published in the market API, so anyone can recompute them.
2. **Resolution.** `POST /admin/markets/:marketHash/resolve` with evidence per party runs `witan query open --question-salt <salt> ...` (paid per call over MPP on Tempo), checks the record with `witan query verify` against the pinned signers, repeats the contract's checks locally, and sends `CouncilOracle.resolve(marketHash, recordJcs, commitmentsJcs, signature)`. Then it calls `MultiVerse.resolve`.
3. **On chain**, `CouncilOracle.resolve`:
   - hashes the record bytes with SHA-256 (the record's digest) and recovers the EIP-191 signer, which must be a trusted signer;
   - requires `"council_hash"` to equal the registered council;
   - recomputes `evidence_root = sha256({"commitments":[...],"question_hash":"0x..."})` from the registered question hash and the record's commitments, and requires the record's `"evidence_root"` to match, which proves the council decided this exact question;
   - reads `"outcome"`: `{"type":"binary","value":"yes"}` resolves YES, `"no"` resolves NO. Anything else (an invalid decision) reverts, the market stays open, and it can be resolved with a new query.

   The resolution is final once set. The contract reads fields by byte pattern; `contracts/src/CouncilOracle.sol` documents why the first match is always the real field.

### Trust model

- **Signer pinned on chain.** Only records signed by an address the owner trusts (`setSigner`) resolve markets. Witan v1 runs without a hardware TEE, so the council's signer key is the trust root: whoever holds it can sign any outcome.
- **Council config pinned by hash.** The models, voting rule and outcome type are fixed by the council hash registered for the market. Use a council with a binary outcome.
- **Question pinned by salted hash.** The council cannot be asked a different question for the market. The salt keeps short questions from being guessed from public records until the backend publishes it.
- **The resolver chooses the evidence.** Evidence goes to the council encrypted and is not on chain; a resolver could give the council slanted evidence. That is why only resolvers (`setResolver`) may submit records. Each query file (question, salts and evidence) is kept in `COUNCIL_QUERY_DIR` and checks against the record's commitments.

### Setup

```bash
# Deploy CouncilOracle alongside the other contracts (deployer = owner and resolver)
COUNCIL_SIGNER=0x<council signer> forge script script/Deploy.s.sol:Deploy --broadcast ...

# Backend (see backend/.env.example)
MARKET_ORACLE=council
COUNCIL_ORACLE_ADDRESS=0x...        # or "councilOracle" in deployments.json
COUNCIL_HASH=0x...                  # council config every new market is pinned to
COUNCIL_SIGNERS=0x...               # trusted record signers, comma-separated
COUNCIL_URL=https://...             # council API (or WITAN_URL)
COUNCIL_PRIVATE_KEY=0x...           # Tempo key that pays for queries
COUNCIL_CLI=witan                   # path to the witan CLI
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

# Resolve a council oracle market (admin)
curl -X POST http://127.0.0.1:3000/admin/markets/{marketHash}/resolve \
  -H 'Authorization: Bearer YOUR_API_KEY' \
  -H 'Content-Type: application/json' \
  -d '{"evidence": [{"party": "yes", "text": "..."}, {"party": "no", "text": "..."}]}'

# Create market (admin)
curl -X POST http://127.0.0.1:3000/admin/markets/open \
  -H 'Authorization: Bearer YOUR_API_KEY' \
  -H 'Content-Type: application/json' \
  -d '{"question": "Will X happen?", "resolutionDeadline": 1767225600}'
```

---

### Roadmap

- Create a TUI to manage markets and resolve them
- Deploy to production once testnet is stable
- Advanced AMM features (concentrated liquidity, dynamic fees)
- Cross-market arbitrage detection and alerts
