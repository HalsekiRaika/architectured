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


pub type PgPool = Pool<sqlx::PgPool>;
pub type PgTransaction = Transaction<sqlx::Transaction<'static, Postgres>>;

use std::ops::{Deref, DerefMut};
use sqlx::Postgres;

pub struct Pool<T>(T);

impl Pool<sqlx::PgPool> {
    pub fn new(pool: sqlx::PgPool) -> Pool<sqlx::PgPool> {
        Self(pool)
    }
}

impl Deref for Pool<sqlx::PgPool> {
    type Target = sqlx::PgPool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Pool<sqlx::PgPool> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct Transaction<T>(T);

impl Transaction<sqlx::Transaction<'_, Postgres>> {
    pub fn new(transaction: sqlx::Transaction<'static, Postgres>) -> Self {
        Self(transaction)
    }
}

impl<'a> Deref for Transaction<sqlx::Transaction<'a, Postgres>> {
    type Target = sqlx::Transaction<'a, Postgres>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Transaction<sqlx::Transaction<'_, Postgres>> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Transaction<T> {
    pub fn into(self) -> T {
        self.0
    }
}