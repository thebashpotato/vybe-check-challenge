//! crate to extract trade fill events from solana using the phoenix dex software development kit

use {
    crate::error::VybeDaemonError,
    derive_getters::Getters,
    futures::StreamExt,
    phoenix_sdk::sdk_client::{MarketEventDetails, PhoenixEvent, SDKClient},
    solana_sdk::{pubkey::Pubkey, signature::Signature, signer::keypair::Keypair},
    std::{convert::TryFrom, str::FromStr, sync::Arc},
    tokio::task::JoinHandle,
    tracing::{debug, error, info, warn},
};

/// Remote Procedure Call endpoint for Solana
const HELIUS_RPC_ENDPOINT: &str = "https://mainnet.helius-rpc.com/?api-key=";

/// The `get_signatures_for_address` endpoint should return 1000 signatures
const NUM_EXPECTED_TRANSACTIONS: usize = 1000;

/// Number of task threads we want to use when parsing transactions concurrently
const NUM_TASK_THREADS: usize = 100;

/// Custom result type
pub type VybeResult<T> = Result<T, VybeDaemonError>;

/// Wraps the Helium blochchain RPC service, and the Phoenix SDK onchain orderbook.
#[derive(Getters)]
pub struct VybeTradeFillExtractor {
    /// Public Key to market
    market_pubkey: Pubkey,
    /// Atomically ref counted client
    sdk_client: Arc<SDKClient>,
}

impl VybeTradeFillExtractor {
    /// Creates a new `VybeTradeFillExtractor` instance by concatenating the Helium API endpoint with the provided API key.
    ///
    /// # Parameters
    ///
    /// - `api_key`: user's API key to Helium RPC
    /// - `phoenix_addr`: Phoenix deployment address
    ///
    /// # Errors
    ///
    /// `VybeDaemonError::Pubkey`
    ///
    /// # Returns
    ///
    /// Result<Self, `VybeDaemonError::Pubkey`>
    pub async fn new(api_key: &str, phoenix_addr: &str) -> VybeResult<Self> {
        let url = format!("{HELIUS_RPC_ENDPOINT}{api_key}");
        let market_pubkey = Pubkey::try_from(phoenix_addr)?;
        match SDKClient::new(&Keypair::new(), url.as_str()).await {
            Ok(sdk_client) => {
                debug!("Connected to solana provider {HELIUS_RPC_ENDPOINT}");
                Ok(Self {
                    market_pubkey,
                    sdk_client: Arc::new(sdk_client),
                })
            }
            Err(e) => Err(VybeDaemonError::PhoenixClient(e.to_string())),
        }
    }

    /// Extracts and returns Fill events, the fill events are sorted by
    /// the `sequence_number`, but since Solana is a distributed system
    /// this makes no gaurentee that fill events will be in order in the database,
    /// we could wait until we have 10, or 20 fill events, then sort by sequence number
    /// then write to the database, but then we get further and further away from real-time/near
    /// real-time.
    ///
    /// # Errors
    ///
    /// `VybeDaemonError::PhoenixClient`
    /// `VybeDaemonError::EllipsisClient`
    /// `VybeDaemonError::SolanaClient`
    /// `VybeDaemonError::TokioJoin`
    ///
    /// # Returns
    ///
    /// `Result<Vec<PhoenixEvent>, VybeDaemonError>>`
    pub async fn extract(&self) -> VybeResult<Option<Vec<PhoenixEvent>>> {
        info!("Extracting new fill events...");
        let signatures = self.get_signatures().await?;
        if signatures.len() < NUM_EXPECTED_TRANSACTIONS {
            warn!("Extracted {} signatures..", signatures.len());
        }

        let handles = self.build_event_handles(signatures);
        let mut fill_events = Self::extract_fill_events(handles).await?;

        info!("Recieved {} fill event(s)", fill_events.len());
        if fill_events.is_empty() {
            Ok(None)
        } else {
            if fill_events.len() > 1 {
                fill_events.sort_by_key(|event| event.sequence_number);
            }
            Ok(Some(fill_events))
        }
    }

    /// Get signatures
    async fn get_signatures(&self) -> VybeResult<Vec<Option<Signature>>> {
        debug!("Getting signatures...");
        Ok(self
            .sdk_client
            .client
            .get_signatures_for_address(&self.market_pubkey)
            .await?
            .into_iter()
            .map(
                |rpc_cts| match Signature::from_str(rpc_cts.signature.as_str()) {
                    Ok(sig) => Some(sig),
                    Err(e) => {
                        error!("Failed to parse a Signature: {e}");
                        None
                    }
                },
            )
            .collect::<Vec<Option<Signature>>>())
    }

    /// Build a single asynchronous task handle responsible for parsing each transaction signature
    fn build_event_handles(
        &self,
        signatures: impl IntoIterator<Item = Option<Signature>>,
    ) -> Vec<JoinHandle<Option<Vec<PhoenixEvent>>>> {
        debug!("Building event task handles...");
        let mut handles: Vec<JoinHandle<Option<Vec<PhoenixEvent>>>> = vec![];
        for opt_sig in signatures {
            let sdk = Arc::<SDKClient>::clone(&self.sdk_client);
            if let Some(sig) = opt_sig {
                handles.push(tokio::spawn(async move {
                    sdk.parse_events_from_transaction(&sig).await
                }));
            }
        }
        handles
    }

    /// Extract fill events from each async handle.
    /// Since 1000 is a decently sized number let's use a stream that buffers around 100
    /// join handles concurrently to try and be as fast as possible.
    async fn extract_fill_events(
        handles: Vec<JoinHandle<Option<Vec<PhoenixEvent>>>>,
    ) -> VybeResult<Vec<PhoenixEvent>> {
        let mut fill_events = Vec::new();

        // Create a stream that buffers up to N join handles concurrently.
        let mut stream = futures::stream::iter(handles).buffered(NUM_TASK_THREADS);
        while let Some(join_result) = stream.next().await {
            let opt_events = join_result?;
            if let Some(events) = opt_events {
                for event in events {
                    if Self::is_market_fill_event(&event.details) {
                        fill_events.push(event);
                    }
                }
            }
        }
        Ok(fill_events)
    }

    /// Identify if a given phoenix event is a fill event
    fn is_market_fill_event(details: &MarketEventDetails) -> bool {
        match *details {
            MarketEventDetails::Fill(_) => true,
            MarketEventDetails::Place(_)
            | MarketEventDetails::Evict(_)
            | MarketEventDetails::Reduce(_)
            | MarketEventDetails::FillSummary(_)
            | MarketEventDetails::Fee(_)
            | MarketEventDetails::TimeInForce(_) => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::VybeTradeFillExtractor;
    const PHOENIX_ADDRESS: &str = "PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY";

    #[tokio::test]
    async fn verify_tx_extractor_construction() {
        match VybeTradeFillExtractor::new("some-api-key", PHOENIX_ADDRESS).await {
            Ok(_txe) => {
                assert!(true);
            }
            Err(err) => {
                println!("{err}");
                assert!(false);
            }
        }
    }
}
