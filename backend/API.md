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
      "blockNumber": 12345
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
- `count`: Number of markets returned in this response
- `total`: Total number of markets in the database

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
  "v4Pools": [
    {
      "poolType": "YES_TOKEN0/YES_TOKEN1",
      "poolId": "0x123abc...",
      "token0": "YES_0x4200000000000000000000000000000000000006",
      "token1": "YES_0x4200000000000000000000000000000000000007",
      "fee": 3000,
      "tickSpacing": 60,
      "liquidity": null,
      "sqrtPriceX96": null,
      "tick": null
    }
  ]
}
```

**Response Fields:**
- All fields from the markets list endpoint, plus:
- `verseTokens`: Array of verse token pairs created for this market
  - `asset`: The underlying asset address
  - `yesVerse`: Address of the YES verse token
  - `noVerse`: Address of the NO verse token
  - `transactionHash`: Transaction hash of verse token creation
- `v4Pools`: Array of Uniswap V4 pools created for this market (if V4 is enabled)
  - `poolType`: Type of pool (e.g., "YES_TOKEN0/YES_TOKEN1")
  - `poolId`: Unique identifier for the V4 pool
  - `token0`: First token in the pool
  - `token1`: Second token in the pool
  - `fee`: Pool fee in basis points (e.g., 3000 = 0.3%)
  - `tickSpacing`: Tick spacing for the pool
  - `liquidity`: Current liquidity (null if not fetched)
  - `sqrtPriceX96`: Current price (null if not fetched)
  - `tick`: Current tick (null if not fetched)

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
  "assets": ["0x4200000000000000000000000000000000000006", "0x4200000000000000000000000000000000000007"],
  "v4PoolConfig": {
    "createYes0Yes1": true,
    "createNo0No1": true,
    "createYes0No0": true,
    "createYes1No1": false
  }
}
```

**Request Fields:**
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `question` | string | Yes | The prediction market question (min 10 characters) |
| `resolutionDeadline` | integer | Yes | Unix timestamp when the market should resolve (must be in the future) |
| `assets` | array[string] | No | Array of ERC20 token addresses to create verse tokens for (minimum 2 required for V4 pools) |
| `v4PoolConfig` | object | No | Configuration for V4 pool creation (requires 2+ assets) |
| ↳ `createYes0Yes1` | boolean | No | Create YES_TOKEN0/YES_TOKEN1 pool (default: true) |
| ↳ `createNo0No1` | boolean | No | Create NO_TOKEN0/NO_TOKEN1 pool (default: true) |
| ↳ `createYes0No0` | boolean | No | Create YES_TOKEN0/NO_TOKEN0 pool (default: true) |
| ↳ `createYes1No1` | boolean | No | Create YES_TOKEN1/NO_TOKEN1 pool (default: false) |

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
      "asset": "0x4200000000000000000000000000000000000006",
      "yesVerse": "0xdef456...",
      "noVerse": "0x789ghi...",
      "transactionHash": "0xjkl012..."
    },
    {
      "asset": "0x4200000000000000000000000000000000000007",
      "yesVerse": "0xmno345...",
      "noVerse": "0xpqr678...",
      "transactionHash": "0xstu901..."
    }
  ],
  "v4Pools": [
    {
      "poolType": "YES_TOKEN0/YES_TOKEN1",
      "poolId": "0x123abc...",
      "token0": "YES_0x4200000000000000000000000000000000000006",
      "token1": "YES_0x4200000000000000000000000000000000000007",
      "fee": 3000,
      "tickSpacing": 60,
      "liquidity": null,
      "sqrtPriceX96": null,
      "tick": null
    },
    {
      "poolType": "NO_TOKEN0/NO_TOKEN1",
      "poolId": "0x456def...",
      "token0": "NO_0x4200000000000000000000000000000000000006",
      "token1": "NO_0x4200000000000000000000000000000000000007",
      "fee": 3000,
      "tickSpacing": 60,
      "liquidity": null,
      "sqrtPriceX96": null,
      "tick": null
    },
    {
      "poolType": "YES_TOKEN0/NO_TOKEN0",
      "poolId": "0x789ghi...",
      "token0": "YES_0x4200000000000000000000000000000000006",
      "token1": "NO_0x4200000000000000000000000000000000006",
      "fee": 3000,
      "tickSpacing": 60,
      "liquidity": null,
      "sqrtPriceX96": null,
      "tick": null
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
- `v4Pools`: Array of created Uniswap V4 pools (if 2+ assets and V4 enabled)

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

### V4 Pools Table
```sql
CREATE TABLE v4_pools (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    market_hash TEXT NOT NULL UNIQUE,
    pool_id_yes0_yes1 TEXT,
    pool_id_no0_no1 TEXT,
    pool_id_yes0_no0 TEXT,
    pool_id_yes1_no1 TEXT,
    token0_address TEXT NOT NULL,
    token1_address TEXT NOT NULL,
    fee INTEGER NOT NULL DEFAULT 3000,
    tick_spacing INTEGER NOT NULL DEFAULT 60,
    transaction_hash TEXT,
    created_at INTEGER NOT NULL,
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

# Blockchain Configuration
MULTIVERSE_ADDRESS=0x...  # MultiVerse contract address
ORACLE_ADDRESS=0x...       # Oracle contract address
RPC_URL=https://...        # HTTP RPC endpoint
WS_RPC_URL=wss://...       # WebSocket RPC endpoint
V4_POOL_MANAGER_ADDRESS=0x... # Uniswap V4 Pool Manager (optional)

# Wallet Configuration
PRIVATE_KEY=0x...          # Private key for signing transactions

# Database
DATABASE_URL=sqlite:///path/to/markets.db

# Admin Authentication
ADMIN_API_KEY_HASH=$2b$12$...  # Bcrypt hash from generate_api_key
```

**Note on V4 Pools:** The `V4_POOL_MANAGER_ADDRESS` is optional. If not provided, the system will run without Uniswap V4 pool creation capability. When provided, the admin API can create V4 pools for markets with 2 or more assets.

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

## Uniswap V4 Pool Integration

### Overview

The backend supports creating Uniswap V4 pools for prediction market tokens. When configured, the system can create pools that enable trading between:
- YES and NO tokens of the same asset (market outcome trading)
- YES tokens of different assets (YES economy)
- NO tokens of different assets (NO economy)

### Pool Configuration

When creating a market with 2 or more assets, you can configure which V4 pools to create:

| Pool Type | Description | Default |
|-----------|-------------|---------|
| `YES_TOKEN0/YES_TOKEN1` | Trade between YES tokens of different assets | ✓ Enabled |
| `NO_TOKEN0/NO_TOKEN1` | Trade between NO tokens of different assets | ✓ Enabled |
| `YES_TOKEN0/NO_TOKEN0` | Trade between YES/NO tokens of first asset | ✓ Enabled |
| `YES_TOKEN1/NO_TOKEN1` | Trade between YES/NO tokens of second asset | ✗ Disabled |

### Pool Parameters

All V4 pools are created with standard parameters:
- **Fee**: 0.3% (3000 basis points)
- **Tick Spacing**: 60
- **Initial Price**: 1:1 ratio (sqrtPriceX96 = 79228162514264337593543950336)
- **Hooks**: None (address(0))

### Requirements

1. **V4 Pool Manager**: Set `V4_POOL_MANAGER_ADDRESS` in environment variables
2. **Multiple Assets**: Provide at least 2 assets when creating a market
3. **Verse Tokens**: Verse tokens are created automatically before pools

### Example: Creating a Market with V4 Pools

```bash
curl -X POST http://127.0.0.1:3000/admin/markets/open \
  -H 'Authorization: Bearer YOUR_API_KEY' \
  -H 'Content-Type: application/json' \
  -d '{
    "question": "Will ETH reach $10k in 2025?",
    "resolutionDeadline": 1767225600,
    "assets": [
      "0x4200000000000000000000000000000000000006",  # WETH
      "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913"   # USDC
    ],
    "v4PoolConfig": {
      "createYes0Yes1": true,
      "createNo0No1": true,
      "createYes0No0": true,
      "createYes1No1": false
    }
  }'
```

This creates:
1. Verse tokens for both WETH and USDC
2. Three V4 pools:
   - YES_WETH/YES_USDC (YES economy)
   - NO_WETH/NO_USDC (NO economy)
   - YES_WETH/NO_WETH (WETH market outcome)

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

