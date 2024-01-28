use std::fmt::{Display, Formatter};
use error_stack::{Context, Report};
use crate::error::DriverError;

#[derive(Debug)]
pub enum InternalError {
    Constraint(&'static str)
}

impl Display for InternalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(driver [Internal Details]): ")?;
        match self {
            InternalError::Constraint(msg) => write!(f, "Error in constraint >> {msg}")
        }
    }
}

impl Context for InternalError {}

impl From<InternalError> for DriverError {
    fn from(value: InternalError) -> Self {
        DriverError::Internal(Report::new(value))
    }
}