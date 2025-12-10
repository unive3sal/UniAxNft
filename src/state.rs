use std::sync::Arc;
use sqlx::PgPool;

use crate::config::ServerConfig;
use crate::error::UniAxNftErr;
use crate::middleware::auth::Authorizer;
use crate::{
    config::Config,
    database,
    error::UniAxNftResult,
    services::nft::NftSrv,
    services::pinata::PinataSrv
};

#[derive(Clone)]
pub struct UniAxNftState {
    pub config: Arc<ServerConfig>,
    pub db: PgPool,
    pub pinata: Arc<PinataSrv>,
    pub nft: Arc<NftSrv>,
}

impl UniAxNftState {
    pub async fn new(config: Config) -> UniAxNftResult<UniAxNftState> {
        let db = database::connection::create_sql_pool(
            &config.database.url,
            config.database.max_connections,
            config.database.min_connections,
        ).await?;

        sqlx::migrate!("./migrations")
            .run(&db)
            .await
            .map_err(|e| UniAxNftErr::DatabaseErr(
                format!("PgSql migration err: {}", e)
            ))?;

        let pinata = Arc::new(PinataSrv::new(config.pinata));
        let nft = Arc::new(NftSrv::new(config.solana));

        // it probably throw a panic if env var is not correctly set
        Authorizer::init_jwt_keypair();

        Ok(Self {
            config: Arc::new(config.server),
            db: db,
            pinata: pinata,
            nft: nft,
        })
    }
}
