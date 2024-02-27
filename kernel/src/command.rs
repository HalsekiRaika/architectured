mod person;
mod book;

pub use self::person::*;
pub use self::book::*;


pub trait Publish<T>: 'static + Sync + Send {
    type Error;
    fn publish(self) -> Result<T, Self::Error>;
}