//! crate to extract trade fill events from solana using the phoenix dex software development kit

mod error;
mod extractor;
pub use error::VybeDaemonError;

use {
    crate::extractor::{VybeResult, VybeTradeFillExtractor},
    tracing::{error, info},
    vn_database_conn::DatabaseConn,
};

/// Top level interface
pub struct VybeDaemon {
    /// Phoenix sdk and Helius interface
    trade_fill_extractor: VybeTradeFillExtractor,
    /// PG database connection and interface
    db: DatabaseConn,
}

impl VybeDaemon {
    /// Creates a new `VybeDaemon` this automatically makes a database connection, and a connection
    /// to Helius.
    ///
    /// # Parameters
    ///
    /// - `api_key`: user's API key to Helium RPC
    /// - `phoenix_addr`: Phoenix deployment address
    ///
    /// # Errors
    ///
    /// `VybeDaemonError::Pubkey` if `phoenix_addr` is incorrect size
    ///
    /// # Returns
    ///
    /// Result<Self, `VybeDaemonError::Pubkey`>
    pub async fn new(api_key: &str, phoenix_addr: &str) -> VybeResult<Self> {
        Ok(Self {
            trade_fill_extractor: VybeTradeFillExtractor::new(api_key, phoenix_addr).await?,
            db: DatabaseConn::new()?,
        })
    }

    /// Run the daemon inside a never ending loop. In the real world we should listen
    /// for sigterm/sigkill events, but I don't want to put operating specific code in here.
    ///
    /// # Errors
    ///
    /// `VybeDaemonError::Pubkey`
    /// `VybeDaemonError::PhoenixClient`
    /// `VybeDaemonError::EllipsisClient`
    /// `VybeDaemonError::SolanaClient`
    /// `VybeDaemonError::TokioJoin`
    ///
    /// # Return
    ///
    /// Result<Self, `VybeDaemonError::Pubkey`>
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vn_extractord_core::VybeDaemon;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let api_key = "your_api_key";
    ///     let phoenix_addr = "phoenix_address";
    ///
    ///     match &mut VybeDaemon::new(api_key, phoenix_addr).await {
    ///         Ok(vdaemon) => {
    ///             if let Err(e) = vdaemon.run().await {
    ///                 println!("{e}");
    ///             }
    ///         }
    ///         Err(e) => {
    ///             println!("{e}");
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn run(&mut self) -> VybeResult<()> {
        loop {
            let fill_events_opt = self.trade_fill_extractor.extract().await?;

            if let Some(fill_events) = fill_events_opt {
                for fill_event in fill_events {
                    match self.db.create_trade_fill(&fill_event.try_into()?) {
                        Ok(_) => {
                            info!("Successfully created new trade fill entry..");
                        }
                        Err(e) => {
                            error!("{e}");
                        }
                    }
                }
            }

            // NOTE: this is a basic polling loop. If there are >1000 signatures in 200ms,
            // events may get dropped.
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        }
    }
}
