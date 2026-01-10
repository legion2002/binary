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
| Stablecoin DEX | `0xDEc0000000000000000000000000000000000000` |

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
6. Seeds DEX liquidity (pair creation is idempotent)
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

## Stablecoin DEX Gotchas

- **Minimum order size**: The Stablecoin DEX requires a minimum order size of 100 tokens (100_000_000 with 6 decimals). Orders below this fail with `BelowMinimumOrderSize` error (`0x46628371`).
- **DEX tick range**: The DEX only supports ±2% from peg (tick range ±2000), meaning prices from $0.98 to $1.02. This limits prediction market probability ranges to ~49-51% when using mid-prices.
- **Use viem/tempo native actions**: Prefer `client.token.approve()`, `client.dex.placeSync()`, `client.dex.createPair()` over raw `writeContract()` calls. Raw calls may fail with `InsufficientAllowance` (`0x13be252b`) even with correct approvals.
- **Common DEX error signatures**: `0x46628371` = `BelowMinimumOrderSize(uint128)`, `0x13be252b` = `InsufficientAllowance()`, `0xbb55fd27` = `InsufficientLiquidity()`.
- **DEX pairs must exist before trading**: Operations like `dex.placeSync()` fail with `PairDoesNotExist` error if the token pair hasn't been created first. Use `client.dex.createPair({ base: token, feeToken: PATH_USD })` before placing orders. Pair creation is idempotent (ignores `PairAlreadyExists`).
