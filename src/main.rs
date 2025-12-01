mod config;
mod database {
    pub mod connection;
}
mod error;
mod services {
    pub mod pinata;
    pub mod solana;
}
mod state;
mod utils {
    pub mod connection_pool;
}

use axum::{
    routing::get,
    Router
};

use config::Config;
use services::pinata::PinataSrv;
use services::solana;

use self::services::solana::SolanaClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get config from environment
    let config = Config::from_env()?;
    println!("config done");

    let sql_pool = database::connection::create_sql_pool(
        &config.database.url,
        config.database.max_connections,
        config.database.min_connections,
    );
    println!("sql connected");

    let pinata_srv = PinataSrv::new(config.pinata);
    pinata_srv.upload_file(Vec::from("test from UniAxNft."), "UniAxNft_test").await?;
    println!("upload_file send");

    let solana_srv = SolanaClient::new(config.solana);
    println!("solana srv connected");

    let app = Router::new()
        .route("/health", get(|| async { "OK" }));

    let listen_addr = format!(
        "{}:{}",
        config.server.host,
        config.server.port,
    );
    let listener= tokio::net::TcpListener::bind(&listen_addr).await?;
    println!("UniAxNft listening on {}", listen_addr);
    axum::serve(listener, app).await?;

    Ok(())
}
