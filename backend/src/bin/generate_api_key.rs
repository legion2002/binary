use bcrypt::{hash, DEFAULT_COST};
use rand::Rng;

fn main() {
    println!("=== Admin API Key Generator ===");
    println!();

    // Generate a random 32-byte API key (64 hex characters)
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    let api_key = hex::encode(&random_bytes);

    println!("Generated API Key:");
    println!("{}", api_key);
    println!();

    // Hash it with bcrypt
    println!("Generating bcrypt hash...");
    let hash = hash(&api_key, DEFAULT_COST).expect("Failed to hash API key");

    println!();
    println!("Bcrypt Hash:");
    println!("{}", hash);
    println!();
    println!("=== Setup Instructions ===");
    println!();
    println!("1. Add this to your .env file:");
    println!("   ADMIN_API_KEY_HASH={}", hash);
    println!();
    println!("2. Give this API key to authorized users:");
    println!("   {}", api_key);
    println!();
    println!("3. Users should send requests with this header:");
    println!("   Authorization: Bearer {}", api_key);
    println!();
    println!("=== Test the API ===");
    println!("curl -X POST http://127.0.0.1:3000/admin/markets/open \\");
    println!("  -H 'Authorization: Bearer {}' \\", api_key);
    println!("  -H 'Content-Type: application/json' \\");
    println!("  -d '{{");
    println!("    \"question\": \"Will FOCIL be included in Ethereum by end of 2025?\",");
    println!("    \"resolutionDeadline\": 1767225600,");
    println!("    \"assets\": [\"0x4200000000000000000000000000000000000006\"]");
    println!("  }}'");
    println!();
}
