mod entities;
mod repository;

pub mod error;

#[cfg(feature = "prelude")]
pub mod prelude {
    pub mod entities {
        pub use crate::entities::*;
    }
}

#[cfg(feature = "interface")]
pub mod interfaces {
    pub mod repository {
        pub use crate::repository::*;
    }
}