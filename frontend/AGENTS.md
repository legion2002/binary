# Frontend Agent Guidelines

## Commands

```bash
pnpm run build   # Build for production
pnpm run dev     # Start dev server
pnpm run lint    # Run ESLint
```

## Wagmi/Tempo API

Native Tempo support is now built into wagmi. Import from `wagmi/tempo`:

- `webAuthn` connector - WebAuthn passkey-based account connector
- `KeyManager.localStorage()` - Store passkey credentials in localStorage
- `Hooks.dex.*` - DEX trading hooks (useBuy, useSell, useBuyQuote, useSellQuote)
- `Hooks.token.*` - Token hooks (useBalance, useMetadata, useTransfer, useApprove)
- `tempoTestnet` chain from `wagmi/chains`

### Wagmi Config Setup

```typescript
import { createConfig, http } from "wagmi";
import { tempoTestnet } from "wagmi/chains";
import { webAuthn, KeyManager } from "wagmi/tempo";

export const config = createConfig({
  chains: [tempoTestnet],
  connectors: [
    webAuthn({
      keyManager: KeyManager.localStorage(),
    }),
  ],
  transports: {
    [tempoTestnet.id]: http(),
  },
});
```

### Using Tempo Hooks

```typescript
import { Hooks } from "wagmi/tempo";

// Token balance (note: useGetBalance, not useBalance)
const { data } = Hooks.token.useGetBalance({ account, token });

// DEX trading
const { data: quote } = Hooks.dex.useBuyQuote({ tokenIn, tokenOut, amountOut });
const { mutate: buy } = Hooks.dex.useBuy();
```

## Architecture

- **WagmiProvider** wraps the app with wagmi config
- **webAuthn connector** manages WebAuthn/passkey accounts natively
- Use `useAccount`, `useConnect`, `useDisconnect` from wagmi for wallet state
- Fee sponsorship configured via `feePayer: true` in transaction params

## Tempo Transaction Features

These native Tempo features enable superior UX compared to traditional EVM:

- **Call batching**: `sendTransaction({ calls: [...] })` for atomic multi-call operations (e.g., approve + swap in one tx)
- **Fee sponsorship**: Gasless transactions via `feePayer: true` - app pays gas for users
- **2D nonces**: `nonceKey` param enables parallel transaction submission without blocking
- **Access keys**: Session keys with spending limits to avoid repeated passkey prompts
