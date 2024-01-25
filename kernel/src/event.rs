mod person;

pub use self::person::*;

pub trait Applier<T>: 'static + Sync + Send {
    fn apply(&mut self, event: T);
}