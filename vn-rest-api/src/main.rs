//! Restful api for open high low close endpoint
use {
    actix_web::{get, web, App, HttpResponse, HttpServer, Responder},
    anyhow::Result,
    clap::Parser,
    serde::Serialize,
    std::sync::{Arc, Mutex},
    tracing::{info, Level},
    tracing_subscriber::EnvFilter,
    vn_database_core::{models::TradeFill, VybeDatabase},
};

/// Enpoint
const SERVER: &str = "127.0.0.1:8080";

/// Solanas tick size
const TICK_SIZE: f64 = 0.001;

/// Simple cli implementation
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Log level (e.g., error, warn, info, debug, trace)
    #[arg(short, long, default_value = "info")]
    log_level: String,
}

/// Simple open/high/low/close
#[derive(Serialize)]
struct OhlcResponse {
    /// First price
    open: f64,
    /// Highest value
    high: f64,
    /// Lowest value
    low: f64,
    /// Last price
    close: f64,
}

/// Generic application state
struct AppState {
    /// Database abstraction will likely need to be shared, wrap it in the ol' Arc-Mutey
    db: Arc<Mutex<VybeDatabase>>,
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

/// Calculate OHLC from a slice of `TradeFill` records.
fn calculate_ohlc(trades: &[TradeFill]) -> Option<(f64, f64, f64, f64)> {
    if trades.is_empty() {
        return None;
    }

    // The daemon extractor does sort the keys before writing them to the database,
    // but since Solana is a distributed system that won't be reliable, they need to be sorted
    // after we get them here as well.
    let mut sorted = trades.to_vec();
    sorted.sort_by_key(|t| t.event_timestamp);

    let open = sorted.first()?.price_in_ticks as f64 * TICK_SIZE;
    let close = sorted.last()?.price_in_ticks as f64 * TICK_SIZE;
    let high = sorted.iter().map(|t| t.price_in_ticks).max()? as f64 * TICK_SIZE;
    let low = sorted.iter().map(|t| t.price_in_ticks).min()? as f64 * TICK_SIZE;

    Some((open, high, low, close))
}

/// Route to fetch all raw trade fills
#[get("/trade_fills")]
async fn get_trade_fills(data: web::Data<AppState>) -> impl Responder {
    if let Ok(mut db) = data.db.lock() {
        // Fetch all trade fill records.
        match db.get_all_trade_fills() {
            Ok(trades) => HttpResponse::Ok().json(trades),
            Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {e}")),
        }
    } else {
        HttpResponse::InternalServerError().body("Lock error".to_string())
    }
}

/// Handler for the `/ohlc` endpoint.
#[get("/ohlc")]
async fn get_ohlc(data: web::Data<AppState>) -> impl Responder {
    let mut db = match data.db.lock() {
        Ok(guard) => guard,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Lock error: {e}")),
    };

    // Fetch all trade fill records.
    let all_trades = match db.get_all_trade_fills() {
        Ok(trades) => trades,
        Err(e) => return HttpResponse::InternalServerError().body(format!("DB error: {e}")),
    };

    match calculate_ohlc(&all_trades) {
        Some((open, high, low, close)) => {
            let response = OhlcResponse {
                open,
                high,
                low,
                close,
            };
            HttpResponse::Ok().json(response)
        }
        None => HttpResponse::NotFound().body("No trade fill data available"),
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    // Parse command line arguments.
    let args = Args::parse();
    let level = convert_log_level(&args.log_level);

    // Filter out the noise from 3rd party libraries
    let filter = EnvFilter::builder()
        .with_default_directive(level.into())
        .from_env()?;

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .compact()
        .init();

    let db = VybeDatabase::new()?;
    let shared_app_state = web::Data::new(AppState {
        db: Arc::new(Mutex::new(db)),
    });

    info!("Starting server at http://{SERVER}");
    let _ = HttpServer::new(move || {
        App::new()
            .app_data(shared_app_state.clone())
            .service(get_trade_fills)
            .service(get_ohlc)
    })
    .bind(SERVER)?
    .run()
    .await;

    Ok(())
}
