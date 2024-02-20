use deadpool_redis::{Config, Pool as RedisPool};
use error_stack::{Report, ResultExt};
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use crate::database::PgPool;
use crate::error::DriverError;

pub fn setup_redis() -> Result<RedisPool, Report<DriverError>> {
    Config::from_url(dotenvy::var("REDIS_CONNECTION").expect("require set `REDIS_CONNECTION`"))
        .create_pool(deadpool_redis::Runtime::Tokio1.into())
        .change_context_lazy(|| DriverError::Initialize)
}

pub async fn setup_query_db() -> Result<PgPool, DriverError> {
    const KEY: &str = "QUERY_DATABASE_URL";

    let url = dotenvy::var(KEY)
        .unwrap_or_else(|_| panic!("`{KEY}` does not set. This value required!"));

    let pool = PgPoolOptions::new()
        .connect(&url)
        .await?;

    sqlx::migrate!("../migrations/query")
        .run(&pool)
        .await?;

    Ok(PgPool::new(pool))
}

pub async fn setup_journal_db() -> Result<PgPool, DriverError> {
    const KEY: &str = "JOURNAL_DATABASE_URL";

    let url = dotenvy::var(KEY)
        .unwrap_or_else(|_| panic!("`{KEY}` does not set. This value required!"));

    let pool = PgPoolOptions::new()
        .connect(&url)
        .await?;

    sqlx::migrate!("../migrations/journal")
        .run(&pool)
        .await?;

    Ok(PgPool::new(pool))
}
