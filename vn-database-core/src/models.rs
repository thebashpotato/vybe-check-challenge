//! Diesel Models

use {
    crate::VybeDatabaseError,
    diesel::prelude::*,
    phoenix_sdk::sdk_client::{MarketEventDetails, PhoenixEvent},
    std::convert::TryFrom,
};

/// Represents a trade fill event as stored in the database.
/// Used to read trade fill records
#[allow(dead_code)]
#[derive(Debug, Queryable, Selectable, Eq, PartialEq)]
#[diesel(table_name = crate::schema::trade_fills)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TradeFill {
    /// Primary key of the trade fill record.
    pub id: i32,
    /// The Unix timestamp (in seconds) when the trade fill event occurred.
    pub event_timestamp: i64,
    /// The price of the trade expressed in ticks.
    /// Multiply by the market's tick size to convert to a standard unit price.
    pub price_in_ticks: i64,
    /// The volume of the base token (in lots) that was filled in this trade event.
    pub base_lots_filled: i64,
}

/// Represents a new trade fill event to be inserted into the database.
/// Used to post new trade fill records.
#[derive(Insertable)]
#[diesel(table_name = crate::schema::trade_fills)]
pub struct NewTradeFill {
    /// The Unix timestamp (in seconds) when the trade fill event occurred.
    pub event_timestamp: i64,
    /// The price of the trade expressed in ticks.
    /// This raw value will be converted to the standard price using the market's tick size.
    pub price_in_ticks: i64,
    /// The volume of the base token (in lots) that was filled in this trade event.
    pub base_lots_filled: i64,
}

impl TryFrom<PhoenixEvent> for NewTradeFill {
    type Error = VybeDatabaseError;

    fn try_from(event: PhoenixEvent) -> Result<Self, Self::Error> {
        match event.details {
            MarketEventDetails::Fill(fill) => Ok(NewTradeFill {
                event_timestamp: event.timestamp,
                price_in_ticks: fill.price_in_ticks as i64,
                base_lots_filled: fill.base_lots_filled as i64,
            }),
            _ => Err(VybeDatabaseError::InvalidPhoenixEvent),
        }
    }
}
