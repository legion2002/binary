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

## Backend Development

- Admin API uses Bearer token authentication (`Authorization: Bearer <token>`)
- Admin endpoint: `POST /admin/markets/open` with JSON body `{ "question": "...", "resolutionDeadline": <unix_timestamp> }`
- SQLite connection string for dev: `sqlite:./dev.db?mode=rwc`

## Frontend Development

- CSS uses custom utility classes (not Tailwind) - defined in `index.css`
- Dark theme CSS variables in `:root` - `--bg-primary`, `--accent-purple`, etc.
- Google Fonts: DM Sans (body), JetBrains Mono (code)
- **RPC URL configuration**: Frontend uses `VITE_RPC_URL` env var (defaults to `http://localhost:9545`). The orchestrator passes this automatically when starting the frontend.

## Dev Environment Orchestrator

The orchestrator at `script/env.ts` handles:
1. Starting Tempo node via docker-compose
2. Setting fee token to PathUSD
3. Deploying contracts via forge
4. Starting backend server
5. Starting frontend dev server

Environment variables are passed to backend/frontend automatically.

## Testing

Integration tests require the orchestrator to be running:
```bash
# Correct way
make test

# Or manually start env, then in another terminal:
RPC_URL=http://localhost:9545 SERVER_URL=http://localhost:3001 cargo test --test integration_test -- --ignored
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
