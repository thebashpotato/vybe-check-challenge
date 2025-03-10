//! Handles database connection, reading and writing to the database.
//! Meant to be used by both the `vn_extractord_core` crate for writing to the database,
//! and used by the future rest api crate for reading.

mod error;
pub mod models;
pub mod schema;

pub use error::VybeDatabaseError;

use {
    diesel::prelude::*,
    dotenvy::dotenv,
    models::{NewTradeFill, TradeFill},
    schema::trade_fills,
    std::env,
    tracing::debug,
};

/// PG Database abstraction/interface
pub struct VybeDatabase {
    /// Connection to our db
    conn: PgConnection,
}

impl VybeDatabase {
    /// Creates a new instance of the database connection,
    /// loads `DATABASE_URL` using dotenv from root .env file
    /// and immediately atttempts to connect.
    /// Should only be called once per application.
    ///
    /// # Errors
    ///
    /// `vn_database_core::VybeDatabaseError::EnvVar`
    /// `vn_database_core::VybeDatabaseError::Connection`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vn_database_core::VybeDatabase;
    ///
    /// match VybeDatabase::new() {
    ///     Ok(conn) => {}
    ///     Err(e) => {}
    /// }
    /// ```
    pub fn new() -> Result<Self, VybeDatabaseError> {
        dotenv().ok();
        match env::var("DATABASE_URL") {
            Ok(database_url) => {
                let conn = PgConnection::establish(&database_url)?;
                debug!("Established connection to {database_url}");
                Ok(Self { conn })
            }
            Err(_) => Err(VybeDatabaseError::EnvVar),
        }
    }

    /// Get a mutable reference to the underlying connection
    pub fn conn(&mut self) -> &mut PgConnection {
        &mut self.conn
    }

    /// Gets a trade fill by the id number
    ///
    /// # Errors
    ///
    /// `vn_database_core::VybeDatabaseError::Diesel` if the Diesel query fails
    pub fn get_trade_fill_by_id(&mut self, id: i32) -> Result<Vec<TradeFill>, VybeDatabaseError> {
        Ok(trade_fills::table
            .filter(trade_fills::id.eq(id))
            .limit(5)
            .select(TradeFill::as_select())
            .load(self.conn())?)
    }

    /// Gets all trade fill records from the database.
    ///
    /// # Errors
    ///
    /// Returns a `VybeDatabaseError::Diesel` if the Diesel query fails.
    pub fn get_all_trade_fills(&mut self) -> Result<Vec<TradeFill>, VybeDatabaseError> {
        Ok(trade_fills::table
            .select(TradeFill::as_select())
            .load(self.conn())?)
    }

    /// Create a new trade fill entry in the database
    ///
    /// # Params
    ///
    ///
    /// # Errors
    ///
    /// `vn_database_core::VybeDatabaseError::Diesel`
    pub fn create_trade_fill(
        &mut self,
        new_trade_fill: &NewTradeFill,
    ) -> Result<TradeFill, VybeDatabaseError> {
        Ok(diesel::insert_into(trade_fills::table)
            .values(new_trade_fill)
            .returning(TradeFill::as_returning())
            .get_result(self.conn())?)
    }
}
