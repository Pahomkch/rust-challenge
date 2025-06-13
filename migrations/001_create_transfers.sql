CREATE TABLE IF NOT EXISTS transfers (
    ts UInt64,
    address_from String,
    address_to String,
    amount Float64,
    usd_price Float64
) ENGINE = MergeTree()
ORDER BY ts;
