use alloy::primitives::FixedBytes;
use chrono::Utc;
use sqlx::{sqlite::SqlitePool, FromRow};

#[derive(Debug, Clone, FromRow)]
pub struct Market {
    pub market_hash: String,
    pub question_hash: String,
    pub resolution_deadline: i64,
    pub oracle: String,
    pub block_number: i64,
    pub indexed_at: i64,
    pub question_text: Option<String>,
}

#[derive(Debug, Clone, FromRow)]
pub struct VerseToken {
    pub market_hash: String,
    pub asset: String,
    pub yes_verse: String,
    pub no_verse: String,
    pub transaction_hash: Option<String>,
    pub created_at: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct OrderbookData {
    pub market_hash: String,
    pub asset_address: String,
    pub quote_token_address: String,
    pub yes_pair_key: Option<String>,
    pub no_pair_key: Option<String>,
    pub created_at: i64,
}

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// Initialize the database and run migrations
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;

        // Create markets table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS markets (
                market_hash TEXT PRIMARY KEY,
                question_hash TEXT NOT NULL,
                resolution_deadline INTEGER NOT NULL,
                oracle TEXT NOT NULL,
                block_number INTEGER NOT NULL,
                indexed_at INTEGER NOT NULL
            )
            "#,
        )
        .execute(&pool)
        .await?;

        // Create index on block_number for efficient queries
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_markets_block_number
            ON markets(block_number)
            "#,
        )
        .execute(&pool)
        .await?;

        // Add question_text column if it doesn't exist
        // SQLite doesn't have ADD COLUMN IF NOT EXISTS, so we ignore the error if column exists
        let _ = sqlx::query(
            r#"
            ALTER TABLE markets ADD COLUMN question_text TEXT
            "#,
        )
        .execute(&pool)
        .await;

        // Create verse_tokens table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS verse_tokens (
                market_hash TEXT NOT NULL,
                asset TEXT NOT NULL,
                yes_verse TEXT NOT NULL,
                no_verse TEXT NOT NULL,
                transaction_hash TEXT,
                created_at INTEGER NOT NULL,
                PRIMARY KEY (market_hash, asset),
                FOREIGN KEY (market_hash) REFERENCES markets(market_hash)
            )
            "#,
        )
        .execute(&pool)
        .await?;

        // Create index on market_hash for verse_tokens
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_verse_tokens_market
            ON verse_tokens(market_hash)
            "#,
        )
        .execute(&pool)
        .await?;

        // Create orderbook_markets table for Tempo Stablecoin DEX
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS orderbook_markets (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                market_hash TEXT NOT NULL,
                asset_address TEXT NOT NULL,
                quote_token_address TEXT NOT NULL,
                yes_pair_key TEXT,
                no_pair_key TEXT,
                created_at INTEGER NOT NULL,
                UNIQUE(market_hash, asset_address),
                FOREIGN KEY (market_hash) REFERENCES markets(market_hash)
            )
            "#,
        )
        .execute(&pool)
        .await?;

        // Create index on market_hash for orderbook_markets
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_orderbook_markets_market
            ON orderbook_markets(market_hash)
            "#,
        )
        .execute(&pool)
        .await?;

        tracing::info!("Database initialized successfully");

        Ok(Self { pool })
    }

    /// Insert a new market from MarketOpened event
    pub async fn insert_market(
        &self,
        market_hash: FixedBytes<32>,
        question_hash: FixedBytes<32>,
        resolution_deadline: u32,
        oracle: alloy::primitives::Address,
        block_number: u64,
    ) -> anyhow::Result<()> {
        let market_hash_str = format!("0x{}", hex::encode(market_hash));
        let question_hash_str = format!("0x{}", hex::encode(question_hash));
        let oracle_str = format!("{:?}", oracle);
        let indexed_at = Utc::now().timestamp();

        sqlx::query(
            r#"
            INSERT OR IGNORE INTO markets
            (market_hash, question_hash, resolution_deadline, oracle, block_number, indexed_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(market_hash_str)
        .bind(question_hash_str)
        .bind(resolution_deadline as i64)
        .bind(oracle_str)
        .bind(block_number as i64)
        .bind(indexed_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get all markets
    pub async fn get_all_markets(&self) -> anyhow::Result<Vec<Market>> {
        let markets = sqlx::query_as::<_, Market>(
            r#"
            SELECT market_hash, question_hash, resolution_deadline, oracle, block_number, indexed_at, question_text
            FROM markets
            ORDER BY block_number DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(markets)
    }

    /// Get markets with pagination
    pub async fn get_markets_paginated(
        &self,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<Market>> {
        let markets = sqlx::query_as::<_, Market>(
            r#"
            SELECT market_hash, question_hash, resolution_deadline, oracle, block_number, indexed_at, question_text
            FROM markets
            ORDER BY block_number DESC
            LIMIT ? OFFSET ?
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(markets)
    }

    /// Get a specific market by hash
    pub async fn get_market(&self, market_hash: &str) -> anyhow::Result<Option<Market>> {
        let market = sqlx::query_as::<_, Market>(
            r#"
            SELECT market_hash, question_hash, resolution_deadline, oracle, block_number, indexed_at, question_text
            FROM markets
            WHERE market_hash = ?
            "#,
        )
        .bind(market_hash)
        .fetch_optional(&self.pool)
        .await?;

        Ok(market)
    }

    /// Get the total count of markets
    pub async fn get_markets_count(&self) -> anyhow::Result<i64> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM markets")
            .fetch_one(&self.pool)
            .await?;

        Ok(count.0)
    }

    /// Insert a new market with question text (for admin API)
    pub async fn insert_market_with_question(
        &self,
        market_hash: FixedBytes<32>,
        question_hash: FixedBytes<32>,
        question_text: String,
        resolution_deadline: u32,
        oracle: alloy::primitives::Address,
    ) -> anyhow::Result<()> {
        let market_hash_str = format!("0x{}", hex::encode(market_hash));
        let question_hash_str = format!("0x{}", hex::encode(question_hash));
        let oracle_str = format!("{:?}", oracle);
        let indexed_at = Utc::now().timestamp();

        sqlx::query(
            r#"
            INSERT INTO markets
            (market_hash, question_hash, resolution_deadline, oracle, block_number, indexed_at, question_text)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(market_hash_str)
        .bind(question_hash_str)
        .bind(resolution_deadline as i64)
        .bind(oracle_str)
        .bind(0i64) // block_number will be updated by indexer when event is received
        .bind(indexed_at)
        .bind(question_text)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Insert verse tokens for a market/asset pair
    pub async fn insert_verse_tokens(
        &self,
        market_hash: FixedBytes<32>,
        asset: alloy::primitives::Address,
        yes_verse: alloy::primitives::Address,
        no_verse: alloy::primitives::Address,
        transaction_hash: Option<String>,
    ) -> anyhow::Result<()> {
        let market_hash_str = format!("0x{}", hex::encode(market_hash));
        let asset_str = format!("{:?}", asset);
        let yes_verse_str = format!("{:?}", yes_verse);
        let no_verse_str = format!("{:?}", no_verse);
        let created_at = Utc::now().timestamp();

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO verse_tokens
            (market_hash, asset, yes_verse, no_verse, transaction_hash, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(market_hash_str)
        .bind(asset_str)
        .bind(yes_verse_str)
        .bind(no_verse_str)
        .bind(transaction_hash)
        .bind(created_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get all verse tokens for a specific market
    pub async fn get_verse_tokens_for_market(
        &self,
        market_hash: &str,
    ) -> anyhow::Result<Vec<VerseToken>> {
        let verse_tokens = sqlx::query_as::<_, VerseToken>(
            r#"
            SELECT market_hash, asset, yes_verse, no_verse, transaction_hash, created_at
            FROM verse_tokens
            WHERE market_hash = ?
            ORDER BY created_at ASC
            "#,
        )
        .bind(market_hash)
        .fetch_all(&self.pool)
        .await?;

        Ok(verse_tokens)
    }

    /// Insert orderbook data for a market
    pub async fn insert_orderbook_market(
        &self,
        market_hash: FixedBytes<32>,
        asset_address: alloy::primitives::Address,
        quote_token_address: alloy::primitives::Address,
        yes_pair_key: Option<String>,
        no_pair_key: Option<String>,
    ) -> anyhow::Result<()> {
        let market_hash_str = format!("0x{}", hex::encode(market_hash));
        let asset_str = format!("{:?}", asset_address);
        let quote_str = format!("{:?}", quote_token_address);
        let created_at = Utc::now().timestamp();

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO orderbook_markets
            (market_hash, asset_address, quote_token_address, yes_pair_key, no_pair_key, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(market_hash_str)
        .bind(asset_str)
        .bind(quote_str)
        .bind(yes_pair_key)
        .bind(no_pair_key)
        .bind(created_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get orderbook data for a specific market
    pub async fn get_orderbooks_for_market(
        &self,
        market_hash: &str,
    ) -> anyhow::Result<Vec<OrderbookData>> {
        let orderbook_data = sqlx::query_as::<_, OrderbookData>(
            r#"
            SELECT market_hash, asset_address, quote_token_address, yes_pair_key, no_pair_key, created_at
            FROM orderbook_markets
            WHERE market_hash = ?
            ORDER BY created_at ASC
            "#,
        )
        .bind(market_hash)
        .fetch_all(&self.pool)
        .await?;

        Ok(orderbook_data)
    }
}
