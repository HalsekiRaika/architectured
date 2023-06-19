#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error(transparent)]
    Internal(anyhow::Error)
}

impl From<driver::error::DriverError> for ServerError {
    fn from(value: driver::error::DriverError) -> Self {
        ServerError::Internal(anyhow::Error::new(value))
    }
}