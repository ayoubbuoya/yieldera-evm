use once_cell::sync::Lazy;

pub const COINGECKO_NETWORK_ID: &str = "polygon_pos"; // Polygon network ID for Coingecko API
pub const RPC_URL: &str = "https://polygon-rpc.com"; // Polygon RPC URL
pub const CHAIN_ID: u64 = 137; // Polygon chain ID

#[derive(Debug, Clone)]
pub struct Config {
    pub coingecko_api_key: String,
}

impl Config {
    pub fn load() -> Self {
        dotenvy::dotenv().ok(); // Load environment variables

        let coingecko_api_key =
            std::env::var("COINGECKO_API_KEY").expect("COINGECKO_API_KEY is not set");


        Self { coingecko_api_key }
    }
}

// Define a globally accessible static Config instance
pub static CONFIG: Lazy<Config> = Lazy::new(Config::load);
