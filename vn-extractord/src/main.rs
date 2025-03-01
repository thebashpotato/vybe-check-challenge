//! The main CLI application

use {
    anyhow::Result,
    clap::Parser,
    tracing::{error, info, Level},
    tracing_subscriber::EnvFilter,
    vn_extractord_core::VybeTradeFillExtractor,
};

/// Mainnet address of active SOL/USDC Market
///
/// From <https://ellipsis-labs.gitbook.io/phoenix-dex/tRIkEFlLUzWK9uKO3W2V/getting-started/technical-overview/market-addresses>
const PHOENIX_SOLUSDC_MARKET_ADDRESS: &str = "4DoNfFBfF7UokCC2FQzriy7yHK6DY6NVdYpuekQ5pRgg";

/// Simple cli implementation
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Helium RPC API Key
    #[arg(short, long)]
    api_key: String,
    /// Log level (e.g., error, warn, info, debug, trace)
    #[arg(short, long, default_value = "info")]
    log_level: String,
}

/// Converts cli argument string log level to tracing `Level`
fn convert_log_level(level_str: &str) -> Level {
    match level_str.to_lowercase().as_str() {
        "error" => Level::ERROR,
        "warn" | "warning" => Level::WARN,
        "debug" => Level::DEBUG,
        "trace" => Level::TRACE,
        _ => Level::INFO,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments.
    let args = Args::parse();
    let level = convert_log_level(&args.log_level);

    // Filter out the noise from 3rd party libraries
    let filter = EnvFilter::builder()
        .with_default_directive(level.into())
        .from_env()?
        .add_directive("hyper=info".parse()?)
        .add_directive("solana_rpc_client=info".parse()?)
        .add_directive("reqwest=info".parse()?)
        .add_directive("rustls=info".parse()?)
        .add_directive("solana_rpc_client=info".parse()?)
        .add_directive("h2=info".parse()?);

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .compact()
        .init();

    info!("Starting vybe-network extractor daemon");

    // Use the API key provided via the command line.
    let vfe =
        VybeTradeFillExtractor::new(args.api_key.as_str(), PHOENIX_SOLUSDC_MARKET_ADDRESS).await?;
    loop {
        if let Err(e) = vfe.run().await {
            error!("{e}");
            break;
        }

        // NOTE: this is a basic polling loop. If there are >1000 signatures in 200ms,
        // events may get dropped.
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }

    info!("Shutting down vybe-network extractor daemon");
    Ok(())
}
