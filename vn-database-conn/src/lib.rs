//! Handles database connection, reading and writing to the database.
//! Meant to be used by both the `vn_extractord_core` crate for writing to the database,
//! and used by the future rest api crate for reading.

mod error;
mod models;
mod schema;

pub use error::VybeDatabaseError;

use {diesel::prelude::*, dotenvy::dotenv, std::env};

/// Wraps the diesel pg database connection
#[allow(dead_code)]
pub struct DatabaseConn {
    /// Connection to our db
    conn: PgConnection,
}

impl DatabaseConn {
    /// Creates a new instance of the database connection,
    /// loads `DATABASE_URL` using dotenv from root .env file
    /// and immediately atttempts to connect.
    /// Should only be called once per application.
    ///
    /// # Errors
    ///
    /// `vn_database_conn::VybeDatabaseError::EnvVar`
    /// `vn_database_conn::VybeDatabaseError::Connection`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vn_database_conn::DatabaseConn;
    ///
    /// match DatabaseConn::new() {
    ///     Ok(conn) => {}
    ///     Err(e) => {}
    /// }
    ///
    /// ```
    pub fn new() -> Result<Self, VybeDatabaseError> {
        dotenv().ok();
        match env::var("DATABASE_URL") {
            Ok(database_url) => {
                let conn = PgConnection::establish(&database_url)?;
                Ok(Self { conn })
            }
            Err(_) => Err(VybeDatabaseError::EnvVar),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DatabaseConn;

    #[test]
    fn database_connection_test() {
        let dbconn_res = DatabaseConn::new();
        assert_eq!(dbconn_res.is_ok(), true);
    }
}
