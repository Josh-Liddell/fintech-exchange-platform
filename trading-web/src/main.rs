#![allow(unused_variables, unused_imports, dead_code)]
use trading_common::errors::ApplicationError;
use warp::reject::Reject;

mod accounting;
mod matching;
mod trading_platform;

struct AccountUpdateRequest {
    signer: String,
    amount: u64,
}

struct AccountBalanceRequest {
    signer: String,
}

struct SendRequest {
    sender: String,
    recipient: String,
    amount: u64,
}

#[derive(Debug)]
struct TradingError(ApplicationError);

impl Reject for TradingError {}

fn main() {
    println!("Hello, world!");
}
