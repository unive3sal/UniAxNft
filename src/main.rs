mod config;
mod database {
    pub mod connection;
}
mod error;
mod state;

use axum::{
    routing::get,
    Router
};

use config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get config from environment
    let config = Config::from_env()?;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    let listener= tokio::net::TcpListener::bind("0.0.0.0:443").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
