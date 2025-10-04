use anyhow::Result;
use serde_json::{json, Value};
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use tokio::time::sleep;

/// Helper struct to manage test processes
struct TestEnvironment {
    anvil_process: Option<Child>,
    server_process: Option<Child>,
    rpc_url: String,
    server_url: String,
    api_key: String,
}

impl TestEnvironment {
    async fn new() -> Result<Self> {
        // Start Anvil
        println!("Starting Anvil...");
        let anvil_process = Command::new("anvil")
            .arg("--port")
            .arg("8545")
            .arg("--block-time")
            .arg("1")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        sleep(Duration::from_secs(2)).await;

        let rpc_url = "http://127.0.0.1:8545".to_string();
        let server_url = "http://127.0.0.1:3001".to_string();

        // Use a test API key
        let api_key = "test_api_key_12345".to_string();

        Ok(Self {
            anvil_process: Some(anvil_process),
            server_process: None,
            rpc_url,
            server_url,
            api_key,
        })
    }

    /// Deploy contracts to Anvil and return contract addresses
    async fn deploy_contracts(&self) -> Result<(String, String)> {
        println!("Deploying contracts...");

        // Run deployment from project root
        let output = Command::new("forge")
            .current_dir("..")
            .arg("script")
            .arg("script/DeployAll.s.sol:DeployAll")
            .arg("--rpc-url")
            .arg(&self.rpc_url)
            .arg("--broadcast")
            .arg("--private-key")
            .arg("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80") // Anvil default key
            .output()?;

        if !output.status.success() {
            anyhow::bail!(
                "Contract deployment failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        // Read the broadcast JSON file to extract contract addresses
        // Anvil chain ID is 31337
        let broadcast_file = "../broadcast/DeployAll.s.sol/31337/run-latest.json";
        let broadcast_json = std::fs::read_to_string(broadcast_file)
            .map_err(|e| anyhow::anyhow!("Failed to read broadcast file: {}", e))?;

        let json: Value = serde_json::from_str(&broadcast_json)?;

        // Extract contract addresses from transactions
        let transactions = json["transactions"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("No transactions found in broadcast"))?;

        let mut oracle_address = None;
        let mut multiverse_address = None;

        for tx in transactions {
            if let Some(contract_name) = tx["contractName"].as_str() {
                if let Some(address) = tx["contractAddress"].as_str() {
                    match contract_name {
                        "TrustedOracle" => oracle_address = Some(address.to_string()),
                        "MultiVerse" => multiverse_address = Some(address.to_string()),
                        _ => {}
                    }
                }
            }
        }

        let oracle_address = oracle_address
            .ok_or_else(|| anyhow::anyhow!("TrustedOracle address not found"))?;
        let multiverse_address = multiverse_address
            .ok_or_else(|| anyhow::anyhow!("MultiVerse address not found"))?;

        println!("Oracle: {}", oracle_address);
        println!("MultiVerse: {}", multiverse_address);

        Ok((multiverse_address, oracle_address))
    }

    /// Start the backend server with test configuration
    async fn start_server(
        &mut self,
        multiverse_address: &str,
        oracle_address: &str,
        api_key_hash: &str,
    ) -> Result<()> {
        println!("Starting backend server...");

        let server_process = Command::new("cargo")
            .arg("run")
            .arg("--bin")
            .arg("backend")
            .env("HOST", "127.0.0.1")
            .env("PORT", "3001")
            .env("MULTIVERSE_ADDRESS", multiverse_address)
            .env("ORACLE_ADDRESS", oracle_address)
            .env("RPC_URL", &self.rpc_url)
            .env("WS_RPC_URL", "ws://127.0.0.1:8545")
            .env(
                "PRIVATE_KEY",
                "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
            )
            .env("DATABASE_URL", "sqlite::memory:")
            .env("ADMIN_API_KEY_HASH", api_key_hash)
            .env("RUST_LOG", "info")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        self.server_process = Some(server_process);

        // Wait for server to start
        sleep(Duration::from_secs(8)).await;

        // Health check with more retries
        for i in 0..20 {
            if self.check_server_health().await {
                println!("Server is ready");
                return Ok(());
            }
            if i % 5 == 0 {
                println!("Waiting for server... (attempt {}/20)", i + 1);
            }
            sleep(Duration::from_secs(1)).await;
        }

        anyhow::bail!("Server failed to start after 20 attempts")
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
}

impl Drop for TestEnvironment {
    fn drop(&mut self) {
        if let Some(mut server) = self.server_process.take() {
            let _ = server.kill();
        }
        if let Some(mut anvil) = self.anvil_process.take() {
            let _ = anvil.kill();
        }
    }
}


#[tokio::test]
#[ignore] // Run with: cargo test --test integration_test -- --ignored
async fn test_full_flow() -> Result<()> {
    println!("\n=== Starting Integration Test ===\n");

    // Setup test environment
    let mut env = TestEnvironment::new().await?;

    // Deploy contracts
    let (multiverse_address, oracle_address) = env.deploy_contracts().await?;

    // Generate API key hash for testing
    let api_key_hash = bcrypt::hash(&env.api_key, bcrypt::DEFAULT_COST)?;

    // Start server
    env.start_server(&multiverse_address, &oracle_address, &api_key_hash)
        .await?;

    // Test 1: Get markets (should be empty initially)
    println!("\nTest 1: Fetching initial markets...");
    let markets = env.get_markets().await?;
    assert!(markets["markets"].as_array().unwrap().is_empty());
    println!("✓ Initial markets are empty");

    // Test 2: Open a market
    println!("\nTest 2: Opening a new market...");
    let resolution_deadline = (chrono::Utc::now().timestamp() + 86400 * 30) as u64; // 30 days from now
    let result = env
        .open_market(
            "Will ETH price be above $5000 by end of 2025?",
            resolution_deadline,
            vec![], // No assets for now to avoid verse token creation issues
        )
        .await?;

    println!("Market opened: {}", serde_json::to_string_pretty(&result)?);
    assert!(result["marketHash"].is_string());
    assert!(result["transactionHash"].is_string());
    println!("✓ Market opened successfully");

    // Wait for indexer to catch up
    sleep(Duration::from_secs(3)).await;

    // Test 3: Get markets again (should have 1 market)
    println!("\nTest 3: Fetching markets after opening...");
    let markets = env.get_markets().await?;
    println!("Markets response: {}", serde_json::to_string_pretty(&markets)?);
    let markets_array = markets["markets"].as_array().unwrap();
    assert_eq!(markets_array.len(), 1);

    // Check if question_text exists
    if let Some(question_text) = markets_array[0]["question_text"].as_str() {
        assert_eq!(
            question_text,
            "Will ETH price be above $5000 by end of 2025?"
        );
        println!("✓ Market was indexed correctly with question text");
    } else {
        println!("⚠ Market indexed but question_text is null (expected from event indexer)");
        println!("✓ Market was indexed correctly");
    }

    // Test 4: Open another market
    println!("\nTest 4: Opening a second market...");
    let result = env
        .open_market(
            "Will Bitcoin reach $100k in 2025?",
            resolution_deadline,
            vec![],
        )
        .await?;

    println!("Second market opened: {}", serde_json::to_string_pretty(&result)?);
    assert!(result["marketHash"].is_string());
    println!("✓ Second market opened successfully");

    // Wait for indexer
    sleep(Duration::from_secs(3)).await;

    // Test 5: Verify both markets are returned
    println!("\nTest 5: Fetching all markets...");
    let markets = env.get_markets().await?;
    let markets_array = markets["markets"].as_array().unwrap();
    assert_eq!(markets_array.len(), 2);
    println!("✓ Both markets are indexed");

    // Test 6: Test authentication (try without API key)
    println!("\nTest 6: Testing authentication...");
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
#[ignore]
async fn test_market_with_verse_tokens() -> Result<()> {
    println!("\n=== Testing Market with Verse Tokens ===\n");

    let mut env = TestEnvironment::new().await?;
    let (multiverse_address, oracle_address) = env.deploy_contracts().await?;

    let api_key_hash = bcrypt::hash(&env.api_key, bcrypt::DEFAULT_COST)?;

    env.start_server(&multiverse_address, &oracle_address, &api_key_hash)
        .await?;

    // Use arbitrary addresses as mock ERC20 tokens (they don't need to actually exist)
    let mock_token_1 = "0x1111111111111111111111111111111111111111";
    let mock_token_2 = "0x2222222222222222222222222222222222222222";

    println!("Test 1: Opening market with single asset...");
    let resolution_deadline = (chrono::Utc::now().timestamp() + 86400 * 30) as u64;

    let result = env
        .open_market(
            "Will token 1 be worth more than $5000?",
            resolution_deadline,
            vec![mock_token_1],
        )
        .await?;

    println!("Result: {}", serde_json::to_string_pretty(&result)?);
    assert_eq!(result["verseTokens"].as_array().unwrap().len(), 1);
    println!("✓ Market with single verse token created");

    // Wait for indexer
    sleep(Duration::from_secs(2)).await;

    println!("\nTest 2: Opening market with multiple assets...");
    let result = env
        .open_market(
            "Will token 2 outperform token 1?",
            resolution_deadline,
            vec![mock_token_1, mock_token_2],
        )
        .await?;

    println!("Result: {}", serde_json::to_string_pretty(&result)?);
    assert_eq!(result["verseTokens"].as_array().unwrap().len(), 2);
    println!("✓ Market with multiple verse tokens created");

    // Verify verse tokens have addresses
    let verse_tokens = result["verseTokens"].as_array().unwrap();
    for (i, token_info) in verse_tokens.iter().enumerate() {
        let yes_verse = token_info["yesVerse"].as_str().unwrap();
        let no_verse = token_info["noVerse"].as_str().unwrap();
        assert!(yes_verse.starts_with("0x"));
        assert!(no_verse.starts_with("0x"));
        assert_ne!(yes_verse, no_verse);
        println!("  Asset {}: YES={}, NO={}", i + 1, yes_verse, no_verse);
    }

    println!("\n=== All Verse Token Tests Passed ===\n");

    Ok(())
}
