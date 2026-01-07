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
│   ├── env.ts        # Central orchestrator (main entry point)
│   ├── fund.ts       # Fund addresses with stablecoins
│   ├── deploy-tempo.ts # Testnet deploy script
│   └── lib/          # Shared utilities
├── Makefile          # Make targets (wrappers around bun commands)
├── package.json      # Root package with unified commands
└── docker-compose.yml # Tempo node config
```

## Commands

| Command | Description |
|---------|-------------|
| `make dev` / `bun run dev` | Start full local stack |
| `make test` / `bun run test` | Run integration tests |
| `make test-unit` | Run unit tests only |
| `make build` | Build for production |
| `bun run deploy:testnet` | Deploy to Tempo testnet |
| `bun run fund <addr> [amt] [-t token]` | Fund address with stablecoins on local devnet |

## Backend Development

- Admin API uses Bearer token authentication (`Authorization: Bearer <token>`)
- SQLite connection string for dev: `sqlite:./dev.db?mode=rwc`

### API Endpoints

| Endpoint | Method | Auth | Description |
|----------|--------|------|-------------|
| `/markets` | GET | None | List markets (paginated) |
| `/markets/:marketHash` | GET | None | Get market detail with orderbooks |
| `/markets/:marketHash/verse-tokens` | GET | None | Get verse tokens for market |
| `/admin/markets/open` | POST | Bearer | Create new market |

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
3. Deploying contracts via forge
4. Starting backend server
5. Starting frontend dev server

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

- Local deployment uses `contracts/script/DeployAndSeedMarkets.s.sol` via the orchestrator
- Testnet deployment uses `bun run deploy:testnet` which runs `script/deploy-tempo.ts`

## Tempo Node Gotchas

- **Fee token must be set before `forge script`**: The deployer account's fee token must be explicitly set to `PATH_USD` *before* broadcasting transactions. Without this, `forge` fails with "Insufficient liquidity for fee token" even if simulation passes.
- **TIP20Factory salt parameter**: The `ITIP20Factory.createToken` function requires a `bytes32 salt` parameter for deterministic token addresses. Update both `ITIP20Factory.sol` and `MultiVerse.sol` if the signature changes.
- **Updating Tempo docker image**: On ARM Macs, must use `docker pull --platform linux/amd64 ghcr.io/tempoxyz/tempo:latest` to pull the latest image, then `docker-compose down && docker-compose up -d` to restart.
- **Local devnet chain ID**: The local Tempo node uses chain ID `1337` (not `42429` like testnet).

## Tempo RPC & Cast Commands

- **Sending tokens with cast**: Use `cast erc20 transfer --fee-token <FEE_TOKEN> <TOKEN> <TO> <AMOUNT> --rpc-url <RPC> --private-key <KEY>`. The flag is `--fee-token`, NOT `--fee-payer`.
- **Local devnet fee token**: Use PATH_USD (`0x20C0...0000`) as fee token on local devnet. AlphaUSD may fail with "Insufficient liquidity for fee token" error.
- **Token decimals**: Stablecoins (PATH_USD, AlphaUSD, etc.) use 6 decimals. To send 1000 tokens, use `1000000000` (1000 * 10^6).
- **Testnet fee sponsorship (viem)**: Use `withFeePayer(http(), http('https://sponsor.testnet.tempo.xyz'))` transport for sponsored transactions.
- **Testnet faucet (viem)**: Call `client.faucet.fund({ account: address })` after extending client with `tempoActions()`. Funds 1M of each stablecoin.
- **Test private key**: `0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80` (address: `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266`) - prefunded on local devnet.
