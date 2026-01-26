use reqwest::Client;
use serde::Serialize;
use trading_common::{PartialOrder, tx::Tx};
// clients that have this trait can interact with my paltform using itself with these methods... pretty cool
// it has abilities to work with my platform
pub trait Trading {
    async fn platform_post<T: Serialize>(&self, req: &T, path: &str);
    async fn print_orderbook(&self);
    async fn print_txlog(&self);
}

impl Trading for Client {
    async fn platform_post<T: Serialize>(&self, req: &T, path: &str) {
        let resp = self
            .post(format!("http://localhost:8080/{path}"))
            .json(req)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        println!("Response: {}", resp);
    }

    async fn print_orderbook(&self) {
        let res = reqwest::get("http://localhost:8080/orderbook")
            .await
            .unwrap()
            .json::<Vec<PartialOrder>>()
            .await
            .unwrap();

        println!("{:#?}", res);
    }

    async fn print_txlog(&self) {
        let res = reqwest::get("http://localhost:8080/order/history")
            .await
            .unwrap()
            .json::<Vec<Tx>>()
            .await
            .unwrap();

        println!("{:#?}", res);
    }
}
