use deadpool_redis::{Pool as RedisPool, Connection as RedisConnection, redis};
use error_stack::{Report, ResultExt};
use kernel::error::KernelError;
use kernel::interfaces::repository::PersonRepository;
use kernel::prelude::entities::{Person, PersonId};
use crate::database::redis_internal;
use crate::error::DriverError;

pub struct PersonDataBase {
    pool: RedisPool
}

impl PersonDataBase {
    pub fn new(pool: RedisPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl PersonRepository for PersonDataBase {
    async fn create(&self, create: &Person) -> Result<(), Report<KernelError>> {
        let mut con = redis_internal::acquire(&self.pool).await
            .change_context_lazy(|| KernelError::Driver)?;
        RedisInternalPersonDataBase::upsert(create, &mut con).await?;
        Ok(())
    }

    async fn update(&self, update: &Person) -> Result<(), Report<KernelError>> {
        let mut con = redis_internal::acquire(&self.pool).await
            .change_context_lazy(|| KernelError::Driver)?;
        RedisInternalPersonDataBase::upsert(update, &mut con).await?;
        Ok(())
    }

    async fn delete(&self, delete: &PersonId) -> Result<(), Report<KernelError>> {
        let mut con = redis_internal::acquire(&self.pool).await
            .change_context_lazy(|| KernelError::Driver)?;
        RedisInternalPersonDataBase::delete(delete, &mut con).await?;
        Ok(())
    }

    async fn find_by_id(&self, id: &PersonId) -> Result<Option<Person>, Report<KernelError>> {
        let mut con = redis_internal::acquire(&self.pool).await
            .change_context_lazy(|| KernelError::Driver)?;
        let found = RedisInternalPersonDataBase::find_by_id(id, &mut con).await?;
        Ok(found)
    }
}


pub(in crate::database) struct RedisInternalPersonDataBase;

impl RedisInternalPersonDataBase {
    pub async fn upsert(
        create: &Person,
        con: &mut RedisConnection
    ) -> Result<(), DriverError> {
        redis::cmd("SET")
            .arg(serde_json::to_string(create.id())?)
            .arg(serde_json::to_string(&create)?)
            .query_async(&mut *con)
            .await?;
        Ok(())
    }

    pub async fn delete(
        delete: &PersonId,
        con: &mut RedisConnection
    ) -> Result<(), DriverError> {
        redis::cmd("DEL")
            .arg(serde_json::to_string(delete)?)
            .query_async(&mut *con)
            .await?;
        Ok(())
    }

    pub async fn find_by_id(
        id: &PersonId,
        con: &mut RedisConnection
    ) -> Result<Option<Person>, DriverError> {
        let raw: Option<String> = redis::cmd("GET")
            .arg(serde_json::to_string(id)?)
            .query_async(&mut *con)
            .await?;
        let person: Option<Person> = raw
            .map(|raw| serde_json::from_str(&raw))
            .transpose()?;
        Ok(person)
    }
}