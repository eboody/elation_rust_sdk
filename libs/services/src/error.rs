// libs/services/error.rs

use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

use std::fmt;

/// A specialized `Result` type for the services crate.
///
/// This is defined as a convenience so that you don't have to write out
/// `core::result::Result<T, Error>` every time.
///
/// # Example
///
/// ```
/// use services::Result;
///
/// fn your_service_function() -> Result<()> {
///     // Your code here...
///     Ok(())
/// }
/// ```
pub type Result<T> = core::result::Result<T, Error>;

/// The error type for the services crate.
///
/// This enum represents all possible errors that can occur when using the services.
/// It includes variants for service-specific errors, as well as wrapping errors
/// originating from the client crate.
#[serde_as]
#[derive(Debug, From, Serialize)]
pub enum Error {
    /// An error originating from the client crate.
    ///
    /// This variant wraps `client::Error`, which can occur during HTTP requests or client operations.
    #[from]
    ClientError(#[serde_as(as = "DisplayFromStr")] client::Error),

    /// Represents an error when invalid input is provided to a service method.
    ///
    /// Contains a message describing the invalid input.
    InvalidInput(String),
}

impl fmt::Display for Error {
    /// Formats the error using the `Display` trait.
    ///
    /// This implementation delegates to the `Debug` representation,
    /// which includes the error variant name and any associated data.
    ///
    /// # Example
    ///
    /// ```
    /// use services::Error;
    ///
    /// let error = Error::InvalidInput("Invalid patient ID".into());
    /// println!("{}", error);
    /// // Output: InvalidInput("Invalid patient ID")
    /// ```
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidInput(msg) => write!(fmt, "Invalid input: {}", msg),
            Error::ClientError(e) => write!(fmt, "{}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::ClientError(client::Error::ReqwestError(err))
    }
}
