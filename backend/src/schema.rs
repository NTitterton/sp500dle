
table! {
    stocks (id) {
        id -> Int4,
        ticker -> Varchar,
        market_cap -> Float8,
        year_growth -> Float8,
        industry -> Varchar,
        headquarters -> Varchar,
        stock_price -> Float8,
        dividend_percent -> Float8,
    }
}
