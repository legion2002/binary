use alloy::primitives::Address;
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::sync::Arc;

use crate::council::CouncilSettings;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainDeployment {
    pub name: String,
    pub multiverse: String,
    pub oracle: String,
    pub uni_v2_factory: String,
    pub uni_v2_router: String,
    /// CouncilOracle address, when deployed
    #[serde(default)]
    pub council_oracle: Option<String>,
}

#[derive(Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub chain_id: u64,
    pub multiverse_address: Address,
    /// Oracle new markets are opened with: TrustedOracle, or CouncilOracle when MARKET_ORACLE=council
    pub oracle_address: Address,
    /// Council settings when MARKET_ORACLE=council
    pub council: Option<Arc<CouncilSettings>>,
    pub univ2_factory_address: Address,
    pub univ2_router_address: Address,
    pub provider: DynProvider,
    pub signer: PrivateKeySigner,
    pub admin_api_key_hash: String,
}

impl Config {
    pub async fn from_env() -> anyhow::Result<Self> {
        // Log RPC_URL before dotenv loading to debug orchestrator env passing
        let rpc_before = env::var("RPC_URL").ok();
        tracing::info!("RPC_URL before dotenv: {:?}", rpc_before);

        // Note: dotenvy::dotenv() does NOT override existing env vars by default
        dotenvy::dotenv().ok();

        let rpc_after = env::var("RPC_URL").ok();
        tracing::info!("RPC_URL after dotenv: {:?}", rpc_after);

        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .expect("PORT must be a valid u16");

        let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");

        // Get chain ID from RPC
        let provider = ProviderBuilder::new().connect(&rpc_url).await?.erased();
        let chain_id = provider.get_chain_id().await?;
        tracing::info!("Connected to chain ID: {}", chain_id);

        // Load deployment addresses
        // Priority: env vars (for local dev) > deployments.json (for production)
        let (multiverse_address, oracle_address, univ2_factory_address, univ2_router_address, deployed_council_oracle) =
            if let (Ok(mv), Ok(or), Ok(uf), Ok(ur)) = (
                env::var("MULTIVERSE_ADDRESS"),
                env::var("ORACLE_ADDRESS"),
                env::var("UNIV2_FACTORY_ADDRESS"),
                env::var("UNIV2_ROUTER_ADDRESS"),
            ) {
                tracing::info!("Using addresses from environment variables");
                (
                    mv.parse().expect("Invalid MULTIVERSE_ADDRESS"),
                    or.parse().expect("Invalid ORACLE_ADDRESS"),
                    uf.parse().expect("Invalid UNIV2_FACTORY_ADDRESS"),
                    ur.parse().expect("Invalid UNIV2_ROUTER_ADDRESS"),
                    None,
                )
            } else {
                // Fall back to deployments.json
                let deployments_path = env::var("DEPLOYMENTS_PATH")
                    .unwrap_or_else(|_| "deployments.json".to_string());
                let deployments_content = fs::read_to_string(&deployments_path)
                    .unwrap_or_else(|_| panic!("Failed to read deployments file: {}", deployments_path));
                let deployments: HashMap<String, ChainDeployment> = serde_json::from_str(&deployments_content)
                    .expect("Failed to parse deployments.json");

                let deployment = deployments
                    .get(&chain_id.to_string())
                    .unwrap_or_else(|| panic!("No deployment found for chain ID {}", chain_id));

                tracing::info!("Using deployment from deployments.json for {}: {:?}", deployment.name, deployment);

                (
                    deployment.multiverse.parse().expect("Invalid multiverse in deployments.json"),
                    deployment.oracle.parse().expect("Invalid oracle in deployments.json"),
                    deployment.uni_v2_factory.parse().expect("Invalid uniV2Factory in deployments.json"),
                    deployment.uni_v2_router.parse().expect("Invalid uniV2Router in deployments.json"),
                    deployment.council_oracle.clone().filter(|s| !s.is_empty()),
                )
            };

        // Oracle selection: MARKET_ORACLE=council opens markets with the CouncilOracle and
        // resolves them through the AI council; anything else keeps the TrustedOracle.
        let market_oracle = env::var("MARKET_ORACLE").unwrap_or_else(|_| "trusted".to_string());
        let (oracle_address, council) = match market_oracle.as_str() {
            "council" => {
                let address: Address = env::var("COUNCIL_ORACLE_ADDRESS")
                    .ok()
                    .or(deployed_council_oracle)
                    .expect("MARKET_ORACLE=council needs COUNCIL_ORACLE_ADDRESS (or councilOracle in deployments.json)")
                    .parse()
                    .expect("Invalid COUNCIL_ORACLE_ADDRESS");
                let settings = CouncilSettings::from_env(address)?;
                tracing::info!(
                    "Council oracle {:?}, council {}, {} pinned signer(s)",
                    address,
                    crate::council::hex32(settings.council_hash),
                    settings.signers.len()
                );
                (address, Some(Arc::new(settings)))
            }
            "trusted" => (oracle_address, None),
            other => anyhow::bail!("MARKET_ORACLE must be 'trusted' or 'council', got {other:?}"),
        };

        // Load private key for signing transactions
        let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
        let signer = private_key
            .parse::<PrivateKeySigner>()
            .expect("PRIVATE_KEY must be a valid private key");

        // Load admin API key hash for authentication
        let admin_api_key_hash =
            env::var("ADMIN_API_KEY_HASH").expect("ADMIN_API_KEY_HASH must be set");

        Ok(Self {
            host,
            port,
            chain_id,
            multiverse_address,
            oracle_address,
            council,
            univ2_factory_address,
            univ2_router_address,
            provider,
            signer,
            admin_api_key_hash,
        })
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
