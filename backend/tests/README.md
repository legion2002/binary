# Integration Tests

This directory contains end-to-end integration tests for the Binary backend server.

## Prerequisites

Before running the integration tests, make sure you have:

1. **Anvil** (from Foundry) installed and in your PATH
2. **Forge** (from Foundry) installed
3. **Contracts** deployed via the DeployAll script at `../script/DeployAll.s.sol`

## Running the Tests

The integration tests are marked with `#[ignore]` to prevent them from running during normal `cargo test`.

### Run all integration tests:

```bash
cargo test --test integration_test -- --ignored
```

### Run a specific integration test:

```bash
cargo test --test integration_test test_full_flow -- --ignored
```

### Run with output:

```bash
cargo test --test integration_test -- --ignored --nocapture
```

## What the Tests Do

### `test_full_flow`

1. Starts a local Anvil testnet on port 8545
2. Deploys MultiVerse and TrustedOracle contracts
3. Starts the backend server on port 3001 with in-memory SQLite
4. Tests the following functionality:
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

The tests use the following configuration:

- **Anvil RPC**: `http://127.0.0.1:8545`
- **Server Port**: `3001` (to avoid conflict with dev server on 3000)
- **Database**: In-memory SQLite (`:memory:`)
- **Private Key**: Anvil's default first account key
- **API Key**: Test key `test_api_key_12345`

## Cleaning Up

The test environment automatically cleans up by:
- Killing Anvil process on test completion
- Killing server process on test completion
- Using in-memory database (no cleanup needed)

## Quick Manual Test (Production/Testnet)

If you want to manually test the admin API against a running server:

```bash
curl -X POST http://127.0.0.1:3000/admin/markets/open \
  -H 'Authorization: Bearer YOUR_API_KEY_HERE' \
  -H 'Content-Type: application/json' \
  -d '{
    "question": "Will Ethereum reach $10k by end of 2025?",
    "resolutionDeadline": 1767225600,
    "assets": ["0x4200000000000000000000000000000000000006"]
  }'
```

Replace `YOUR_API_KEY_HERE` with your actual API key (generated via `cargo run --bin generate_api_key`).

**Expected successful response:**
```json
{
  "marketHash": "0x...",
  "questionHash": "0x...",
  "question": "Will Ethereum reach $10k by end of 2025?",
  "resolutionDeadline": 1767225600,
  "oracle": "0x...",
  "transactionHash": "0x...",
  "verseTokens": [...]
}
```

**Fetch all markets:**
```bash
curl http://127.0.0.1:3000/markets
```

## Troubleshooting

### "Anvil not found"
Install Foundry: `curl -L https://foundry.paradigm.xyz | bash && foundryup`

### "Contract deployment failed"
Make sure the contracts compile: `cd .. && forge build`

### "Server failed to start"
Check that port 3001 is available: `lsof -i :3001`

### Tests hang
Make sure no other Anvil instance is running on port 8545
