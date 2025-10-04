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
            SELECT market_hash, question_hash, resolution_deadline, oracle, block_number, indexed_at
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
            SELECT market_hash, question_hash, resolution_deadline, oracle, block_number, indexed_at
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
            SELECT market_hash, question_hash, resolution_deadline, oracle, block_number, indexed_at
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
}
