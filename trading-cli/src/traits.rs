use reqwest::{Client, Error};
use serde::Serialize;
use trading_common::{PartialOrder, tx::Tx};

// clients that have this trait can interact with my paltform using itself with these methods, abilities to work with the platform
pub trait Trading {
    async fn platform_post<T: Serialize>(
        &self,
        req: &T,
        path: &str,
    ) -> Result<String, reqwest::Error>;
    async fn print_orderbook(&self) -> Result<(), Error>;
    async fn print_txlog(&self) -> Result<(), Error>;
}

impl Trading for Client {
    async fn platform_post<T: Serialize>(&self, req: &T, path: &str) -> Result<String, Error> {
        let resp = self
            .post(format!("http://localhost:8080/{path}"))
            .json(req)
            .send()
            .await?
            .text()
            .await?;

        Ok(resp)
    }

    async fn print_orderbook(&self) -> Result<(), Error> {
        let res: Vec<PartialOrder> = self
            .get("http://localhost:8080/orderbook")
            .send()
            .await?
            .json()
            .await?;

        println!("{:#?}", res);
        Ok(())
    }

    async fn print_txlog(&self) -> Result<(), Error> {
        let res: Vec<Tx> = self
            .get("http://localhost:8080/order/history")
            .send()
            .await?
            .json()
            .await?;

        println!("{:#?}", res);
        Ok(())
    }
}
