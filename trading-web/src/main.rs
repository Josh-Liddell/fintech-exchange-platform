mod accounting;
mod matching;
mod routes;
mod trading_platform;

use actix_web::{App, HttpServer, web};
use routes::*;
use std::sync::Mutex;
use trading_platform::TradingPlatform;

// struct TradingError(ApplicationError); ??

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let platform = web::Data::new(Mutex::new(TradingPlatform::new()));

    println!("Starting server on http://127.0.0.1:8080...");
    HttpServer::new(move || {
        App::new()
            .app_data(platform.clone())
            .service(account)
            .service(deposit)
            .service(withdraw)
            .service(send)
            .service(order)
            .service(orderbook)
            .service(orderhistory)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
