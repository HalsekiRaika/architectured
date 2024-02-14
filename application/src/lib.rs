mod impls;
mod error;
mod gateway;
mod service;

#[cfg(feature = "prelude")]
mod prelude {
    pub mod errors {
        pub use crate::error::*;
    }
    pub mod services {
        pub use crate::service::*;
    }
}

#[cfg(feature = "prelude")]
pub use self::prelude::{errors, services};

#[cfg(feature = "interfaces")]
pub mod interfaces {
    pub use super::gateway::*;
}
