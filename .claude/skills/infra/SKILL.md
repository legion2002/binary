---
description: Deploy and manage Binary Markets infrastructure (Fly.io backend, Cloudflare Pages frontend, production seeding)
triggers:
  - deploy
  - infrastructure
  - fly.io
  - cloudflare
  - production
---

# Binary Markets Infrastructure

This skill covers deploying and managing the Binary Markets production infrastructure.

## Architecture Overview

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  Cloudflare     │     │    Fly.io        │     │  Tempo Testnet  │
│  Pages          │────▶│    Backend       │────▶│  (Moderato)     │
│  (Frontend)     │     │    (Rust/Axum)   │     │                 │
└─────────────────┘     └──────────────────┘     └─────────────────┘
```

| Component | Platform | URL |
|-----------|----------|-----|
| Frontend | Cloudflare Pages | `https://binary-markets.pages.dev` |
| Backend | Fly.io | `https://binary-markets-api.fly.dev` |
| RPC | Tempo Testnet | `https://rpc.moderato.tempo.xyz` |
| Fee Sponsor | Tempo | `https://sponsor.moderato.tempo.xyz` |

## Quick Commands

| Command | Description |
|---------|-------------|
| `bun run production:seed` | Seed deployed backend with markets + liquidity |
| `cd backend && fly deploy` | Deploy backend to Fly.io |
| `cd frontend && bun run build && npx wrangler pages deploy dist` | Deploy frontend |
| `fly logs -a binary-markets-api` | Tail backend logs |
| `fly status -a binary-markets-api` | Check backend status |
| `fly secrets list -a binary-markets-api` | List configured secrets |

---

## Backend Deployment (Fly.io)

### First-Time Setup

```bash
# Install flyctl
curl -L https://fly.io/install.sh | sh

# Login
fly auth login

# Create app (from backend directory)
cd backend
fly launch --no-deploy --name binary-markets-api --region iad

# Create persistent volume for SQLite
fly volumes create data --size 1 --region iad

# Set secrets (get contract addresses from .testnet-config.json)
fly secrets set \
  PRIVATE_KEY="0x..." \
  ADMIN_API_KEY_HASH='$2b$10$pFkqqOn5jFfiIPFGLoU9/uGIF8.QDon0ZOE9ZZ6WxrmSEypkTpI0O' \
  MULTIVERSE_ADDRESS="0x..." \
  ORACLE_ADDRESS="0x..."

# Deploy
fly deploy
```

### Required Secrets

| Secret | Description | How to Get |
|--------|-------------|------------|
| `PRIVATE_KEY` | Deployer private key | Your wallet |
| `ADMIN_API_KEY_HASH` | bcrypt hash for admin API | `bun -e "console.log(await Bun.password.hash('your-key', {algorithm:'bcrypt',cost:10}))"` |
| `MULTIVERSE_ADDRESS` | MultiVerse contract | `cat .testnet-config.json \| jq -r '.multiverse'` |
| `ORACLE_ADDRESS` | Oracle contract | `cat .testnet-config.json \| jq -r '.oracle'` |

### Environment Variables (in fly.toml)

These are set in `backend/fly.toml` and don't need to be secrets:

| Variable | Value |
|----------|-------|
| `HOST` | `0.0.0.0` |
| `PORT` | `3000` |
| `DATABASE_URL` | `sqlite:///data/markets.db?mode=rwc` |
| `RPC_URL` | `https://rpc.moderato.tempo.xyz` |
| `RUST_LOG` | `info,backend=debug` |

### Subsequent Deploys

```bash
cd backend
fly deploy
```

### Useful Commands

```bash
# View logs
fly logs -a binary-markets-api

# SSH into machine
fly ssh console -a binary-markets-api

# Check status
fly status -a binary-markets-api

# Scale to 2 machines
fly scale count 2 -a binary-markets-api

# View secrets
fly secrets list -a binary-markets-api

# Update a secret
fly secrets set PRIVATE_KEY="0xnew..." -a binary-markets-api
```

---

## Frontend Deployment (Cloudflare Pages)

### First-Time Setup

```bash
# Install wrangler
bun add -g wrangler

# Login
npx wrangler login

# Build and deploy (from frontend directory)
cd frontend
bun run build
npx wrangler pages project create binary-markets
npx wrangler pages deploy dist
```

### Environment Variables

Set in Cloudflare Dashboard → Pages → binary-markets → Settings → Environment variables:

| Variable | Production Value |
|----------|------------------|
| `VITE_RPC_URL` | `https://rpc.moderato.tempo.xyz` |
| `VITE_API_URL` | `https://binary-markets-api.fly.dev` |
| `VITE_FEE_PAYER_URL` | `https://sponsor.moderato.tempo.xyz` |

### Subsequent Deploys

```bash
cd frontend
bun run build
npx wrangler pages deploy dist
```

Or use the root command:
```bash
bun run deploy:frontend
```

---

## Production Seeding

After deploying backend and frontend, seed the database with markets and liquidity:

```bash
PRIVATE_KEY=0x... bun run production:seed
```

This script (`script/seed-production.ts`):
1. Checks backend health
2. Creates seed markets via admin API
3. Sets up stablecoin routing on DEX
4. Seeds liquidity for all market verse tokens

### Custom Backend URL

```bash
PRIVATE_KEY=0x... BACKEND_URL=https://custom-backend.fly.dev bun run production:seed
```

---

## CI/CD (GitHub Actions)

The workflow at `.github/workflows/deploy.yml` auto-deploys on push to `main`.

### Required GitHub Secrets

| Secret | How to Get |
|--------|------------|
| `FLY_API_TOKEN` | `fly tokens create deploy -x 999999h` |
| `CLOUDFLARE_API_TOKEN` | Cloudflare Dashboard → API Tokens → Create Token → Edit Cloudflare Pages |
| `CLOUDFLARE_ACCOUNT_ID` | From dashboard URL: `dash.cloudflare.com/<ACCOUNT_ID>/...` |

---

## Troubleshooting

### Backend Build Fails: "edition2024 is required"

The `alloy` crate requires Rust 1.85+. Update `backend/Dockerfile`:
```dockerfile
FROM rust:1.87-slim-bookworm AS builder
```

### Backend Build Fails: "file not found for module bindings"

The `.dockerignore` is excluding `bindings/`. Remove this line from `backend/.dockerignore`:
```
**/bindings
```

### Production Seed: 429 Rate Limit Errors

Tempo testnet RPC rate-limits requests. The `seed-production.ts` script includes 2-second delays between operations. If still failing, increase `RATE_LIMIT_DELAY` in the script.

### Frontend Not Picking Up Env Vars

Vite bakes env vars at build time. After changing Cloudflare env vars:
1. Trigger a new build, OR
2. Rebuild locally with env vars and redeploy:
   ```bash
   VITE_API_URL=https://... bun run build
   npx wrangler pages deploy dist
   ```

### Backend Database Issues

SSH into the machine and check the database:
```bash
fly ssh console -a binary-markets-api
ls -la /data/
sqlite3 /data/markets.db ".tables"
```

To reset the database, delete and redeploy:
```bash
fly ssh console -a binary-markets-api -C "rm /data/markets.db"
fly deploy
```

---

Change region during setup:
```bash
fly launch --no-deploy --name binary-markets-api --region iad
fly volumes create data --size 1 --region iad
```
