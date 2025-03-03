// @generated automatically by Diesel CLI.

diesel::table! {
    trade_fills (id) {
        id -> Int4,
        event_timestamp -> Int8,
        price_in_ticks -> Int8,
        base_lots_filled -> Int8,
    }
}
