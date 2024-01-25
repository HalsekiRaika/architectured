use std::fmt::{Display, Formatter};
use error_stack::Context;

#[derive(Debug)]
pub enum SqlXCompatError {
    Core(sqlx::Error),
    Migrate(sqlx::migrate::MigrateError),
}

impl Display for SqlXCompatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SqlXCompatError::Core(e) => Display::fmt(e, f),
            SqlXCompatError::Migrate(e) => Display::fmt(e, f)
        }
    }
}

impl Context for SqlXCompatError {}

impl From<sqlx::Error> for SqlXCompatError {
    fn from(e: sqlx::Error) -> Self {
        Self::Core(e)
    }
}

impl From<sqlx::migrate::MigrateError> for SqlXCompatError {
    fn from(e: sqlx::migrate::MigrateError) -> Self {
        Self::Migrate(e)
    }
}

impl From<SqlXCompatError> for crate::error::DriverError {
    fn from(e: SqlXCompatError) -> Self {
        Self::Sqlx(error_stack::Report::new(e))
    }
}

// Install Section
impl From<sqlx::Error> for crate::error::DriverError {
    fn from(e: sqlx::Error) -> Self {
        Self::Sqlx(error_stack::Report::new(e.into()))
    }
}

impl From<sqlx::migrate::MigrateError> for crate::error::DriverError {
    fn from(e: sqlx::migrate::MigrateError) -> Self {
        Self::Sqlx(error_stack::Report::new(e.into()))
    }
}