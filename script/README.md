# Binary Protocol Environment Orchestrator

This directory contains TypeScript scripts for managing the Binary Protocol development and testing environment.

## Quick Start

```bash
# Start full development environment (Tempo + contracts + backend + frontend)
make env-dev

# Run integration tests
make env-test
```

## Prerequisites

1. **Docker** - Required for running the local Tempo node
2. **Bun** - TypeScript runtime for the orchestrator scripts
3. **tempo-foundry** - Tempo's fork of Foundry with fee token support
4. **Rust/Cargo** - For building the backend

### Installing tempo-foundry

```bash
# Install foundryup if not already installed
curl -L https://foundry.paradigm.xyz | bash

# Install tempo-foundry
foundryup -n tempo

# Verify installation
forge --version  # Should show "tempo" in version
```

## Commands

### Development Mode

```bash
bun run script/env.ts dev
```

This starts:
1. Local Tempo node (Docker) at `http://localhost:9545`
2. Deploys contracts (MultiVerse, TrustedOracle)
3. Starts backend server at `http://localhost:3000`
4. Starts frontend dev server at `http://localhost:5173`

Press `Ctrl+C` to stop all services.

### Test Mode

```bash
bun run script/env.ts test
```

This starts the environment and runs the backend integration tests.

### Custom Test Commands

```bash
# Run specific cargo tests
bun run script/env.ts test -- cargo test --test integration_test -- --ignored

# Run e2e tests with frontend
bun run script/env.ts test --with-frontend -- pnpm --prefix frontend test:e2e
```

## Architecture

```
script/
├── env.ts              # Main orchestrator entry point
├── lib/
│   ├── tempo.ts        # Tempo node management
│   ├── deploy.ts       # Contract deployment
│   ├── backend.ts      # Backend server management
│   ├── frontend.ts     # Frontend dev server management
│   ├── fee-amm.ts      # Fee token configuration
│   └── health.ts       # Health check utilities
├── package.json        # Script dependencies
└── tsconfig.json       # TypeScript config
```

## How It Works

### Fee Token Configuration

The local Tempo dev node uses chain ID 1337 with the Tempo fee system. Before deploying contracts:

1. **Set Fee Token Preference**: The orchestrator calls `setUserToken(PathUSD)` on the FeeManager precompile
2. **PathUSD is the base token**: Using PathUSD means no Fee AMM swap is required (the validator accepts PathUSD directly)
3. **Subsequent transactions**: All transactions from this account will use PathUSD for gas fees

This is necessary because:
- AlphaUSD (the default) requires a Fee AMM swap to PathUSD
- The local dev node doesn't have Fee AMM liquidity pre-seeded
- PathUSD doesn't require a swap (it's the base token)

### Docker Compose

The Tempo node is managed via Docker Compose. Key configuration in `docker-compose.yml`:

```yaml
command:
  - node
  - --dev                              # Dev mode
  - --dev.block-time=100ms             # Fast blocks
  - --faucet.enabled                   # Enable faucet
  - --faucet.node-address=http://localhost:9545  # Faucet RPC
```

### Environment Variables

The orchestrator sets these environment variables for the backend:

| Variable | Description |
|----------|-------------|
| `RPC_URL` | Tempo node RPC URL |
| `MULTIVERSE_ADDRESS` | Deployed MultiVerse contract address |
| `ORACLE_ADDRESS` | Deployed TrustedOracle address |
| `ADMIN_API_KEY_HASH` | Bcrypt hash for API authentication |
| `DATABASE_URL` | SQLite database path |

And for tests:

| Variable | Description |
|----------|-------------|
| `SERVER_URL` | Backend server URL |
| `ADMIN_API_KEY` | Plain text API key for tests |

## Troubleshooting

### "Insufficient liquidity for fee token"

This error occurs when the Fee AMM doesn't have liquidity for the token swap. Solution:
- Ensure `setUserFeeToken` is called before deploying contracts
- Use PathUSD (not AlphaUSD) as the fee token

### "failed to decode signed transaction"

This happens when using `--fee-token` flag with tempo-foundry on a chain ID that doesn't support Tempo transaction types. The orchestrator avoids this by:
1. Setting fee token preference at the account level
2. Using standard EVM transactions (no `--fee-token` flag)

### Backend won't start

Check that:
1. Tempo node is running: `curl http://localhost:9545`
2. Contracts are deployed: Check broadcast files in `broadcast/`
3. Port 3000 (or 3001 for tests) is available

### Tests fail with "Invalid API key"

The bcrypt hash in `lib/backend.ts` must match the `TEST_API_KEY`. Regenerate with:

```bash
bun -e "console.log(await Bun.password.hash('test_api_key_12345', { algorithm: 'bcrypt', cost: 10 }))"
```
