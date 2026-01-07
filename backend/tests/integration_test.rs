//! Integration tests for the backend API
//!
//! Run with: make test (requires orchestrator)
//!
//! These tests require the full stack to be running:
//! - Tempo node
//! - Deployed contracts
//! - Backend server

use anyhow::Result;
use serde_json::{json, Value};
use std::time::Duration;
use tokio::time::sleep;

/// Test environment that requires the orchestrator to be running.
/// Run tests via: `make test` or `bun run test`
struct TestEnvironment {
    #[allow(dead_code)]
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

    /// Fetch markets with custom query parameters
    async fn get_markets_with_params(&self, limit: i64, offset: i64) -> Result<(u16, Value)> {
        let client = reqwest::Client::new();

        let response = client
            .get(format!(
                "{}/markets?limit={}&offset={}",
                self.server_url, limit, offset
            ))
            .send()
            .await?;

        let status = response.status().as_u16();
        let body = response.text().await?;

        Ok((status, serde_json::from_str(&body)?))
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

    /// Fetch a single market by hash (returns status code)
    async fn get_market_raw(&self, market_hash: &str) -> Result<(u16, String)> {
        let client = reqwest::Client::new();

        let response = client
            .get(format!("{}/markets/{}", self.server_url, market_hash))
            .send()
            .await?;

        let status = response.status().as_u16();
        let body = response.text().await?;

        Ok((status, body))
    }

    /// Fetch verse tokens for a market
    async fn get_verse_tokens(&self, market_hash: &str) -> Result<(u16, String)> {
        let client = reqwest::Client::new();

        let response = client
            .get(format!(
                "{}/markets/{}/verse-tokens",
                self.server_url, market_hash
            ))
            .send()
            .await?;

        let status = response.status().as_u16();
        let body = response.text().await?;

        Ok((status, body))
    }

    /// Try to open a market without authentication
    async fn open_market_no_auth(&self, question: &str, resolution_deadline: u64) -> Result<u16> {
        let client = reqwest::Client::new();

        let response = client
            .post(format!("{}/admin/markets/open", self.server_url))
            .header("Content-Type", "application/json")
            .json(&json!({
                "question": question,
                "resolutionDeadline": resolution_deadline,
                "assets": [],
            }))
            .send()
            .await?;

        Ok(response.status().as_u16())
    }

    /// Try to open a market with invalid auth
    async fn open_market_invalid_auth(
        &self,
        question: &str,
        resolution_deadline: u64,
    ) -> Result<u16> {
        let client = reqwest::Client::new();

        let response = client
            .post(format!("{}/admin/markets/open", self.server_url))
            .header("Authorization", "Bearer invalid_api_key_12345")
            .header("Content-Type", "application/json")
            .json(&json!({
                "question": question,
                "resolutionDeadline": resolution_deadline,
                "assets": [],
            }))
            .send()
            .await?;

        Ok(response.status().as_u16())
    }

    /// Try to open a market with Basic auth instead of Bearer
    async fn open_market_wrong_auth_type(
        &self,
        question: &str,
        resolution_deadline: u64,
    ) -> Result<u16> {
        let client = reqwest::Client::new();

        let response = client
            .post(format!("{}/admin/markets/open", self.server_url))
            .header("Authorization", "Basic dXNlcjpwYXNz")
            .header("Content-Type", "application/json")
            .json(&json!({
                "question": question,
                "resolutionDeadline": resolution_deadline,
                "assets": [],
            }))
            .send()
            .await?;

        Ok(response.status().as_u16())
    }
}

async fn wait_for_server(env: &TestEnvironment) -> Result<()> {
    for i in 0..10 {
        if env.check_server_health().await {
            println!("Server is ready");
            return Ok(());
        }
        if i == 9 {
            anyhow::bail!("Server not responding after 10 attempts");
        }
        sleep(Duration::from_secs(1)).await;
    }
    Ok(())
}

// ============================================================================
// CORE API TESTS
// ============================================================================

#[tokio::test]
#[ignore] // Run with: make test
async fn test_full_flow() -> Result<()> {
    println!("\n=== Starting Integration Test ===\n");

    let env = TestEnvironment::new()?;
    wait_for_server(&env).await?;

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
    let status = env.open_market_no_auth("This should fail", resolution_deadline).await?;
    assert_eq!(status, 401);
    println!("✓ Authentication works correctly");

    println!("\n=== All Integration Tests Passed ===\n");

    Ok(())
}

#[tokio::test]
#[ignore] // Run with: make test
async fn test_market_with_verse_tokens() -> Result<()> {
    println!("\n=== Testing Market with Verse Tokens ===\n");

    let env = TestEnvironment::new()?;
    wait_for_server(&env).await?;

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

// ============================================================================
// PAGINATION TESTS
// ============================================================================

#[tokio::test]
#[ignore] // Run with: make test
async fn test_pagination() -> Result<()> {
    println!("\n=== Testing Pagination ===\n");

    let env = TestEnvironment::new()?;
    wait_for_server(&env).await?;

    // Test default pagination
    println!("Test 1: Default pagination...");
    let (status, response) = env.get_markets_with_params(50, 0).await?;
    assert_eq!(status, 200);
    assert!(response["markets"].is_array());
    assert!(response["count"].is_number());
    assert!(response["total"].is_number());
    println!("✓ Default pagination works");

    // Test limit=0
    println!("\nTest 2: Limit=0...");
    let (status, response) = env.get_markets_with_params(0, 0).await?;
    assert_eq!(status, 200);
    assert_eq!(response["markets"].as_array().unwrap().len(), 0);
    println!("✓ Limit=0 returns empty array");

    // Test limit > 100 (should be capped)
    println!("\nTest 3: Limit=200 (should be capped to 100)...");
    let (status, response) = env.get_markets_with_params(200, 0).await?;
    assert_eq!(status, 200);
    let count = response["markets"].as_array().unwrap().len();
    assert!(count <= 100);
    println!("✓ Limit capped correctly");

    // Test large offset (beyond data)
    println!("\nTest 4: Large offset beyond data...");
    let (status, response) = env.get_markets_with_params(50, 1000000).await?;
    assert_eq!(status, 200);
    assert_eq!(response["markets"].as_array().unwrap().len(), 0);
    println!("✓ Large offset returns empty array");

    println!("\n=== Pagination Tests Passed ===\n");

    Ok(())
}

// ============================================================================
// ERROR HANDLING TESTS
// ============================================================================

#[tokio::test]
#[ignore] // Run with: make test
async fn test_market_not_found() -> Result<()> {
    println!("\n=== Testing Market Not Found ===\n");

    let env = TestEnvironment::new()?;
    wait_for_server(&env).await?;

    // Test non-existent market hash
    let fake_hash = "0x0000000000000000000000000000000000000000000000000000000000000000";
    let (status, body) = env.get_market_raw(fake_hash).await?;
    assert_eq!(status, 404);
    let response: Value = serde_json::from_str(&body)?;
    assert!(response["error"].is_string());
    println!("✓ Non-existent market returns 404");

    Ok(())
}

#[tokio::test]
#[ignore] // Run with: make test
async fn test_verse_tokens_endpoint() -> Result<()> {
    println!("\n=== Testing Verse Tokens Endpoint ===\n");

    let env = TestEnvironment::new()?;
    wait_for_server(&env).await?;

    // First create a market
    let resolution_deadline = (chrono::Utc::now().timestamp() + 86400 * 30) as u64;
    let result = env
        .open_market("Test verse tokens endpoint?", resolution_deadline, vec![])
        .await?;

    let market_hash = result["marketHash"].as_str().unwrap();

    // Wait for indexer
    sleep(Duration::from_secs(2)).await;

    // Test verse tokens endpoint
    println!("Test 1: Fetching verse tokens for market...");
    let (status, body) = env.get_verse_tokens(market_hash).await?;
    assert_eq!(status, 200);
    let tokens: Value = serde_json::from_str(&body)?;
    assert!(tokens.is_array());
    println!("✓ Verse tokens endpoint works");

    // Test 404 for non-existent market
    println!("\nTest 2: Verse tokens for non-existent market...");
    let fake_hash = "0x0000000000000000000000000000000000000000000000000000000000000000";
    let (status, _) = env.get_verse_tokens(fake_hash).await?;
    assert_eq!(status, 404);
    println!("✓ Non-existent market returns 404");

    println!("\n=== Verse Tokens Endpoint Tests Passed ===\n");

    Ok(())
}

// ============================================================================
// AUTHENTICATION TESTS
// ============================================================================

#[tokio::test]
#[ignore] // Run with: make test
async fn test_authentication_comprehensive() -> Result<()> {
    println!("\n=== Testing Authentication ===\n");

    let env = TestEnvironment::new()?;
    wait_for_server(&env).await?;

    let resolution_deadline = (chrono::Utc::now().timestamp() + 86400 * 30) as u64;

    // Test 1: No auth header
    println!("Test 1: No Authorization header...");
    let status = env.open_market_no_auth("Test question?", resolution_deadline).await?;
    assert_eq!(status, 401);
    println!("✓ Missing auth returns 401");

    // Test 2: Invalid API key
    println!("\nTest 2: Invalid API key...");
    let status = env
        .open_market_invalid_auth("Test question?", resolution_deadline)
        .await?;
    assert_eq!(status, 401);
    println!("✓ Invalid API key returns 401");

    // Test 3: Wrong auth type (Basic instead of Bearer)
    println!("\nTest 3: Wrong auth type (Basic)...");
    let status = env
        .open_market_wrong_auth_type("Test question?", resolution_deadline)
        .await?;
    assert_eq!(status, 401);
    println!("✓ Wrong auth type returns 401");

    // Test 4: Valid auth works
    println!("\nTest 4: Valid authentication...");
    let result = env
        .open_market("Valid auth test question?", resolution_deadline, vec![])
        .await;
    assert!(result.is_ok());
    println!("✓ Valid auth works");

    println!("\n=== Authentication Tests Passed ===\n");

    Ok(())
}

// ============================================================================
// VALIDATION TESTS
// ============================================================================

#[tokio::test]
#[ignore] // Run with: make test
async fn test_admin_validation() -> Result<()> {
    println!("\n=== Testing Admin Request Validation ===\n");

    let env = TestEnvironment::new()?;
    wait_for_server(&env).await?;

    let client = reqwest::Client::new();

    // Test 1: Empty question
    println!("Test 1: Empty question...");
    let response = client
        .post(format!("{}/admin/markets/open", env.server_url))
        .header("Authorization", format!("Bearer {}", env.api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "question": "",
            "resolutionDeadline": u32::MAX,
            "assets": [],
        }))
        .send()
        .await?;
    assert_eq!(response.status().as_u16(), 400);
    println!("✓ Empty question returns 400");

    // Test 2: Short question
    println!("\nTest 2: Short question (< 10 chars)...");
    let response = client
        .post(format!("{}/admin/markets/open", env.server_url))
        .header("Authorization", format!("Bearer {}", env.api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "question": "Short?",
            "resolutionDeadline": u32::MAX,
            "assets": [],
        }))
        .send()
        .await?;
    assert_eq!(response.status().as_u16(), 400);
    let body: Value = response.json().await?;
    assert!(body["error"].as_str().unwrap().contains("10 characters"));
    println!("✓ Short question returns 400");

    // Test 3: Past deadline
    println!("\nTest 3: Past deadline...");
    let response = client
        .post(format!("{}/admin/markets/open", env.server_url))
        .header("Authorization", format!("Bearer {}", env.api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "question": "Will this fail with past deadline?",
            "resolutionDeadline": 1000,
            "assets": [],
        }))
        .send()
        .await?;
    assert_eq!(response.status().as_u16(), 400);
    let body: Value = response.json().await?;
    assert!(body["error"].as_str().unwrap().contains("future"));
    println!("✓ Past deadline returns 400");

    // Test 4: Invalid asset address
    println!("\nTest 4: Invalid asset address...");
    let response = client
        .post(format!("{}/admin/markets/open", env.server_url))
        .header("Authorization", format!("Bearer {}", env.api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "question": "Test with invalid asset address?",
            "resolutionDeadline": u32::MAX,
            "assets": ["not_an_address"],
        }))
        .send()
        .await?;
    assert_eq!(response.status().as_u16(), 400);
    let body: Value = response.json().await?;
    assert!(body["error"].as_str().unwrap().contains("Invalid asset"));
    println!("✓ Invalid asset returns 400");

    println!("\n=== Admin Validation Tests Passed ===\n");

    Ok(())
}

// ============================================================================
// RESPONSE FORMAT TESTS
// ============================================================================

#[tokio::test]
#[ignore] // Run with: make test
async fn test_response_format() -> Result<()> {
    println!("\n=== Testing Response Format ===\n");

    let env = TestEnvironment::new()?;
    wait_for_server(&env).await?;

    // Create a market first
    let resolution_deadline = (chrono::Utc::now().timestamp() + 86400 * 30) as u64;
    let result = env
        .open_market("Response format test?", resolution_deadline, vec![])
        .await?;

    let market_hash = result["marketHash"].as_str().unwrap();
    sleep(Duration::from_secs(2)).await;

    // Test list response format
    println!("Test 1: Verify list response format...");
    let markets = env.get_markets().await?;

    // Check required fields
    assert!(markets["markets"].is_array());
    assert!(markets["count"].is_number());
    assert!(markets["total"].is_number());

    // Check market fields use camelCase
    if let Some(market) = markets["markets"].as_array().unwrap().first() {
        assert!(market["marketHash"].is_string());
        assert!(market["questionHash"].is_string());
        assert!(market["resolutionDeadline"].is_number());
        assert!(market["blockNumber"].is_number());
        // In list view, probabilities should be null
        assert!(market["yesProbability"].is_null());
        assert!(market["noProbability"].is_null());
    }
    println!("✓ List response format correct");

    // Test detail response format
    println!("\nTest 2: Verify detail response format...");
    let detail = env.get_market(market_hash).await?;

    assert!(detail["marketHash"].is_string());
    assert!(detail["questionHash"].is_string());
    assert!(detail["resolutionDeadline"].is_number());
    assert!(detail["blockNumber"].is_number());
    assert!(detail["verseTokens"].is_array());
    assert!(detail["orderbooks"].is_array());
    // Resolution should be present in detail view
    assert!(detail["resolution"].is_string() || detail["resolution"].is_null());
    println!("✓ Detail response format correct");

    println!("\n=== Response Format Tests Passed ===\n");

    Ok(())
}
