use sqlx::Postgres;
use crate::database::{Pool, Transaction};

impl kernel::interfaces::io::AcquireTransaction for Pool<sqlx::PgPool> {
    type Transaction = Transaction<sqlx::Transaction<'static, Postgres>>;
    type Error = sqlx::Error;
    async fn acquire(&self) -> Result<Self::Transaction, Self::Error> {
        Ok(Transaction::new(self.begin().await?))
    }
}

impl kernel::interfaces::io::Transaction for Transaction<sqlx::Transaction<'_, Postgres>> {
    type Error = sqlx::Error;
    
    async fn commit(self) -> Result<(), Self::Error> {
        sqlx::Transaction::commit(self.into()).await
    }
    
    async fn rollback(self) -> Result<(), Self::Error> {
        sqlx::Transaction::rollback(self.into()).await
    }
}