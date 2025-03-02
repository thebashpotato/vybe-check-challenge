-- Your SQL goes here
CREATE TABLE trade_fills (
    id SERIAL PRIMARY KEY,
    event_timestamp BIGINT NOT NULL, -- UNIX timestamp (in seconds)
    price_in_ticks BIGINT NOT NULL,   -- Raw price expressed in ticks
    base_lots_filled BIGINT NOT NULL  -- Volume traded (in base lots)
);
