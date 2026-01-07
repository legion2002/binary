# Frontend Agent Guidelines

## Commands

```bash
bun run build   # Build for production
bun run dev     # Start dev server
bun run lint    # Run ESLint
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

### WebAuthn Connection

When using the `webAuthn` connector, pass `capabilities` to control sign-up vs sign-in behavior:

```typescript
// Sign up (create new passkey)
connect({
  connector: webAuthnConnector,
  capabilities: { type: 'sign-up', label: 'My App' }
});

// Sign in (use existing passkey, allow selection if multiple)
connect({
  connector: webAuthnConnector,
  capabilities: { type: 'sign-in', selectAccount: true }
});
```

- `sign-up`: Creates a new passkey credential
- `sign-in` with `selectAccount: true`: Prompts user to choose from available passkeys
- Without `selectAccount`, it uses the last active credential automatically
- Error `publicKey not found` means localStorage was cleared but browser still has the passkey - prompt user to sign up again

## Tempo Transaction Features

These native Tempo features enable superior UX compared to traditional EVM:

- **Call batching**: `sendTransaction({ calls: [...] })` for atomic multi-call operations (e.g., approve + swap in one tx)
- **Fee sponsorship**: Gasless transactions via `feePayer: true` - app pays gas for users
- **2D nonces**: `nonceKey` param enables parallel transaction submission without blocking
- **Access keys**: Session keys with spending limits to avoid repeated passkey prompts
