pub mod compat;

use std::fmt::{Debug, Display, Formatter};
use error_stack::{Context, Report};
use kernel::error::KernelError;

#[derive(Debug)]
pub enum DriverError {
    Sqlx(Report<compat::SqlXCompatError>),
    Serde(Report<serde_json::Error>),
    Redis(Report<deadpool_redis::redis::RedisError>),
    DeadPool,
    Initialize,
    Other
}

impl Display for DriverError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(driver): ")?;
        match self {
            DriverError::Sqlx(e) => Display::fmt(e, f),
            DriverError::Serde(e) => write!(f, "{:?}", e),
            DriverError::Redis(e) => write!(f, "{:?}", e),
            DriverError::DeadPool => write!(f, "describe me"),
            DriverError::Initialize => write!(f, "describe me"),
            DriverError::Other => write!(f, "describe me")
        }
    }
}

impl Context for DriverError {}

#[derive(Debug)]
enum CategorizeDriverError {
    External,
    Internal(DriverError)
}

impl Display for CategorizeDriverError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::External => write!(f, "(driver [external]): failed on except for driver."),
            Self::Internal(e) => Display::fmt(e, f)
        }
    }
}

impl Context for CategorizeDriverError {}

impl From<serde_json::Error> for DriverError {
    fn from(e: serde_json::Error) -> Self {
        Self::Serde(Report::new(e))
    }
}

impl From<deadpool_redis::redis::RedisError> for DriverError {
    fn from(e: deadpool_redis::redis::RedisError) -> Self {
        Self::Redis(Report::new(e))
    }
}

impl From<DriverError> for Report<CategorizeDriverError> {
    fn from(e: DriverError) -> Self {
        match e {
            DriverError::Sqlx(r) => Report::new(CategorizeDriverError::External).attach_printable(r),
            DriverError::Serde(r) => Report::new(CategorizeDriverError::External).attach_printable(r),
            DriverError::Redis(r) => Report::new(CategorizeDriverError::External).attach_printable(r),
            DriverError::DeadPool => Report::new(CategorizeDriverError::Internal(e)),
            DriverError::Initialize => Report::new(CategorizeDriverError::Internal(e)),
            DriverError::Other => Report::new(CategorizeDriverError::Internal(e)),
        }
    }
}

impl From<DriverError> for Report<KernelError> {
    fn from(e: DriverError) -> Self {
        Report::<CategorizeDriverError>::from(e)
            .change_context(KernelError::Driver)
    }
}


#[cfg(test)]
pub(crate) mod test {
    use std::fmt::{Display, Formatter};
    use error_stack::{Context, Report};
    use crate::error::{CategorizeDriverError, DriverError};
    
    #[derive(Debug)]
    pub struct AnyDriverError;
    
    impl Display for AnyDriverError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "(driver [Test]): Seems there was a `defection` somewhere...")
        }
    }
    
    impl Context for AnyDriverError {}
    
    impl From<DriverError> for Report<AnyDriverError> {
        fn from(e: DriverError) -> Self {
            Report::<CategorizeDriverError>::from(e)
                .change_context(AnyDriverError)
        }
    }
}