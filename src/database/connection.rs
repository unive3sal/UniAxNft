use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use crate::error::{UniAxNftErr, UniAxNftResult};

pub async fn create_sql_pool(
    database_url: &str,
    max_connections: u32,
    min_connections: u32,
) -> UniAxNftResult<PgPool> {
    let pool = PgPoolOptions::new()
        .min_connections(min_connections)
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(60 * 1))
        .max_lifetime(Duration::from_secs(60 * 5))
        .connect(database_url).await
        .map_err(|e| UniAxNftErr::DatabaseErr(format!("Failed to create sql pool: {}", e)))?;

    Ok(pool)
}

