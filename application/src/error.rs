use std::fmt::{Display, Formatter};
use std::ops::{Not, Shl};
use error_stack::{Context, Report};
use kernel::error::KernelError;

#[derive(Debug)]
pub enum ApplicationError {
    Driver,
    Kernel
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(application): ")?;
        match self {
            ApplicationError::Driver => write!(f, "from driver error."),
            ApplicationError::Kernel => write!(f, "from kernel error."),
        }
    }
}

impl Context for ApplicationError {}