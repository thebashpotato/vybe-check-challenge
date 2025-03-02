//! Database connection errors

use {diesel::ConnectionError, thiserror::Error};

/// Encapsulate 3rd party and std lib errors for this crate
#[derive(Error, Debug)]
pub enum VybeDatabaseError {
    /// Encapsulate var error
    #[error("DATABASE_URL env variable not found")]
    EnvVar,
    /// Encapsulates Diesel database `diesel::ConnectionError`
    #[error(transparent)]
    Connection(#[from] ConnectionError),
}
