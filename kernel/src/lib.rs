mod entities;
mod repository;

pub mod error;
mod journal;
mod event;

#[cfg(feature = "prelude")]
pub mod prelude {
    pub mod entities {
        pub use crate::entities::*;
    }
    pub mod events {
        pub use crate::event::*;
    }
}

#[cfg(feature = "interface")]
pub mod interfaces {
    pub mod repository {
        pub use crate::repository::*;
    }

    pub mod journal {
        pub use crate::journal::*;
    }
}