use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use std::sync::Mutex;
use crate::models::Stock;
use super::GameState;

#[derive(Deserialize)]
pub struct StockGuess {
    ticker: String,
}

#[derive(Serialize)]
pub struct Hint {
    market_cap_hint: String,
    growth_hint: String,
    industry_hint: String,
    location_hint: String,
    stock_price_hint: String,
    dividend_hint: String,
}

#[derive(Serialize)]
pub struct GuessResult {
    correct: bool,
    hints: Hint,
    remaining_attempts: i32,
}

#[post("/start")]
pub fn start_game(state: &State<GameState>) -> &'static str {
    let mut attempts = state.attempts.lock().unwrap();
    *attempts = 0;
    "New game started! You have 10 guesses."
}

#[post("/guess", format = "json", data = "<guess>")]
pub fn make_guess(guess: Json<StockGuess>, state: &State<GameState>) -> Json<GuessResult> {
    let target = &state.target_stock;
    let mut attempts = state.attempts.lock().unwrap();
    *attempts += 1;
    
    let remaining_attempts = 10 - *attempts;

    let correct = guess.ticker == target.ticker;

    let hints = Hint {
        market_cap_hint: if guess.ticker > target.ticker { "higher" } else { "lower" }.to_string(),
        growth_hint: if guess.ticker > target.ticker { "higher" } else { "lower" }.to_string(),
        industry_hint: if guess.ticker == target.ticker { "match" } else { "different" }.to_string(),
        location_hint: "some location hint".to_string(),
        stock_price_hint: if guess.ticker > target.ticker { "higher" } else { "lower" }.to_string(),
        dividend_hint: if guess.ticker > target.ticker { "higher" } else { "lower" }.to_string(),
    };

    Json(GuessResult {
        correct,
        hints,
        remaining_attempts,
    })
}
