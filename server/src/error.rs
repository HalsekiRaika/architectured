use std::fmt::{Display, Formatter};
use std::process::{ExitCode, Termination};
use error_stack::{Context, Report};
use driver::error::DriverError;

#[derive(Debug)]
pub struct StackTrace(ServerError);

impl From<ServerError> for StackTrace {
    fn from(e: ServerError) -> Self {
        StackTrace(e)
    }
}

impl Termination for StackTrace {
    fn report(self) -> ExitCode {
        Report::new(self.0).report()
    }
}

#[derive(Debug)]
pub enum ServerError {
    IO(Report<std::io::Error>),
    Driver(Report<DriverError>)
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(server): ")?;
        match self {
            ServerError::IO(e) => write!(f, "network io error: {:?}", e),
            ServerError::Driver(e) => Display::fmt(e, f)
        }
    }
}

impl Context for ServerError {}

impl From<std::io::Error> for ServerError {
    fn from(value: std::io::Error) -> Self {
        ServerError::IO(Report::new(value))
    }
}

impl From<DriverError> for ServerError {
    fn from(value: DriverError) -> Self {
        ServerError::Driver(Report::new(value))
    }
}

impl From<Report<DriverError>> for ServerError {
    fn from(value: Report<DriverError>) -> Self {
        Self::Driver(value)
    }
}
