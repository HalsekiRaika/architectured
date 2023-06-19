mod person;

pub(in crate::database) mod redis_internal {
    use deadpool_redis::{Pool as RedisPool, Connection as RedisConnection};
    use crate::error::DriverError;

    pub async fn acquire(pool: &RedisPool) -> Result<RedisConnection, DriverError> {
        Ok(pool.get().await?)
    }
}

pub use self::{
    person::*
};