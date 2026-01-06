# Backend

A Rust web server with real-time event indexing for the MultiVerse prediction market protocol.

## Features

- **Real-time Event Indexing**: Listens to `MarketOpened` events via WebSocket and stores them in SQLite
- **REST API**: Query indexed markets with pagination
- **Self-contained**: Embedded SQLite database, no external services required

## Architecture

- **Database** (`src/db.rs`): SQLite schema and queries
- **Event Listener** (`src/indexer.rs`): WebSocket subscription to MultiVerse contract events
- **API Routes** (`src/routes.rs`): REST endpoints for querying markets
- **Contract Client** (`src/contract.rs`): Alloy-based MultiVerse contract interaction

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
RPC_URL=https://base-sepolia.g.alchemy.com/v2/YOUR_KEY
WS_RPC_URL=wss://base-sepolia.g.alchemy.com/v2/YOUR_KEY
MULTIVERSE_ADDRESS=0x...
DATABASE_URL=sqlite:///absolute/path/to/markets.db
```

4. Create the database file:
```bash
touch markets.db
```

## Running

```bash
cargo run --bin backend
```

The server will:
1. Initialize SQLite database with markets table
2. Connect to WebSocket RPC
3. Subscribe to MarketOpened events
4. Start HTTP server on `127.0.0.1:3000`

## API Endpoints

### GET /markets

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
      "resolutionDeadline": 1234567890,
      "oracle": "0x...",
      "blockNumber": 12345
    }
  ],
  "count": 1,
  "total": 1
}
```

**Example:**
```bash
curl http://127.0.0.1:3000/markets?limit=10&offset=0
```

## Database Schema

```sql
CREATE TABLE markets (
    market_hash TEXT PRIMARY KEY,
    question_hash TEXT NOT NULL,
    resolution_deadline INTEGER NOT NULL,
    oracle TEXT NOT NULL,
    block_number INTEGER NOT NULL,
    indexed_at INTEGER NOT NULL
);

CREATE INDEX idx_markets_block_number ON markets(block_number);
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
