#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

use rocket::serde::{json::Json, Serialize, Deserialize};
use rocket::State;
use std::sync::{Arc, Mutex};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;
use rocket_contrib::serve::StaticFiles;
use rocket::fs::FileServer;

mod routes;
mod models;
mod schema;
mod db;

use routes::{start_game, make_guess};
use db::DbPool;

#[derive(Clone)]
struct GameState {
    target_stock: models::Stock,
    attempts: Arc<Mutex<i32>>,
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    
    let target_stock = models::Stock {
        id: 1,
        ticker: "AAPL".to_string(),
        market_cap: 2.0e12,
        year_growth: 20.0,
        industry: "Technology".to_string(),
        headquarters: "Cupertino, CA".to_string(),
        stock_price: 150.0,
        dividend_percent: 1.5,
    };
    
    let game_state = GameState {
        target_stock,
        attempts: Arc::new(Mutex::new(0)),
    };

    rocket::build()
        .manage(pool)
        .manage(game_state)
        .mount("/api", routes![start_game, make_guess])
        .mount("/", FileServer::from("static"))
}
