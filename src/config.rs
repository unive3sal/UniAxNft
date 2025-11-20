pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub solana: SolanaConfig,
    pub pinata: PinataConfig,
}

pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

pub struct DatabaseConfig {
    pub url: String,
    pub min_connections: u32,
    pub max_connections: u32,
}

pub struct SolanaConfig {
    pub rpc_url: String,
    pub service_wallet: String,
}

pub struct PinataConfig {
    pub api_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Config {
            server: ServerConfig {
                host: std::env::var("SERVER_HOST")
                    .unwrap_or("0.0.0.0".to_string()),
                port: std::env::var("SERVER_PORT")
                    .unwrap_or("8080".to_string()).parse()?,
            },
            database: DatabaseConfig {
                url: std::env::var("DATABASE_URL")?,
                max_connections: std::env::var("DATABASE_MAX_CONN")
                    .unwrap_or("20".to_string()).parse()?
            },
            solana: SolanaConfig {
                rpc_url: std::env::var("SOLANA_RPC_RUL")?,
                service_wallet: std::env::var("SOLANA_SERVICE_KEY")?,
            },
            pinata: PinataConfig {
                api_key: std::env::var("PINATA_API_KEY")?,
            },
        };

        Ok(config)
    }
}
