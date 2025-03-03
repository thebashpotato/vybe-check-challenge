//! Database connection errors

use {
    diesel::{result::Error, ConnectionError},
    thiserror::Error,
};

/// Encapsulate 3rd party and std lib errors for this crate
#[derive(Error, Debug)]
pub enum VybeDatabaseError {
    /// Encapsulate var error
    #[error("DATABASE_URL env variable not found")]
    EnvVar,
    /// Encapsulates Diesel database `diesel::ConnectionError`
    #[error(transparent)]
    Connection(#[from] ConnectionError),
    /// Encapsulates the actual database errors from diesel
    #[error(transparent)]
    Diesel(#[from] Error),
    /// Represents an unexpected `PhoenixEvent::MarketDetails::Fill` variant.
    #[error("PhoenixEvent does not contain a Fill event")]
    InvalidPhoenixEvent,
}
