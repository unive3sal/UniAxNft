use crate::error::{UniAxNftErr, UniAxNftResult};

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
    pub gateway: String,
    pub upload_url: String,
    pub api_url: String,
    pub api_secret: String,
    pub api_key: String,
    pub jwt: String,
}

impl Config {
    pub fn from_env() -> UniAxNftResult<Config> {
        let config = Config {
            server: ServerConfig {
                host: std::env::var("SERVER_HOST")
                        .unwrap_or("0.0.0.0".to_string()),
                port: std::env::var("SERVER_PORT")
                    .unwrap_or("8080".to_string())
                    .parse()
                    .map_err(|_| UniAxNftErr::ConfigErr(
                        "parse environment var SERVER_PORT err.".to_string()
                    ))?,
            },
            database: DatabaseConfig {
                url: std::env::var("DATABASE_URL")
                    .map_err(|e| UniAxNftErr::ConfigErr(
                        format!("environment var DATABASE_URL err: {}", e)
                    ))?,
                min_connections: std::env::var("DATABASE_MIN_CONN")
                    .unwrap_or("5".to_string())
                    .parse()
                    .map_err(|_| UniAxNftErr::ConfigErr(
                        "parse environment var DATABASE_MIN_CONN err".to_string()
                    ))?,
                max_connections: std::env::var("DATABASE_MAX_CONN")
                    .unwrap_or("20".to_string())
                    .parse()
                    .map_err(|_| UniAxNftErr::ConfigErr(
                        "parse environment var DATABASE_MAX_CONN err.".to_string()
                    ))?,
            },
            solana: SolanaConfig {
                rpc_url: std::env::var("SOLANA_RPC_RUL")
                    .unwrap_or("https://api.devnet.solana.com".to_string()),
                service_wallet: std::env::var("SOLANA_SERVICE_KEY")
                    .map_err(|e| UniAxNftErr::ConfigErr(
                        format!("environment var SOLANA_SERVICE_KEY err: {}", e)
                    ))?,
            },
            pinata: PinataConfig {
                gateway: std::env::var("PINATA_GATEWAY")
                    .map_err(|e| UniAxNftErr::ConfigErr(
                        format!("environment var PINATA_GATEWAY err: {}", e)
                    ))?,
                upload_url: std::env::var("PINATA_UPLOAD_URL")
                    .map_err(|e| UniAxNftErr::ConfigErr(
                        format!("environment var PINATA_UPLOAD_URL err: {}", e)
                    ))?,
                api_url: std::env::var("PINATA_API_URL")
                    .map_err(|e| UniAxNftErr::ConfigErr(
                        format!("environment var PINATA_API_URL err: {}", e)
                    ))?,
                api_key: std::env::var("PINATA_API_KEY")
                    .map_err(|e| UniAxNftErr::ConfigErr(
                        format!("environment var PINATA_API_KEY err: {}", e)
                    ))?,
                api_secret: std::env::var("PINATA_API_SECRET")
                    .map_err(|e| UniAxNftErr::ConfigErr(
                        format!("environment var PINATA_API_SECRET err: {}", e)
                    ))?,
                jwt: std::env::var("PINATA_JWT")
                    .map_err(|e| UniAxNftErr::ConfigErr(
                        format!("environment var PINATA_JWT err: {}", e)
                    ))?,
            },
        };

        Ok(config)
    }
}
