use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, Serialize, Clone)]
pub struct Stock {
    pub id: i32,
    pub ticker: String,
    pub market_cap: f64,
    pub year_growth: f64,
    pub industry: String,
    pub headquarters: String,
    pub stock_price: f64,
    pub dividend_percent: f64,
}
