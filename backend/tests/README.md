# Integration Tests

This directory contains end-to-end integration tests for the Binary backend server.

## Running Tests

Integration tests require the full local environment (Tempo node + contracts). Use the orchestrator:

```bash
# From project root
make test

# Or with bun
bun run test
```

The orchestrator will:
1. Start a local Tempo node via Docker
2. Deploy contracts (MultiVerse, TrustedOracle)
3. Start the backend server
4. Run `cargo test --test integration_test -- --ignored`
5. Shut down all services

### Run specific tests:

```bash
bun run script/env.ts test -- cargo test --test integration_test test_full_flow -- --ignored
```

### Run with output:

```bash
bun run script/env.ts test -- cargo test --test integration_test -- --ignored --nocapture
```

## What the Tests Do

### `test_full_flow`

1. Uses the orchestrator-started Tempo node and backend
2. Tests the following functionality:
   - Fetching empty markets list
   - Opening a new market via admin API
   - Verifying the market was indexed
   - Opening a second market
   - Fetching all markets (should return 2)
   - Testing authentication (unauthorized access)

### `test_market_with_verse_tokens`

1. Same setup as `test_full_flow`
2. Opens a market with verse token creation
3. Verifies verse tokens (YES/NO) are created successfully

## Test Configuration

When run via the orchestrator, tests receive these environment variables:

| Variable | Value |
|----------|-------|
| `RPC_URL` | `http://localhost:9545` |
| `SERVER_URL` | `http://localhost:3001` |
| `ADMIN_API_KEY` | `test_api_key_12345` |
| `MULTIVERSE_ADDRESS` | (deployed address) |
| `ORACLE_ADDRESS` | (deployed address) |

## Running Tests Manually

If you have the environment already running (via `make dev`):

```bash
cd backend
RPC_URL=http://localhost:9545 \
SERVER_URL=http://localhost:3001 \
ADMIN_API_KEY=test_api_key_12345 \
cargo test --test integration_test -- --ignored
```

## Quick Manual API Test

Test the admin API against a running server:

```bash
curl -X POST http://127.0.0.1:3000/admin/markets/open \
  -H 'Authorization: Bearer test_api_key_12345' \
  -H 'Content-Type: application/json' \
  -d '{
    "question": "Will Ethereum reach $10k by end of 2025?",
    "resolutionDeadline": 1767225600
  }'
```

**Fetch all markets:**
```bash
curl http://127.0.0.1:3000/markets
```

## Troubleshooting

### "Docker not running"
Start Docker Desktop or the Docker daemon.

### "Contract deployment failed"
Make sure contracts compile: `cd contracts && forge build`

### "Server failed to start"
Check that ports 3000/3001 and 9545 are available:
```bash
lsof -i :3000
lsof -i :9545
```

### Tests timeout
The backend compilation can take time on first run. The orchestrator waits up to 120 seconds.
