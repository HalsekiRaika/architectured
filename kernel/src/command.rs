mod person;

pub use self::person::*;


pub trait Publication<T>: 'static + Sync + Send {
    type Error;
    fn publish(self) -> Result<T, Self::Error>;
}