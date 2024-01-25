mod person;

pub(in crate::database) mod redis_internal {
    use deadpool_redis::{Pool as RedisPool, Connection as RedisConnection};
    use error_stack::{Report, ResultExt};
    use crate::error::DriverError;

    pub async fn acquire(pool: &RedisPool) -> Result<RedisConnection, Report<DriverError>> {
        pool.get().await.change_context_lazy(|| DriverError::DeadPool)
    }
}

pub use self::{
    person::*
};