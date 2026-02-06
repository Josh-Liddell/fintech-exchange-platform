use crate::trading_platform::TradingPlatform;
use actix_web::{HttpResponse, Responder, get, post, web};
use std::sync::Mutex;
use trading_common::Order;
use trading_common::requests::*;

// type alias
// web::Data uses Arc so no need to specify it here
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

// custom error?, implement responder?

#[post("/account/deposit")]
async fn deposit(req_body: web::Json<AccountUpdateRequest>, data: Platform) -> impl Responder {
    match data
        .lock()
        .unwrap()
        .deposit(&req_body.signer, req_body.amount)
    {
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

#[post("/account/order")]
async fn order(req_body: web::Json<Order>, data: Platform) -> impl Responder {
    match data.lock().unwrap().order(req_body.into_inner()) {
        Ok(r) => HttpResponse::Ok().body(format!("Order placed: {r:#?}")),
        Err(e) => HttpResponse::BadRequest().body(format!("Order failed: {e:?}")),
    }
}

#[get("/orderbook")]
async fn orderbook(data: Platform) -> impl Responder {
    let (bids, asks) = data.lock().unwrap().orderbook();
    web::Json(OrderBookResponse { bids, asks })
}

#[get("/order/history")]
async fn orderhistory(data: Platform) -> impl Responder {
    web::Json(data.lock().unwrap().transaction_log.clone())
}
