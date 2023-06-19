#[derive(Debug, thiserror::Error)]
pub enum KernelError {
    #[error(transparent)]
    Driver(anyhow::Error)
}