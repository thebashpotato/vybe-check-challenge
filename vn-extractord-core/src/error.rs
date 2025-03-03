//! Custom error handling

use {
    ellipsis_client::EllipsisClientError, solana_client::client_error::ClientError,
    solana_sdk::pubkey::ParsePubkeyError, thiserror::Error, tokio::task::JoinError,
    vn_database_core::VybeDatabaseError,
};

/// All custom and 3rd party crate errors will be encapusulated here
#[derive(Error, Debug)]
pub enum VybeDaemonError {
    /// Address string is wrong size or invalid
    #[error(transparent)]
    ParsePubkey(#[from] ParsePubkeyError),
    /// Wrapper error for phoenix-sdk client errors
    #[error("{0}")]
    PhoenixClient(String),
    /// Encapsulate all the ellipsis client errors
    #[error(transparent)]
    EllipsisClient(#[from] EllipsisClientError),
    /// Encapsulate Solana Client Errors
    #[error(transparent)]
    SolanaClient(#[from] ClientError),
    /// Encapsulate tokio join errors
    #[error(transparent)]
    TokioJoin(#[from] JoinError),
    /// Database connection errors
    #[error(transparent)]
    Database(#[from] VybeDatabaseError),
}
