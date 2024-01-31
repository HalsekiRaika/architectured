mod person;

pub use self::person::*;


use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Envelope<T> {
    pub version: i64,
    inner: T,
}

impl<T> Envelope<T> {
    pub fn new(version: i64, val: T) -> Envelope<T> {
        Self { version, inner: val }
    }

    pub fn version(&self) -> i64 {
        self.version
    }

    pub fn into(self) -> T {
        self.inner
    }
}

impl<T> From<T> for Envelope<T> {
    fn from(value: T) -> Self {
        Self::new(1, value)
    }
}

impl<T> Deref for Envelope<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Envelope<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
