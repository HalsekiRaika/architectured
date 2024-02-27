use std::fmt::{Display, Formatter};
use error_stack::Context;

#[derive(Debug)]
pub enum KernelError {
    Validate {
        entity: &'static str,
        source: String,
    },
    Driver,
    EventPublish
}

impl Display for KernelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(kernel): ")?;
        match self {
            KernelError::Validate { entity, source } => write!(f, "validation error: {source} from {entity}"),
            KernelError::Driver => write!(f, "driver error."),
            KernelError::EventPublish => write!(f, "attempted to issue to non-supported Event(s).")
        }
    }
}

impl Context for KernelError {}



#[cfg(test)]
pub mod test {
    use std::fmt::{Display, Formatter};
    use error_stack::{Context, Report};
    use crate::error::KernelError;
    
    #[derive(Debug)]
    pub struct AnyKernelError;
    
    impl Display for AnyKernelError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "(kernel [Test]): Seems there was a `defection` somewhere...")
        }
    }
    
    impl Context for AnyKernelError {}
    
    impl From<KernelError> for Report<AnyKernelError> {
        fn from(value: KernelError) -> Self {
            Report::new(value).change_context(AnyKernelError)
        }
    }
}
