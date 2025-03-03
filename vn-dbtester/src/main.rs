//! Simple driver, will be deleted after wards

use {
    anyhow::Result,
    clap::Parser,
    tracing::{error, info, warn, Level},
    tracing_subscriber::EnvFilter,
    vn_database_core::VybeDatabase,
};

/// Simple cli implementation
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Fetch a database entry by its id
    #[arg(short, long)]
    id: i32,
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

fn main() -> Result<()> {
    // Parse command line arguments.
    let args = Args::parse();
    let level = convert_log_level(&args.log_level);
    let trade_id = args.id;

    // Filter out the noise from 3rd party libraries
    let filter = EnvFilter::builder()
        .with_default_directive(level.into())
        .from_env()?;

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .compact()
        .init();

    info!("Starting Vybe database test tool");

    match &mut VybeDatabase::new() {
        Ok(db) => match db.get_trade_fill_by_id(trade_id) {
            Ok(trades) => {
                if !trades.is_empty() {
                    for trade in trades {
                        println!("{trade:#?}");
                    }
                } else {
                    warn!("No trade fill entry at id {trade_id}");
                }
            }
            Err(e) => {
                error!("{e}");
            }
        },
        Err(e) => {
            error!("{e}");
        }
    }
    Ok(())
}
