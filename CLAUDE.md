# Binary Markets - Development Notes

## Backend Development

- Admin API uses Bearer token authentication (`Authorization: Bearer <token>`), not `X-API-Key` header
- Admin endpoint to create markets is `POST /admin/markets/open` with JSON body `{ "question": "...", "resolutionDeadline": <unix_timestamp> }` - the `resolutionDeadline` field must be camelCase and the timestamp must be in the future
- SQLite connection string for dev mode should use `sqlite:./dev.db?mode=rwc` (not `sqlite://`) - the `mode=rwc` flag creates the file if it doesn't exist

## Frontend Development

- CSS uses custom utility classes mimicking Tailwind (not actual Tailwind) - classes like `.bg-dark`, `.text-primary`, `.card-dark` are defined in `index.css`
- Dark theme CSS variables are in `:root` in `index.css` - key variables include `--bg-primary`, `--accent-purple`, `--gradient-purple`, etc.
- Google Fonts are loaded via `@import url()` at the top of `index.css` - currently uses DM Sans for body and JetBrains Mono for monospace

## Dev Environment Orchestrator

- Frontend health check should use `http://localhost:${port}` (not `127.0.0.1`) and add `--host` flag to vite for network binding
- The orchestrator script is at `script/env.ts` - run with `bun run script/env.ts dev` for interactive mode or `bun run script/env.ts test` for test mode
