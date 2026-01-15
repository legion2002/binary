# Binary Markets - Development Notes

## Quick Start

```bash
# Start full local environment (Tempo + contracts + backend + frontend)
make dev
# or
bun run dev

# Run integration tests
make test

# Run unit tests only (fast, no orchestration)
make test-unit
```

## Project Structure

```
binary/
├── backend/          # Rust/Axum API server
├── frontend/         # React/Vite app
├── contracts/        # Solidity contracts
│   ├── src/          # Contract source files
│   ├── test/         # Contract tests
│   ├── script/       # Forge deploy scripts
│   └── lib/          # Forge dependencies (forge-std, solady)
├── script/
│   ├── env.ts        # Local dev orchestrator
│   ├── env-testnet.ts # Testnet orchestrator (fault-tolerant)
│   ├── fund.ts       # Fund addresses with stablecoins
│   └── lib/          # Shared utilities (tempo, deploy, backend, etc.)
├── Makefile          # Make targets (wrappers around bun commands)
├── package.json      # Root package with unified commands
├── docker-compose.yml # Local Tempo node config
└── .testnet-config.json # Testnet contract addresses (generated)
```

## Commands

| Command | Description |
|---------|-------------|
| `make dev` / `bun run dev` | Start full local stack |
| `make test` / `bun run test` | Run integration tests |
| `make test-unit` | Run unit tests only |
| `make build` | Build for production |
| `bun run testnet` | Deploy and run full stack on testnet |
| `bun run testnet:backend` | Backend only on testnet (no frontend) |
| `bun run testnet:redeploy` | Force redeploy contracts on testnet |
| `bun run fund <addr> [amt] [-t token] [--chain <id>] [--faucet]` | Fund address with stablecoins (local or testnet) |

## Backend Development

- Admin API uses Bearer token authentication (`Authorization: Bearer <token>`)
- SQLite connection string for dev: `sqlite:./dev.db?mode=rwc`

### API Endpoints

| Endpoint | Method | Auth | Description |
|----------|--------|------|-------------|
| `/markets` | GET | None | List markets (paginated) |
| `/markets/:marketHash` | GET | None | Get market detail with orderbooks |
| `/markets/:marketHash/verse-tokens` | GET | None | Get verse tokens for market |
| `/admin/markets/open` | POST | Bearer | Create new market on-chain |
| `/admin/markets/add` | POST | Bearer | Add existing on-chain market to DB |

### Admin Request Example

```json
{
  "question": "Will ETH reach $5000?",
  "resolutionDeadline": 1767225600,
  "assets": ["0x20C0000000000000000000000000000000000000"]
}
```

## Frontend Development

- CSS uses custom utility classes (not Tailwind) - defined in `index.css`
- Light theme with white background, yellow accents (`--accent-yellow`), black borders
- JetBrains Mono font throughout (monospace, minimalist design)
- **RPC URL configuration**: Frontend uses `VITE_RPC_URL` env var (defaults to `http://localhost:9545`). The orchestrator passes this automatically when starting the frontend.
- **Component structure**: MarketList → MarketCard (expandable) → TradePanel (buy/sell/split)

## Dev Environment Orchestrator

The orchestrator at `script/env.ts` handles:
1. Starting Tempo node via docker-compose
2. Setting fee token to PathUSD
3. Deploying contracts via forge (`Deploy.s.sol`)
4. Starting backend server
5. Creating seed markets via admin API
6. Starting frontend dev server

Environment variables are passed to backend/frontend automatically.

## Testing

### Unit Tests (Fast, no external dependencies)
```bash
# Run backend unit tests
make test-unit
# Or directly
cd backend && cargo test --test unit_tests

# Run frontend unit tests
bun run test:unit
```

### Integration Tests (Requires orchestrator)
```bash
# Correct way
make test

# Or manually start env, then in another terminal:
RPC_URL=http://localhost:9545 SERVER_URL=http://localhost:3001 cargo test --test integration_test -- --ignored
```

### Frontend E2E Tests (Requires orchestrator + frontend)
```bash
# Runs Playwright tests with full stack
bun run test:e2e
```

## Contract Addresses

| Contract | Address |
|----------|---------|
| TIP-20 Factory | `0x20Fc000000000000000000000000000000000000` |
| PATH_USD | `0x20C0000000000000000000000000000000000000` |
| UniV2 Factory | Deployed dynamically (see `.univ2-config.json`) |
| UniV2 Router | Deployed dynamically (see `.univ2-config.json`) |

## Contract Deployment

- Local deployment uses `contracts/script/Deploy.s.sol` via the orchestrator (markets are seeded via admin API)
- Testnet deployment uses `bun run testnet` which runs `script/env-testnet.ts`

## Testnet Deployment

The testnet orchestrator (`script/env-testnet.ts`) provides a fault-tolerant deployment for Tempo testnet:

### Quick Start

```bash
# Set your private key
export PRIVATE_KEY=0x...

# Deploy and run full stack
bun run testnet

# Or backend only (for production deployment)
bun run testnet:backend
```

### Features

- **Auto-funding**: Funds deployer via faucet if balance is low
- **Idempotent**: Safe to run multiple times - checks if contracts already deployed
- **Persistent**: Contract addresses saved to `.testnet-config.json`, database persists at `testnet.db`
- **Fault-tolerant**: Backend can be restarted without losing data

### How It Works

1. Checks deployer balance, funds via faucet if needed
2. Checks if contracts are deployed at saved addresses (verifies bytecode exists)
3. Deploys contracts only if not already deployed (or `--redeploy` flag used)
4. Starts backend with persistent SQLite database
5. Seeds markets via admin API (idempotent - duplicates ignored)
6. Seeds UniV2 liquidity (pair creation is idempotent)
7. Starts frontend (unless `--skip-frontend`)

### Persistent Files

| File | Description |
|------|-------------|
| `.testnet-config.json` | Contract addresses, deployer info |
| `backend/testnet.db` | SQLite database for markets |

### Redeploying

To force redeploy contracts (creates new contract instances):

```bash
bun run testnet:redeploy
# or
rm .testnet-config.json && bun run testnet
```

### Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `PRIVATE_KEY` | Yes | - | Deployer private key |
| `FORGE_PATH` | No | `forge` | Path to tempo-foundry forge binary (required for chain 42431 support) |
| `DATABASE_URL` | No | `sqlite:./testnet.db?mode=rwc` | Backend database |
| `ADMIN_API_KEY_HASH` | No | Test hash | bcrypt hash for admin API |
| `PORT` | No | `3000` | Backend server port |

### Tempo-Foundry Requirements

The testnet deployment requires a tempo-foundry build with alloy-chains 0.2.25+ for Tempo Moderato (chain ID 42431) support. If you have tempo-foundry cloned locally:

```bash
# Update alloy-chains and rebuild
cd /path/to/tempo-foundry
cargo update alloy-chains
cargo build --release -p forge

# Then run testnet with FORGE_PATH
FORGE_PATH=/path/to/tempo-foundry/target/release/forge bun run testnet
```

## Tempo Node Gotchas

- **Fee token must be set before `forge script`**: The deployer account's fee token must be explicitly set to `PATH_USD` *before* broadcasting transactions. Without this, `forge` fails with "Insufficient liquidity for fee token" even if simulation passes.
- **TIP20Factory salt parameter**: The `ITIP20Factory.createToken` function requires a `bytes32 salt` parameter for deterministic token addresses. Update both `ITIP20Factory.sol` and `MultiVerse.sol` if the signature changes.
- **Updating Tempo docker image**: On ARM Macs, must use `docker pull --platform linux/amd64 ghcr.io/tempoxyz/tempo:latest` to pull the latest image, then `docker-compose down && docker-compose up -d` to restart.
- **Local devnet chain ID**: The local Tempo node uses chain ID `1337` (not `42431` like testnet).
- **viem tempoModerato chain not in published viem**: The `tempoModerato` chain (ID 42431) may not be in the published viem package. If so, inline the chain definition manually (see `script/lib/testnet.ts`).

## Tempo RPC & Cast Commands

- **Sending tokens with cast**: Use `cast erc20 transfer --fee-token <FEE_TOKEN> <TOKEN> <TO> <AMOUNT> --rpc-url <RPC> --private-key <KEY>`. The flag is `--fee-token`, NOT `--fee-payer`.
- **Local devnet fee token**: Use PATH_USD (`0x20C0...0000`) as fee token on local devnet. AlphaUSD may fail with "Insufficient liquidity for fee token" error.
- **Token decimals**: Stablecoins (PATH_USD, AlphaUSD, etc.) use 6 decimals. To send 1000 tokens, use `1000000000` (1000 * 10^6).
- **Testnet fee sponsorship (viem)**: Use `withFeePayer(http(), http('https://sponsor.moderato.tempo.xyz'))` transport for sponsored transactions.
- **Testnet faucet (viem)**: Call `client.faucet.fund({ account: address })` after extending client with `tempoActions()`. Funds 1M of each stablecoin.
- **Test private key**: `0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80` (address: `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266`) - prefunded on local devnet.

## UniswapV2 AMM

The prediction markets use UniswapV2 for token swaps and liquidity, replacing the previous Stablecoin DEX precompile.

### Deployed Contracts
- **UniV2 Factory**: Deployed via `DeployUniV2.s.sol`
- **UniV2 Router02**: Deployed via `DeployUniV2.s.sol`
- **WETH equivalent**: PATH_USD (`0x20C0...0000`) is used as the WETH address

### Key Differences from Stablecoin DEX
- **AMM vs Orderbook**: UniV2 uses constant-product AMM (`x*y=k`) instead of limit orderbook
- **Probability from reserves**: Price = `quote_reserve / base_reserve`, probability = `yes_price / (yes_price + no_price)`
- **Liquidity provision**: Use `addLiquidity()` instead of placing limit orders
- **No tick constraints**: Full price range supported (unlike DEX's ±2% tick range)

### Scripts
- `script/lib/univ2/` - UniV2 deployment, pair creation, liquidity, and swap functions
- `createPair(tokenA, tokenB)` - Create trading pair
- `addLiquidity({ tokenA, tokenB, amountA, amountB })` - Provide liquidity
- `swapExactTokensForTokens(...)` - Execute swaps

### Environment Variables
- `VITE_UNIV2_ROUTER_ADDRESS` - Frontend UniV2 Router address
- `VITE_UNIV2_FACTORY_ADDRESS` - Frontend UniV2 Factory address  
- `UNIV2_FACTORY_ADDRESS` - Backend UniV2 Factory address

### Implementation Notes
- **UniV2 config file**: Deployed addresses are persisted to `.univ2-config.json` for idempotent redeployment. Delete this file to force redeploy.
- **Deploying UniV2 with forge**: Uses `vm.getCode()` to load bytecode from artifacts since Factory (0.5.16) and Router (0.6.6) have incompatible Solidity versions.
- **UniV2 Router deadline**: Always pass a deadline parameter (e.g., `BigInt(Math.floor(Date.now() / 1000) + 1800)` for 30 min).
- **50/50 probability seeding**: Add equal liquidity to YES/PATH_USD and NO/PATH_USD pairs (1:1 ratio) for initial 50/50 market probability.
