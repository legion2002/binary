# MultiVerse Prediction Market API Documentation

## Overview

The MultiVerse backend provides a REST API for interacting with prediction markets on the blockchain. The API includes public endpoints for querying market data and authenticated admin endpoints for managing markets.

## Base URL

```
http://127.0.0.1:3000
```

For production deployments, replace with your server's domain.

## Authentication

### Admin Endpoints

Admin endpoints require Bearer token authentication using an API key.

**Header Format:**
```
Authorization: Bearer <API_KEY>
```

### Generating API Keys

To generate a new API key and hash:

```bash
cargo run --bin generate_api_key
```

This will output:
1. A random 64-character hexadecimal API key
2. A bcrypt hash to store in your `.env` file
3. Instructions for using the API key

**Example Output:**
```
Generated API Key:
a1b2c3d4e5f6...

Bcrypt Hash:
$2b$12$HASH_VALUE_HERE

Add to .env:
ADMIN_API_KEY_HASH=$2b$12$HASH_VALUE_HERE
```

## Response Format

All responses are JSON formatted. Successful responses will have the appropriate HTTP status code (200, 201, etc.) and JSON body. Error responses include an `error` field with a description.

### Error Response Format

```json
{
  "error": "Error description"
}
```

## Endpoints

### Public Endpoints

#### 1. Get Markets

Retrieve a paginated list of all prediction markets.

**Endpoint:** `GET /markets`

**Query Parameters:**
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `limit` | integer | No | 50 | Maximum number of markets to return (max: 100) |
| `offset` | integer | No | 0 | Number of markets to skip for pagination |

**Response:** `200 OK`
```json
{
  "markets": [
    {
      "marketHash": "0x123abc...",
      "questionHash": "0x456def...",
      "question": "Will ETH price be above $5000 by end of 2025?",
      "resolutionDeadline": 1767225600,
      "oracle": "0x789ghi...",
      "blockNumber": 12345,
      "yesProbability": null,
      "noProbability": null,
      "resolution": null
    }
  ],
  "count": 1,
  "total": 1
}
```

**Response Fields:**
- `marketHash`: Unique identifier for the market (bytes32 hex string)
- `questionHash`: Keccak256 hash of the question text
- `question`: The actual question text (may be null for markets indexed from events)
- `resolutionDeadline`: Unix timestamp when the market resolves
- `oracle`: Address of the oracle that will resolve this market
- `blockNumber`: Block number when the market was created
- `yesProbability`: Probability of YES outcome (0.0-1.0, null in list view for performance)
- `noProbability`: Probability of NO outcome (0.0-1.0, null in list view for performance)
- `resolution`: Market resolution status (null in list view, use detail endpoint for full data)
- `count`: Number of markets returned in this response
- `total`: Total number of markets in the database

**Note:** For performance reasons, `yesProbability`, `noProbability`, and `resolution` are null in the list view. Use the single market endpoint (`GET /markets/:marketHash`) to get full data including live orderbook prices and resolution status.

**Example Request:**
```bash
curl "http://127.0.0.1:3000/markets?limit=10&offset=0"
```

#### 2. Get Single Market

Retrieve detailed information about a specific market.

**Endpoint:** `GET /markets/:marketHash`

**Path Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| `marketHash` | string | The market hash (with or without 0x prefix) |

**Response:** `200 OK`
```json
{
  "marketHash": "0x123abc...",
  "questionHash": "0x456def...",
  "question": "Will ETH price be above $5000 by end of 2025?",
  "resolutionDeadline": 1767225600,
  "oracle": "0x789ghi...",
  "blockNumber": 12345,
  "verseTokens": [
    {
      "asset": "0x4200000000000000000000000000000000000006",
      "yesVerse": "0xabc123...",
      "noVerse": "0xdef456...",
      "transactionHash": "0x789ghi..."
    }
  ],
  "orderbooks": [
    {
      "pairType": "YES/QUOTE",
      "pairKey": "0x123abc...",
      "baseToken": "YES_0x4200000000000000000000000000000000000006",
      "quoteToken": "0x20c0000000000000000000000000000000000000",
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

**Response Fields:**
- All fields from the markets list endpoint, plus:
- `verseTokens`: Array of verse token pairs created for this market
  - `asset`: The underlying asset address
  - `yesVerse`: Address of the YES verse token
  - `noVerse`: Address of the NO verse token
  - `transactionHash`: Transaction hash of verse token creation
- `orderbooks`: Array of Tempo DEX orderbook info for this market (fetched live from Tempo DEX)
  - `pairType`: Type of pair (e.g., "YES/QUOTE" or "NO/QUOTE")
  - `pairKey`: Unique identifier for the trading pair
  - `baseToken`: Base token address (YES or NO verse token)
  - `quoteToken`: Quote token address (typically pathUSD)
  - `bestBidTick`: Best bid price tick (null if no bids)
  - `bestAskTick`: Best ask price tick (null if no asks)
  - `bestBidPrice`: Human-readable bid price (live from Tempo DEX)
  - `bestAskPrice`: Human-readable ask price (live from Tempo DEX)
  - `midPrice`: Mid-market price (live from Tempo DEX)
  - `spreadBps`: Spread in basis points
- `yesProbability`: Probability of YES outcome (0.0-1.0), calculated from orderbook mid prices
- `noProbability`: Probability of NO outcome (0.0-1.0), calculated from orderbook mid prices
- `resolution`: Market resolution status from on-chain contract ("UNRESOLVED", "YES", "NO", "EVEN")

**Error Responses:**
- `404 Not Found`: Market does not exist

**Example Request:**
```bash
curl "http://127.0.0.1:3000/markets/0x123abc..."
```

#### 3. Get Verse Tokens for Market

Retrieve all verse tokens associated with a specific market.

**Endpoint:** `GET /markets/:marketHash/verse-tokens`

**Path Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| `marketHash` | string | The market hash (with or without 0x prefix) |

**Response:** `200 OK`
```json
[
  {
    "asset": "0x4200000000000000000000000000000000000006",
    "yesVerse": "0xabc123...",
    "noVerse": "0xdef456...",
    "transactionHash": "0x789ghi..."
  }
]
```

**Error Responses:**
- `404 Not Found`: Market does not exist

**Example Request:**
```bash
curl "http://127.0.0.1:3000/markets/0x123abc.../verse-tokens"
```

### Admin Endpoints (Authenticated)

#### 4. Open Market

Create a new prediction market on the blockchain.

**Endpoint:** `POST /admin/markets/open`

**Headers:**
```
Authorization: Bearer <API_KEY>
Content-Type: application/json
```

**Request Body:**
```json
{
  "question": "Will FOCIL be included in Ethereum by end of 2025?",
  "resolutionDeadline": 1767225600,
  "assets": ["0x20c0000000000000000000000000000000000001"],
  "quoteToken": "0x20c0000000000000000000000000000000000000"
}
```

**Request Fields:**
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `question` | string | Yes | The prediction market question (min 10 characters) |
| `resolutionDeadline` | integer | Yes | Unix timestamp when the market should resolve (must be in the future) |
| `assets` | array[string] | No | Array of TIP20 token addresses to create verse tokens for |
| `quoteToken` | string | No | Quote token for orderbook trading (defaults to pathUSD: `0x20c0...`) |

**Response:** `200 OK`
```json
{
  "marketHash": "0x123abc...",
  "questionHash": "0x456def...",
  "question": "Will FOCIL be included in Ethereum by end of 2025?",
  "resolutionDeadline": 1767225600,
  "oracle": "0x789ghi...",
  "transactionHash": "0xabc123...",
  "verseTokens": [
    {
      "asset": "0x20c0000000000000000000000000000000000001",
      "yesVerse": "0xdef456...",
      "noVerse": "0x789ghi...",
      "transactionHash": "0xjkl012..."
    }
  ],
  "orderbooks": [
    {
      "pairType": "YES/QUOTE",
      "pairKey": "0x123abc...",
      "baseToken": "YES_0x20c0000000000000000000000000000000000001",
      "quoteToken": "0x20c0000000000000000000000000000000000000",
      "bestBidTick": null,
      "bestAskTick": null,
      "bestBidPrice": null,
      "bestAskPrice": null,
      "midPrice": null,
      "spreadBps": null
    },
    {
      "pairType": "NO/QUOTE",
      "pairKey": "0x456def...",
      "baseToken": "NO_0x20c0000000000000000000000000000000000001",
      "quoteToken": "0x20c0000000000000000000000000000000000000",
      "bestBidTick": null,
      "bestAskTick": null,
      "bestBidPrice": null,
      "bestAskPrice": null,
      "midPrice": null,
      "spreadBps": null
    }
  ]
}
```

**Response Fields:**
- `marketHash`: Unique identifier for the created market
- `questionHash`: Keccak256 hash of the question
- `question`: The question text
- `resolutionDeadline`: Unix timestamp for resolution
- `oracle`: Address of the oracle for this market
- `transactionHash`: Transaction hash of market creation
- `verseTokens`: Array of created verse tokens (if assets were provided)
- `orderbooks`: Array of Tempo DEX orderbook info for created verse tokens

**Error Responses:**
- `400 Bad Request`: Invalid request parameters
  - Question is empty or too short
  - Resolution deadline is in the past
  - Invalid asset addresses
- `401 Unauthorized`: Missing or invalid API key
- `206 Partial Content`: Market created but some verse tokens failed
- `500 Internal Server Error`: Transaction failed

**Example Request:**
```bash
curl -X POST http://127.0.0.1:3000/admin/markets/open \
  -H 'Authorization: Bearer a1b2c3d4e5f6...' \
  -H 'Content-Type: application/json' \
  -d '{
    "question": "Will Bitcoin reach $100k in 2025?",
    "resolutionDeadline": 1767225600,
    "assets": ["0x4200000000000000000000000000000000000006"]
  }'
```

## Database Schema

The backend uses SQLite to store indexed market data:

### Markets Table
```sql
CREATE TABLE markets (
    market_hash TEXT PRIMARY KEY,
    question_hash TEXT NOT NULL,
    resolution_deadline INTEGER NOT NULL,
    oracle TEXT NOT NULL,
    block_number INTEGER NOT NULL,
    indexed_at INTEGER NOT NULL,
    question_text TEXT
);
```

### Verse Tokens Table
```sql
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
```

### Orderbook Markets Table
```sql
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

## Real-Time Event Indexing

The backend automatically indexes blockchain events via WebSocket connection:

### Indexed Events
- **MarketOpened**: Captures new markets created on-chain
  - Stores market hash, question hash, deadline, oracle, and block number
  - Note: Question text is only available for markets created via admin API

### WebSocket Configuration

Configure the WebSocket URL in your `.env` file:
```env
WS_RPC_URL=wss://base-sepolia.g.alchemy.com/v2/YOUR_KEY
```

The indexer will:
1. Connect to the WebSocket RPC endpoint
2. Subscribe to `MarketOpened` events from the MultiVerse contract
3. Store new markets in the database in real-time
4. Handle reconnections automatically

## Environment Variables

Required environment variables for the backend:

```env
# Server Configuration
HOST=127.0.0.1
PORT=3000

# Blockchain Configuration (Tempo Testnet)
MULTIVERSE_ADDRESS=0x...  # MultiVerse contract address
ORACLE_ADDRESS=0x...       # Oracle contract address
RPC_URL=https://rpc.testnet.tempo.xyz  # Tempo HTTP RPC endpoint
WS_RPC_URL=wss://rpc.testnet.tempo.xyz # Tempo WebSocket RPC endpoint

# Wallet Configuration
PRIVATE_KEY=0x...          # Private key for signing transactions

# Database
DATABASE_URL=sqlite:///path/to/markets.db

# Admin Authentication
ADMIN_API_KEY_HASH=$2b$12$...  # Bcrypt hash from generate_api_key
```

**Note on Tempo:** The backend now uses Tempo's native Stablecoin DEX for orderbook-based trading. The DEX is available at the precompile address `0xDEc0000000000000000000000000000000000000` on Tempo networks.

## Rate Limiting

Currently, the API does not implement rate limiting. For production deployments, consider adding rate limiting middleware.

## CORS Configuration

The API is configured with permissive CORS settings, allowing requests from any origin. For production, configure CORS to restrict allowed origins:

```rust
let cors = CorsLayer::new()
    .allow_origin(["https://yourfrontend.com"])
    .allow_methods([Method::GET, Method::POST])
    .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);
```

## Error Handling

The API uses standard HTTP status codes:

| Status Code | Description |
|-------------|-------------|
| 200 | Success |
| 400 | Bad Request - Invalid parameters |
| 401 | Unauthorized - Invalid or missing API key |
| 404 | Not Found - Resource doesn't exist |
| 500 | Internal Server Error |

All error responses include a JSON body with an `error` field describing the issue.

## Testing the API

### Health Check
```bash
# Check if the server is running
curl http://127.0.0.1:3000/markets
```

### Create a Test Market (Admin)
```bash
# First generate an API key
cargo run --bin generate_api_key

# Then use the key to create a market
curl -X POST http://127.0.0.1:3000/admin/markets/open \
  -H 'Authorization: Bearer YOUR_API_KEY' \
  -H 'Content-Type: application/json' \
  -d '{
    "question": "Test market question?",
    "resolutionDeadline": 1893456000,
    "assets": []
  }'
```

### Integration Tests

Run the full integration test suite:
```bash
cargo test --test integration_test -- --ignored
```

This will:
1. Start a local Anvil instance
2. Deploy contracts
3. Start the backend server
4. Test all API endpoints
5. Verify event indexing

## Contract Interactions

The backend interacts with the following smart contracts:

### MultiVerse Contract
- **open(questionHash, resolutionDeadline, oracle)**: Create a new market
- **create(asset, marketHash)**: Create verse tokens for an asset/market pair
- **getVerseAddress(asset, marketHash)**: Get verse token addresses
- **markets(marketHash)**: Get market details
- **resolutions(marketHash)**: Get market resolution status

### Market Hash Calculation

Market hashes are calculated as:
```solidity
keccak256(abi.encode(questionHash, resolutionDeadline, oracle))
```

Question hashes are calculated as:
```solidity
keccak256(bytes(questionText))
```

## Tempo Stablecoin DEX Integration

### Overview

The backend integrates with Tempo's native Stablecoin DEX (orderbook) at `0xDEc0000000000000000000000000000000000000`. This enables trading between prediction market tokens (YES/NO verse tokens) using Tempo's enshrined orderbook.

### Key Features

- **Orderbook-based trading**: Price-time priority orderbook for efficient price discovery
- **pathUSD quote token**: Stablecoins trade against pathUSD (`0x20c0000000000000000000000000000000000000`)
- **Gasless transactions**: Fee sponsorship via `withFeePayer` transport
- **Call batching**: Atomic multi-call operations (e.g., approve + swap in one tx)

### Orderbook Data

Markets return orderbook information for each asset's verse tokens:

| Field | Description |
|-------|-------------|
| `pairType` | Either "YES/QUOTE" or "NO/QUOTE" |
| `pairKey` | Unique identifier for the trading pair |
| `baseToken` | YES or NO verse token address |
| `quoteToken` | Quote token address (typically pathUSD) |
| `bestBidTick` | Best bid price tick (null if no bids) |
| `bestAskTick` | Best ask price tick (null if no asks) |
| `bestBidPrice` | Human-readable bid price |
| `bestAskPrice` | Human-readable ask price |
| `midPrice` | Mid-market price |
| `spreadBps` | Spread in basis points |

### Tick Pricing

Tempo DEX uses tick-based pricing:
- `tick = (price - 1) * 100,000`
- Price $1.00 = tick 0
- Price $0.99 = tick -1000
- Price $1.01 = tick 1000

### Example: Creating a Market with Orderbook Trading

```bash
curl -X POST http://127.0.0.1:3000/admin/markets/open \
  -H 'Authorization: Bearer YOUR_API_KEY' \
  -H 'Content-Type: application/json' \
  -d '{
    "question": "Will ETH reach $10k in 2025?",
    "resolutionDeadline": 1767225600,
    "assets": [
      "0x20c0000000000000000000000000000000000001"
    ],
    "quoteToken": "0x20c0000000000000000000000000000000000000"
  }'
```

This creates:
1. Verse tokens (YES and NO) for the specified asset
2. Orderbook info for trading YES/pathUSD and NO/pathUSD

### Frontend Integration

Use viem/tempo for swap execution:

```typescript
import { Actions, Addresses } from 'viem/tempo';

const calls = [
  Actions.token.approve.call({
    amount: maxAmountIn,
    spender: Addresses.stablecoinExchange,
    token: tokenIn,
  }),
  Actions.dex.buy.call({
    amountOut: amount,
    maxAmountIn,
    tokenIn,
    tokenOut,
  }),
];

sendCalls.sendCallsSync({ calls });
```

## Monitoring and Logging

The backend uses the `tracing` crate for structured logging. Configure log levels via the `RUST_LOG` environment variable:

```bash
# Debug level for backend, info for others
RUST_LOG=backend=debug,tower_http=info cargo run

# Info level for all
RUST_LOG=info cargo run
```

Key log events:
- Server startup and configuration
- Market creation (admin API)
- Event indexing from blockchain
- API request/response tracing
- Database operations
- Authentication attempts

## Deployment Considerations

### Production Checklist

1. **Security**
   - Use HTTPS with TLS certificates
   - Implement rate limiting
   - Configure restrictive CORS policies
   - Use strong API keys (64+ characters)
   - Store API key hashes securely
   - Never log sensitive data

2. **Database**
   - Use PostgreSQL for production instead of SQLite
   - Implement database backups
   - Add connection pooling
   - Monitor database performance

3. **Monitoring**
   - Set up application monitoring (e.g., Prometheus)
   - Configure alerting for errors
   - Monitor blockchain RPC connection health
   - Track API response times

4. **High Availability**
   - Deploy multiple instances behind a load balancer
   - Use persistent storage for the database
   - Implement graceful shutdown handling
   - Configure health check endpoints

### Docker Deployment

Example Dockerfile:
```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin backend

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/backend /usr/local/bin/backend
CMD ["backend"]
```

