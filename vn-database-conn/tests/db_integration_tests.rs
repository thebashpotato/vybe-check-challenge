//! These tests require a postgresql database to be running locally with
//! credentials that match those found the .env file in the root of the project
//!
//! These tests are ran with `just itest` the database is migrated and cleared afterwards
#![allow(clippy::unwrap_used)]

#[cfg(feature = "integration_tests")]
use vn_database_conn::{
    models::{NewTradeFill, TradeFill},
    DatabaseConn, VybeDatabaseError,
};

#[cfg(feature = "integration_tests")]
#[test]
fn database_connection_test() {
    let dbconn_res = DatabaseConn::new();
    assert_eq!(dbconn_res.is_ok(), true);
}

#[cfg(feature = "integration_tests")]
#[test]
fn database_read_write_test() -> Result<(), VybeDatabaseError> {
    let db = &mut DatabaseConn::new()?;
    // Write Test

    let new_trade_fill = NewTradeFill {
        event_timestamp: 1740956436,
        price_in_ticks: 177096,
        base_lots_filled: 16782,
    };

    let returned_trade_fill: TradeFill = db.create_trade_fill(&new_trade_fill)?;
    assert_eq!(
        returned_trade_fill.event_timestamp,
        new_trade_fill.event_timestamp
    );
    assert_eq!(
        returned_trade_fill.price_in_ticks,
        new_trade_fill.price_in_ticks
    );
    assert_eq!(
        returned_trade_fill.base_lots_filled,
        new_trade_fill.base_lots_filled
    );

    // Read Test
    let trade_fills = db.get_trade_fill_by_id(1)?;
    let trade_fill_opt = trade_fills.first();
    assert!(trade_fill_opt.is_some());
    let trade_fill = trade_fill_opt.unwrap();
    assert_eq!(trade_fill, &returned_trade_fill);

    Ok(())
}
