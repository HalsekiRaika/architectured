#[derive(Debug, thiserror::Error)]
pub enum DriverError {
    #[error(transparent)]
    Serde(serde_json::Error),
    #[error(transparent)]
    Redis(anyhow::Error),
    #[error(transparent)]
    Other(anyhow::Error)
}

impl From<serde_json::Error> for DriverError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

impl From<deadpool_redis::CreatePoolError> for DriverError {
    fn from(value: deadpool_redis::CreatePoolError) -> Self {
        Self::Redis(anyhow::Error::new(value))
    }
}

impl From<deadpool_redis::PoolError> for DriverError {
    fn from(value: deadpool_redis::PoolError) -> Self {
        Self::Redis(anyhow::Error::new(value))
    }
}

impl From<deadpool_redis::redis::RedisError> for DriverError {
    fn from(value: deadpool_redis::redis::RedisError) -> Self {
        Self::Redis(anyhow::Error::new(value))
    }
}

impl From<kernel::error::KernelError> for DriverError {
    fn from(value: kernel::error::KernelError) -> Self {
        Self::Other(anyhow::Error::new(value))
    }
}


impl From<DriverError> for kernel::error::KernelError {
    fn from(value: DriverError) -> Self {
        Self::Driver(anyhow::Error::new(value))
    }
}