mod config;
mod database {
    pub mod connection;
}
mod error;
mod middleware {
    pub mod auth;
}
mod services {
    pub mod pinata;
    pub mod nft;
}
mod state;

use std::time::Duration;
use axum::{
    http,
    routing::{
        get,
        post,
        patch,
        delete,
    },
    Router
};
use tower_http::{
    auth::AsyncRequireAuthorizationLayer,
    cors::CorsLayer,
    request_id::{
        SetRequestIdLayer,
        MakeRequestUuid
    },
    trace::TraceLayer,
};
use tracing::{info, Level};
use tracing_subscriber;

use config::Config;
use middleware::auth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: logging system
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .finish();
    // get config from environment
    let config = Config::from_env()?;
    info!("config done");

    let listen_addr = format!(
        "{}:{}",
        config.server.host,
        config.server.port,
    );

    let state = state::UniAxNftState::new(config).await?;

    // TODO: cors should be re-defined later
    let cors = CorsLayer::new()
        .allow_origin([
            "https://nft.noman.work".parse().unwrap()
        ])
        .allow_methods([
            http::Method::GET,
            http::Method::POST,
            http::Method::PUT,
            http::Method::DELETE
        ])
        .allow_headers([
            http::header::CONTENT_TYPE,
            http::header::AUTHORIZATION,
        ])
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600));

    let protected_routes = Router::new()
        /*
        .route("/change_pwd", put(change_password))
        .route("/user/:user_id", get(handler))
        .route("/users/:user_id/nfts?page=1&limit=20&sort=created_at&order=desc", get(handler))
        .route("/nfts", post(handler))
        .route("/nfts/:nft_id", get(handler))
        .route("/nfts/:nft_id/status", get(handler))
        .route("/nfts/:nft_id", patch(handler))
        .route("/nfts/:nft_id", delete(handler))
        */
        .layer(AsyncRequireAuthorizationLayer::new(auth::Authorizer))
        .with_state(state.clone());

    let public_routes = Router::new()
        //.route("/auth/register", post(user_register))
        //.route("/auth/login", post(user_login));
        .with_state(state.clone());

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .nest("/api/v1", protected_routes)
        .nest("/api/v1", public_routes)
        .layer(TraceLayer::new_for_http())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid::default()))
        .layer(cors)
        .with_state(state.clone());

    let listener= tokio::net::TcpListener::bind(&listen_addr).await?;
    info!("UniAxNft listening on {}", listen_addr);

    axum::serve(listener, app).await?;

    Ok(())
}
