use alloy::network::EthereumWallet;
use alloy::primitives::Address;
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub multiverse_address: Address,
    pub oracle_address: Address,
    pub provider: alloy::providers::ReqwestProvider,
}

impl Config {
    pub async fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .expect("PORT must be a valid u16");

        let multiverse_address = env::var("MULTIVERSE_ADDRESS")
            .expect("MULTIVERSE_ADDRESS must be set")
            .parse()
            .expect("MULTIVERSE_ADDRESS must be a valid Ethereum address");

        let oracle_address = env::var("ORACLE_ADDRESS")
            .expect("ORACLE_ADDRESS must be set")
            .parse()
            .expect("ORACLE_ADDRESS must be a valid Ethereum address");

        let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");

        let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
        let signer: PrivateKeySigner = private_key
            .parse()
            .expect("PRIVATE_KEY must be a valid private key");

        let wallet = EthereumWallet::from(signer);

        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .on_http(rpc_url.parse()?);

        Ok(Self {
            host,
            port,
            multiverse_address,
            oracle_address,
            provider,
        })
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
