CREATE TABLE stocks (
    id SERIAL PRIMARY KEY,
    ticker VARCHAR NOT NULL,
    market_cap FLOAT8 NOT NULL,
    year_growth FLOAT8 NOT NULL,
    industry VARCHAR NOT NULL,
    headquarters VARCHAR NOT NULL,
    stock_price FLOAT8 NOT NULL,
    dividend_percent FLOAT8 NOT NULL
);
