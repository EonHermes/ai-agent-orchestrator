use sqlx::{PgPool, Postgres};
use std::time::Duration;
use crate::error::Result;

pub async fn connect(config: &Arc<config::Config>) -> Result<PgPool> {
    let database_url = config.get_string("DATABASE_URL")?;

    let pool = PgPool::connect_lazy_with(&sqlx::postgres::PgConnectOptions::from_str(&database_url)?, {
        let mut opts = sqlx::postgres::PgPoolOptions::new();
        opts = opts.max_connections(config.get_int("DATABASE_MAX_CONNECTIONS")? as u32)
            .acquire_timeout(Duration::from_secs(
                config.get_int("DATABASE_ACQUIRE_TIMEOUT_SECONDS")? as u64
            ));
        opts
    });

    Ok(pool)
}

pub async fn health_check(pool: &PgPool) -> Result<()> {
    sqlx::query!("SELECT 1")
        .fetch_one(pool)
        .await?;
    Ok(())
}
