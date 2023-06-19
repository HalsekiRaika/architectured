use deadpool_redis::{Config, Pool as RedisPool};
use crate::error::DriverError;

pub fn setup() -> Result<RedisPool, DriverError> {
    Config::from_url(dotenvy::var("REDIS_CONNECTION").expect("require set `REDIS_CONNECTION`"))
        .create_pool(deadpool_redis::Runtime::Tokio1.into())
        .map_err(Into::into)
}