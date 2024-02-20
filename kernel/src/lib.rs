mod entities;
mod repository;

pub mod error;
mod journal;
mod event;
mod command;
mod io;

#[cfg(feature = "prelude")]
pub mod prelude {
    pub mod entities {
        pub use crate::entities::*;
    }

    pub mod commands {
        pub use crate::command::*;
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
    
    pub mod io {
        pub use crate::io::*;
    }
}