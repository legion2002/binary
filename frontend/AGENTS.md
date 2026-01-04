# Frontend Agent Guidelines

## Commands

```bash
pnpm run build   # Build for production
pnpm run dev     # Start dev server
pnpm run lint    # Run ESLint
```

## Viem/Tempo API

When using `viem/tempo` for Tempo blockchain integration:

- `WebAuthnP256.createCredential({ label: "..." })` - uses `label` not `name`
- `Account.fromWebAuthnP256(credential)` - takes credential object directly, not `{ credential }`
- `withFeePayer(defaultTransport, relayTransport)` - positional args, not an object
- `tempoActions()` - call with no args, returns decorator that adds `.dex`, `.token`, `.amm` methods to clients
- **DEX Actions**: Use `Actions.dex.buy.call()`, `Actions.dex.sell.call()`, `Actions.dex.place.call()` for orderbook trading
- **DEX Address**: Import `Addresses` from `viem/tempo` - use `Addresses.stablecoinExchange` for DEX precompile
- **Swap pattern**: Always batch approve + swap: `[Actions.token.approve.call({...}), Actions.dex.buy.call({...})]`

## Architecture

- **PasskeyContext** manages WebAuthn accounts and Tempo clients (no wagmi connectors needed for passkeys)
- Use `client.extend(tempoActions())` to get decorated client with Tempo-specific methods
- Fee sponsorship configured via `withFeePayer` transport - testnet sponsor at `https://sponsor.testnet.tempo.xyz`
- Passkey credentials stored in localStorage, public keys should be stored remotely for production

## Tempo Transaction Features

These native Tempo features enable superior UX compared to traditional EVM:

- **Call batching**: `sendTransaction({ calls: [...] })` for atomic multi-call operations (e.g., approve + swap in one tx)
- **Fee sponsorship**: Gasless transactions via `withFeePayer` transport - app pays gas for users
- **2D nonces**: `nonceKey` param enables parallel transaction submission without blocking
- **Access keys**: Session keys with spending limits to avoid repeated passkey prompts
