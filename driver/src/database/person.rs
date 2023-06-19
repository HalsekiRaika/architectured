use deadpool_redis::{Pool as RedisPool, Connection as RedisConnection, redis};
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
    async fn create(&self, create: &Person) -> Result<(), KernelError> {
        let mut con = redis_internal::acquire(&self.pool).await?;
        RedisInternalPersonDataBase::upsert(create, &mut con).await?;
        Ok(())
    }

    async fn update(&self, update: &Person) -> Result<(), KernelError> {
        let mut con = redis_internal::acquire(&self.pool).await?;
        RedisInternalPersonDataBase::upsert(update, &mut con).await?;
        Ok(())
    }

    async fn delete(&self, delete: &PersonId) -> Result<(), KernelError> {
        let mut con = redis_internal::acquire(&self.pool).await?;
        RedisInternalPersonDataBase::delete(delete, &mut con).await?;
        Ok(())
    }

    async fn find_by_id(&self, id: &PersonId) -> Result<Option<Person>, KernelError> {
        let mut con = redis_internal::acquire(&self.pool).await?;
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
            .arg(create.id().as_ref())
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
            .arg(delete.as_ref())
            .query_async(&mut *con)
            .await?;
        Ok(())
    }

    pub async fn find_by_id(
        id: &PersonId,
        con: &mut RedisConnection
    ) -> Result<Option<Person>, DriverError> {
        let raw: Option<String> = redis::cmd("GET")
            .arg(id.as_ref())
            .query_async(&mut *con)
            .await?;
        let person: Option<Person> = raw
            .map(|raw| serde_json::from_str(&raw))
            .transpose()?;
        Ok(person)
    }
}