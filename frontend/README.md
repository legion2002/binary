# Binary Markets Frontend

React + TypeScript + Vite frontend for Binary prediction markets.

## Quick Start

```bash
# Install dependencies
bun install

# Start dev server
bun run dev

# Build for production
bun run build

# Run linter
bun run lint
```

## Development

The frontend connects to:
- **Backend API**: `http://localhost:3000` (configurable via `VITE_BACKEND_URL`)
- **Tempo RPC**: `http://localhost:9545` (configurable via `VITE_RPC_URL`)

For local development, use the orchestrator from the project root:

```bash
# From project root
make dev
```

This starts the Tempo node, deploys contracts, starts the backend, and the frontend together.

## Stack

- **React 19** with TypeScript
- **Vite 7** for bundling
- **Wagmi 3** for wallet connections with native Tempo support
- **TanStack Query** for data fetching
- **Custom CSS** (not Tailwind) with dark theme

## Tempo Integration

The frontend uses Wagmi's native Tempo support:

```typescript
import { webAuthn, KeyManager } from "wagmi/tempo";
import { Hooks } from "wagmi/tempo";

// WebAuthn passkey connector
webAuthn({ keyManager: KeyManager.localStorage() })

// Tempo DEX hooks
Hooks.dex.useBuy()
Hooks.dex.useSell()

// Token hooks
Hooks.token.useGetBalance()
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `VITE_RPC_URL` | `http://localhost:9545` | Tempo RPC endpoint |
| `VITE_BACKEND_URL` | `http://localhost:3000` | Backend API endpoint |
| `VITE_FEE_PAYER_URL` | (auto) | Fee sponsor URL for gasless transactions |
