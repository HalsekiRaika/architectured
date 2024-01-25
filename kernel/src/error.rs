use std::fmt::{Display, Formatter};
use error_stack::Context;

#[derive(Debug)]
pub enum KernelError {
    Driver,
}

impl Display for KernelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(kernel): ")?;
        match self {
            KernelError::Driver => write!(f, "driver error.")
        }
    }
}

impl Context for KernelError {}
