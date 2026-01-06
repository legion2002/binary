use anyhow::Result;
use serde_json::{json, Value};
use std::time::Duration;
use tokio::time::sleep;

/// Test environment that requires the orchestrator to be running.
/// Run tests via: `make test` or `bun run test`
struct TestEnvironment {
    rpc_url: String,
    server_url: String,
    api_key: String,
}

impl TestEnvironment {
    /// Create environment from env vars set by the orchestrator (script/env.ts)
    fn new() -> Result<Self> {
        let rpc_url = std::env::var("RPC_URL")
            .expect("RPC_URL must be set. Run tests via: make test");
        let server_url = std::env::var("SERVER_URL")
            .expect("SERVER_URL must be set. Run tests via: make test");
        let api_key = std::env::var("ADMIN_API_KEY")
            .unwrap_or_else(|_| "test_api_key_12345".to_string());

        println!("Test environment:");
        println!("  RPC_URL: {}", rpc_url);
        println!("  SERVER_URL: {}", server_url);

        Ok(Self {
            rpc_url,
            server_url,
            api_key,
        })
    }

    async fn check_server_health(&self) -> bool {
        reqwest::get(format!("{}/markets", self.server_url))
            .await
            .is_ok()
    }

    /// Open a market via the admin API
    async fn open_market(
        &self,
        question: &str,
        resolution_deadline: u64,
        assets: Vec<&str>,
    ) -> Result<Value> {
        let client = reqwest::Client::new();

        let response = client
            .post(format!("{}/admin/markets/open", self.server_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "question": question,
                "resolutionDeadline": resolution_deadline,
                "assets": assets,
            }))
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("Failed to open market: {} - {}", status, body);
        }

        Ok(serde_json::from_str(&body)?)
    }

    /// Fetch markets from the public API
    async fn get_markets(&self) -> Result<Value> {
        let client = reqwest::Client::new();

        let response = client
            .get(format!("{}/markets", self.server_url))
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("Failed to get markets: {} - {}", status, body);
        }

        Ok(serde_json::from_str(&body)?)
    }

    /// Fetch a single market by hash
    async fn get_market(&self, market_hash: &str) -> Result<Value> {
        let client = reqwest::Client::new();

        let response = client
            .get(format!("{}/markets/{}", self.server_url, market_hash))
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("Failed to get market: {} - {}", status, body);
        }

        Ok(serde_json::from_str(&body)?)
    }
}

#[tokio::test]
#[ignore] // Run with: make test
async fn test_full_flow() -> Result<()> {
    println!("\n=== Starting Integration Test ===\n");

    let env = TestEnvironment::new()?;

    // Wait for server to be ready
    println!("Waiting for server...");
    for i in 0..10 {
        if env.check_server_health().await {
            println!("Server is ready");
            break;
        }
        if i == 9 {
            anyhow::bail!("Server not responding after 10 attempts");
        }
        sleep(Duration::from_secs(1)).await;
    }

    // Test 1: Get markets (may have seeded markets from deploy script)
    println!("\nTest 1: Fetching initial markets...");
    let markets = env.get_markets().await?;
    let initial_count = markets["markets"].as_array().unwrap().len();
    println!("✓ Found {} existing markets", initial_count);

    // Test 2: Open a market
    println!("\nTest 2: Opening a new market...");
    let resolution_deadline = (chrono::Utc::now().timestamp() + 86400 * 30) as u64;
    let result = env
        .open_market(
            "Will ETH price be above $5000 by end of 2025?",
            resolution_deadline,
            vec![],
        )
        .await?;

    let market_hash = result["marketHash"].as_str().unwrap();
    println!("Market opened: {}", market_hash);
    assert!(result["transactionHash"].is_string());
    println!("✓ Market opened successfully");

    // Wait for indexer to catch up
    sleep(Duration::from_secs(3)).await;

    // Test 3: Get markets again (should have 1 more)
    println!("\nTest 3: Fetching markets after opening...");
    let markets = env.get_markets().await?;
    let new_count = markets["markets"].as_array().unwrap().len();
    assert_eq!(new_count, initial_count + 1);
    println!("✓ Market count increased to {}", new_count);

    // Test 4: Get single market detail
    println!("\nTest 4: Fetching market detail...");
    let market_detail = env.get_market(market_hash).await?;
    assert_eq!(market_detail["marketHash"].as_str().unwrap(), market_hash);
    println!("✓ Market detail fetched successfully");

    // Test 5: Open another market
    println!("\nTest 5: Opening a second market...");
    let result = env
        .open_market(
            "Will Bitcoin reach $100k in 2025?",
            resolution_deadline,
            vec![],
        )
        .await?;

    assert!(result["marketHash"].is_string());
    println!("✓ Second market opened successfully");

    // Wait for indexer
    sleep(Duration::from_secs(3)).await;

    // Test 6: Verify both new markets are returned
    println!("\nTest 6: Verifying all markets...");
    let markets = env.get_markets().await?;
    let final_count = markets["markets"].as_array().unwrap().len();
    assert_eq!(final_count, initial_count + 2);
    println!("✓ All {} markets are indexed", final_count);

    // Test 7: Test authentication (try without API key)
    println!("\nTest 7: Testing authentication...");
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/admin/markets/open", env.server_url))
        .header("Content-Type", "application/json")
        .json(&json!({
            "question": "This should fail",
            "resolutionDeadline": resolution_deadline,
            "assets": [],
        }))
        .send()
        .await?;

    assert_eq!(response.status(), 401);
    println!("✓ Authentication works correctly");

    println!("\n=== All Integration Tests Passed ===\n");

    Ok(())
}

#[tokio::test]
#[ignore] // Run with: make test
async fn test_market_with_verse_tokens() -> Result<()> {
    println!("\n=== Testing Market with Verse Tokens ===\n");

    let env = TestEnvironment::new()?;

    // Wait for server
    for _ in 0..10 {
        if env.check_server_health().await {
            break;
        }
        sleep(Duration::from_secs(1)).await;
    }

    // Use PathUSD as the asset (available on Tempo)
    let path_usd = "0x20C0000000000000000000000000000000000000";

    println!("Test 1: Opening market with PathUSD asset...");
    let resolution_deadline = (chrono::Utc::now().timestamp() + 86400 * 30) as u64;

    let result = env
        .open_market(
            "Will PathUSD TVL exceed $1B?",
            resolution_deadline,
            vec![path_usd],
        )
        .await?;

    println!("Result: {}", serde_json::to_string_pretty(&result)?);
    
    let verse_tokens = result["verseTokens"].as_array().unwrap();
    assert_eq!(verse_tokens.len(), 1);
    
    let yes_verse = verse_tokens[0]["yesVerse"].as_str().unwrap();
    let no_verse = verse_tokens[0]["noVerse"].as_str().unwrap();
    
    assert!(yes_verse.starts_with("0x"));
    assert!(no_verse.starts_with("0x"));
    assert_ne!(yes_verse, no_verse);
    
    println!("  YES verse: {}", yes_verse);
    println!("  NO verse: {}", no_verse);
    println!("✓ Market with verse tokens created");

    println!("\n=== Verse Token Tests Passed ===\n");

    Ok(())
}
