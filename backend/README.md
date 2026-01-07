# Backend

A Rust web server with real-time event indexing for the MultiVerse prediction market protocol.

## Features

- **Real-time Event Indexing**: Listens to `MarketOpened` events via WebSocket and stores them in SQLite
- **REST API**: Query indexed markets with pagination, get market details with live orderbook data
- **Admin API**: Create new markets with authenticated endpoints
- **Tempo DEX Integration**: Live orderbook prices from Tempo's Stablecoin DEX
- **Self-contained**: Embedded SQLite database, no external services required

## Architecture

- **Database** (`src/db.rs`): SQLite schema and queries for markets, verse tokens, and orderbooks
- **Event Listener** (`src/indexer.rs`): WebSocket subscription to MultiVerse contract events
- **API Routes** (`src/routes.rs`): REST endpoints for querying markets
- **Admin Routes** (`src/admin.rs`): Authenticated endpoints for creating markets
- **Auth Middleware** (`src/auth.rs`): Bearer token authentication
- **Contract Client** (`src/contract.rs`): Alloy-based MultiVerse contract interaction
- **Orderbook** (`src/tempo_orderbook.rs`): Tempo Stablecoin DEX integration

## Quick Start

```bash
# Start full local environment (from project root)
make dev

# Or run backend directly (requires Tempo node and deployed contracts)
cargo run --bin backend
```

## Setup

1. Generate contract bindings (run from project root):
```bash
cd contracts
forge bind --module --bindings-path ../backend/src/bindings --select MultiVerse \
  --select Verse --select TrustedOracle --overwrite
```

2. Copy `.env.example` to `.env` and configure:
```bash
cd backend
cp .env.example .env
```

3. Update `.env` with your RPC URLs and contract addresses:
```env
RPC_URL=https://tempo-testnet.rpc.example/v1/YOUR_KEY
WS_RPC_URL=wss://tempo-testnet.rpc.example/v1/YOUR_KEY
MULTIVERSE_ADDRESS=0x...
DATABASE_URL=sqlite:./markets.db?mode=rwc
ADMIN_API_KEY_HASH=$2b$12$...  # From generate_api_key
```

4. Generate an admin API key:
```bash
cargo run --bin generate_api_key
```

## Running

```bash
cargo run --bin backend
```

The server will:
1. Initialize SQLite database with markets, verse_tokens, and orderbook_markets tables
2. Connect to WebSocket RPC
3. Subscribe to MarketOpened events
4. Start HTTP server on `127.0.0.1:3000`

## API Endpoints

### Public Endpoints

#### GET /markets

Query indexed markets with pagination.

**Query Parameters:**
- `limit` (optional): Max results to return (default: 50, max: 100)
- `offset` (optional): Number of results to skip (default: 0)

**Response:**
```json
{
  "markets": [
    {
      "marketHash": "0x...",
      "questionHash": "0x...",
      "question": "Will ETH reach $5000?",
      "resolutionDeadline": 1767225600,
      "oracle": "0x...",
      "blockNumber": 12345,
      "yesProbability": null,
      "noProbability": null,
      "resolution": null
    }
  ],
  "count": 1,
  "total": 10
}
```

**Note:** `yesProbability`, `noProbability`, and `resolution` are `null` in list view for performance. Use the detail endpoint for full data.

**Example:**
```bash
curl http://127.0.0.1:3000/markets?limit=10&offset=0
```

#### GET /markets/:marketHash

Get detailed market information with live orderbook data.

**Response:**
```json
{
  "marketHash": "0x...",
  "questionHash": "0x...",
  "question": "Will ETH reach $5000?",
  "resolutionDeadline": 1767225600,
  "oracle": "0x...",
  "blockNumber": 12345,
  "verseTokens": [
    {
      "asset": "0x20C0...",
      "yesVerse": "0xabc...",
      "noVerse": "0xdef...",
      "transactionHash": "0x..."
    }
  ],
  "orderbooks": [
    {
      "pairType": "YES/QUOTE",
      "pairKey": "0x...",
      "baseToken": "0x...",
      "quoteToken": "0x...",
      "bestBidTick": -10,
      "bestAskTick": 10,
      "bestBidPrice": "0.999900",
      "bestAskPrice": "1.000100",
      "midPrice": "1.000000",
      "spreadBps": 2.0
    }
  ],
  "yesProbability": 0.65,
  "noProbability": 0.35,
  "resolution": "UNRESOLVED"
}
```

**Example:**
```bash
curl http://127.0.0.1:3000/markets/0x123abc...
```

#### GET /markets/:marketHash/verse-tokens

Get verse tokens for a specific market.

**Response:**
```json
[
  {
    "asset": "0x20C0...",
    "yesVerse": "0xabc...",
    "noVerse": "0xdef...",
    "transactionHash": "0x..."
  }
]
```

**Example:**
```bash
curl http://127.0.0.1:3000/markets/0x123abc.../verse-tokens
```

### Admin Endpoints (Authenticated)

#### POST /admin/markets/open

Create a new prediction market.

**Headers:**
```
Authorization: Bearer <API_KEY>
Content-Type: application/json
```

**Request Body:**
```json
{
  "question": "Will ETH reach $5000 by end of 2025?",
  "resolutionDeadline": 1767225600,
  "assets": ["0x20C0000000000000000000000000000000000000"],
  "quoteToken": "0x20C0000000000000000000000000000000000000"
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `question` | string | Yes | Market question (min 10 chars) |
| `resolutionDeadline` | number | Yes | Unix timestamp (must be in future) |
| `assets` | string[] | No | Asset addresses for verse tokens |
| `quoteToken` | string | No | Quote token for orderbook (defaults to pathUSD) |

**Response:** `200 OK`
```json
{
  "marketHash": "0x...",
  "questionHash": "0x...",
  "question": "Will ETH reach $5000 by end of 2025?",
  "resolutionDeadline": 1767225600,
  "oracle": "0x...",
  "transactionHash": "0x...",
  "verseTokens": [...],
  "orderbooks": [...]
}
```

**Example:**
```bash
curl -X POST http://127.0.0.1:3000/admin/markets/open \
  -H 'Authorization: Bearer YOUR_API_KEY' \
  -H 'Content-Type: application/json' \
  -d '{"question": "Test market?", "resolutionDeadline": 1893456000, "assets": []}'
```

## Database Schema

```sql
-- Markets table
CREATE TABLE markets (
    market_hash TEXT PRIMARY KEY,
    question_hash TEXT NOT NULL,
    resolution_deadline INTEGER NOT NULL,
    oracle TEXT NOT NULL,
    block_number INTEGER NOT NULL,
    indexed_at INTEGER NOT NULL,
    question_text TEXT
);

-- Verse tokens table
CREATE TABLE verse_tokens (
    market_hash TEXT NOT NULL,
    asset TEXT NOT NULL,
    yes_verse TEXT NOT NULL,
    no_verse TEXT NOT NULL,
    transaction_hash TEXT,
    created_at INTEGER NOT NULL,
    PRIMARY KEY (market_hash, asset),
    FOREIGN KEY (market_hash) REFERENCES markets(market_hash)
);

-- Orderbook markets table
CREATE TABLE orderbook_markets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    market_hash TEXT NOT NULL,
    asset_address TEXT NOT NULL,
    quote_token_address TEXT NOT NULL,
    yes_pair_key TEXT,
    no_pair_key TEXT,
    created_at INTEGER NOT NULL,
    UNIQUE(market_hash, asset_address),
    FOREIGN KEY (market_hash) REFERENCES markets(market_hash)
);
```

## Testing

### Unit Tests (No external dependencies)

```bash
# Run unit tests only (fast)
cargo test --test unit_tests

# Or via make
make test-unit
```

### Integration Tests (Requires orchestrator)

```bash
# Run full test suite (starts Tempo, deploys contracts, etc.)
make test

# Or manually with env vars
RPC_URL=http://localhost:9545 SERVER_URL=http://localhost:3001 cargo test --test integration_test -- --ignored
```

## Event Indexing

The indexer automatically:
- Subscribes to `MarketOpened` events from the MultiVerse contract
- Stores new markets in the database as they're created
- Resumes from the last indexed block on restart
- Handles reconnections automatically

When a new market is opened on-chain, you'll see logs like:
```
INFO backend::indexer: Received MarketOpened event: market_hash=0x..., block=12345
INFO backend::indexer: Market indexed successfully
```

## Error Handling

| Status Code | Description |
|-------------|-------------|
| 200 | Success |
| 400 | Bad Request - Invalid parameters |
| 401 | Unauthorized - Invalid or missing API key |
| 404 | Not Found - Market doesn't exist |
| 500 | Internal Server Error |

All error responses include a JSON body:
```json
{
  "error": "Error description"
}
```

## See Also

- [API.md](./API.md) - Full API documentation with examples
- [CLAUDE.md](../CLAUDE.md) - Project development notes
