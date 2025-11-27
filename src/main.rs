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
    pub mod rpc_connection_pool;
}

use axum::{
    routing::get,
    Router
};

use config::Config;
use services::pinata::PinataSrv;

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
