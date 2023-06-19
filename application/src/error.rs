use kernel::error::KernelError;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("cannot find `{value}:{entity}` in the following {method}.")]
    NotFound {
        entity: &'static str,
        method: &'static str,
        value: String
    },
    #[error(transparent)]
    Other(anyhow::Error)
}

impl From<KernelError> for ApplicationError {
    fn from(value: KernelError) -> Self {
        match value {
            KernelError::Driver(e)
                => Self::Other(e)
        }
    }
}