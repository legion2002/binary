---
name: testnet-deploy
description: Deploys Binary Markets to Tempo Moderato testnet. Use when asked to deploy, redeploy, or reset the testnet environment, contracts, backend, or database.
---

# Testnet Deployment

Deploys and manages Binary Markets on Tempo Moderato testnet (chain ID 42431).

## Quick Reference

| Resource | URL |
|----------|-----|
| RPC | `https://rpc.moderato.tempo.xyz` |
| Fee Sponsor | `https://sponsor.moderato.tempo.xyz` |
| Backend API | `https://binary-markets-api.fly.dev` |
| Frontend | Cloudflare Pages (`binary-markets`) |
| Chain ID | `42431` |

## Full Redeploy from Scratch

Complete steps to redeploy everything and reset the database:

### Step 1: Deploy Contracts

```bash
# Ensure private key is set
source .env  # or export PRIVATE_KEY=0x...

# Deploy MultiVerse + Oracle
cd contracts && forge script script/Deploy.s.sol:Deploy \
  --rpc-url https://rpc.moderato.tempo.xyz \
  --chain 42431 --broadcast --private-key "$PRIVATE_KEY" \
  --skip-simulation --slow -vvv

# Deploy UniswapV2 Factory + Router
forge script script/DeployUniV2.s.sol:DeployUniV2 \
  --rpc-url https://rpc.moderato.tempo.xyz \
  --chain 42431 --broadcast --private-key "$PRIVATE_KEY" \
  --skip-simulation --slow -vvv

# Update deployments.json from broadcast files
cd .. && bun run update-deployments
```

### Step 2: Redeploy Backend

```bash
# Backend reads addresses from deployments.json (not env vars)
# If migrating from env vars, remove old secrets:
fly secrets unset MULTIVERSE_ADDRESS ORACLE_ADDRESS UNIV2_FACTORY_ADDRESS --app binary-markets-api

# Deploy backend (copies deployments.json into container)
cd backend && fly deploy --now
```

### Step 3: Reset Database

```bash
# Delete existing database
fly ssh console --app binary-markets-api -C "rm /data/markets.db"

# Restart to recreate empty database
fly apps restart binary-markets-api

# Verify backend is running
curl -s https://binary-markets-api.fly.dev/config | jq .
```

### Step 4: Seed Markets

```bash
cd script && bun --eval "
import { createSeedMarkets } from './lib/seed-markets'
const result = await createSeedMarkets({
  backendUrl: 'https://binary-markets-api.fly.dev',
  verbose: true
})
console.log('Created:', result.count, 'markets')
"
```

### Step 5: Add Liquidity

```bash
# Adds routing liquidity (ALPHA_USD/PATH_USD) + market liquidity (YES/NO pairs)
bun run script/add-testnet-liquidity.ts --add
```

### Step 6: Update Frontend

Set environment variables in Cloudflare Pages dashboard (Settings > Environment Variables):

| Variable | Value (from deployments.json) |
|----------|-------------------------------|
| `VITE_RPC_URL` | `https://rpc.moderato.tempo.xyz` |
| `VITE_API_URL` | `https://binary-markets-api.fly.dev` |
| `VITE_FEE_PAYER_URL` | `https://sponsor.moderato.tempo.xyz` |
| `VITE_UNIV2_FACTORY_ADDRESS` | (chain 42431 uniV2Factory) |
| `VITE_UNIV2_ROUTER_ADDRESS` | (chain 42431 uniV2Router) |

Then redeploy:
```bash
bun run deploy:frontend
```

## Partial Operations

### Deploy Only UniV2 Contracts

```bash
source .env
cd contracts && forge script script/DeployUniV2.s.sol:DeployUniV2 \
  --rpc-url https://rpc.moderato.tempo.xyz \
  --chain 42431 --broadcast --private-key "$PRIVATE_KEY" \
  --skip-simulation --slow -vvv
cd .. && bun run update-deployments
```

### Reset Only Database

```bash
fly ssh console --app binary-markets-api -C "rm /data/markets.db"
fly apps restart binary-markets-api
```

### Add Only Liquidity

```bash
bun run script/add-testnet-liquidity.ts --add
```

### Check Current State

```bash
# Backend config
curl -s https://binary-markets-api.fly.dev/config | jq .

# Markets
curl -s https://binary-markets-api.fly.dev/markets | jq .

# Deployments
cat deployments.json | jq '.["42431"]'
```

## Important Files

| File | Purpose |
|------|---------|
| `deployments.json` | All contract addresses by chain ID (source of truth) |
| `backend/deployments.json` | Copy for Docker build |
| `frontend/src/deployments.json` | Copy for frontend build |
| `contracts/broadcast/*/42431/run-latest.json` | Forge broadcast logs |

## Updating Contract Addresses

After deploying contracts, always run:
```bash
bun run update-deployments
```

This parses broadcast files and updates all copies of deployments.json.

## Gotchas

- **Chain ID mismatch**: `tempoTestnet` from wagmi defaults to Andantino (42429). Frontend must override to `42431` for Moderato.
- **Transaction sequencing**: Must `waitForTransactionReceipt` between approve and addLiquidity calls.
- **Verse token matching**: ALPHA_USD and PATH_USD have different verse addresses for the same market. Use ALPHA_USD's verses when splitting ALPHA_USD.
- **Routing liquidity**: The ALPHA_USD/PATH_USD pair must have liquidity for multi-hop swaps to work.
- **UniV2 broadcast parsing**: Factory and Router have null `contractName` in broadcasts. Parse by order: Factory first, Router second.
