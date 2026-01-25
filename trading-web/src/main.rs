#![allow(unused_variables, unused_imports, dead_code)]
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use trading_common::errors::ApplicationError;
use trading_platform::TradingPlatform;

mod accounting;
mod matching;
mod trading_platform;

#[derive(Serialize, Deserialize)]
struct AccountUpdateRequest {
    signer: String,
    amount: u64,
}

#[derive(Serialize, Deserialize)]
struct AccountBalanceRequest {
    signer: String,
}

#[derive(Serialize, Deserialize)]
struct SendRequest {
    sender: String,
    recipient: String,
    amount: u64,
}

// #[derive(Debug)]
// struct TradingError(ApplicationError); // ?????

// type alias
// web::Data uses Arc so I dont need to specify it here
type Platform = web::Data<Mutex<TradingPlatform>>;

#[post("/account")]
async fn account(req_body: web::Json<AccountBalanceRequest>, data: Platform) -> impl Responder {
    match data.lock().unwrap().balance_of(&req_body.signer) {
        Ok(b) => {
            HttpResponse::Ok().body(format!("Balance of {} account is: {}", &req_body.signer, b))
        }
        Err(e) => HttpResponse::BadRequest().body(format!("Balance check failed: {e:?}")),
    }
}

#[post("/account/deposit")]
async fn deposit(req_body: web::Json<AccountUpdateRequest>, data: Platform) -> impl Responder {
    match data
        .lock()
        .unwrap()
        .deposit(&req_body.signer, req_body.amount)
    {
        // okay problem with returning string is that it is a 200 ok response and maybe we dont want that,
        // CONSIDER implementing responder for a custom error?
        Ok(r) => HttpResponse::Ok().body(format!("Deposit success: {r:#?}")),
        Err(e) => HttpResponse::BadRequest().body(format!("Deposit failed: {e:?}")),
    }
}

#[post("/account/withdraw")]
async fn withdraw(req_body: web::Json<AccountUpdateRequest>, data: Platform) -> impl Responder {
    match data
        .lock()
        .unwrap()
        .withdraw(&req_body.signer, req_body.amount)
    {
        Ok(r) => HttpResponse::Ok().body(format!("Withdraw success: {r:#?}")),
        Err(e) => HttpResponse::BadRequest().body(format!("Withdraw failed: {e:?}")),
    }
}

#[post("/account/send")]
async fn send(req_body: web::Json<SendRequest>, data: Platform) -> impl Responder {
    match data
        .lock()
        .unwrap()
        .send(&req_body.sender, &req_body.recipient, req_body.amount)
    {
        Ok(r) => HttpResponse::Ok().body(format!("Send success: {r:#?}")),
        Err(e) => HttpResponse::BadRequest().body(format!("Send failed: {e:?}")),
    }
}

// #[post("/account/order")]
// async fn order(req_body: Order, data: Platform) -> impl Responder {}

// // it says get calls dont have parameters, but I think they do?
// #[get("/orderbook")]
// async fn orderbook(data: Platform) -> impl Responder {}

// #[get("/order/history")]
// async fn orderhistory(data: Platform) -> impl Responder {}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let platform = web::Data::new(Mutex::new(TradingPlatform::new()));

    // serve forever
    println!("Starting server on http://127.0.0.1:8080...");
    HttpServer::new(move || {
        App::new()
            .app_data(platform.clone())
            .service(account)
            .service(deposit)
            .service(withdraw)
            .service(send)
        // .service(order)
        // .service(orderbook)
        // .service(orderhistory)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
