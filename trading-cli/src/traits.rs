use reqwest::{Client, Error};
use serde::Serialize;
use tabled::{Table, settings::Style};
use trading_common::{requests::OrderBookResponse, tx::Tx};

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
        let res: OrderBookResponse = self
            .get("http://localhost:8080/orderbook")
            .send()
            .await?
            .json()
            .await?;

        let OrderBookResponse { mut bids, mut asks } = res;
        bids.sort_by(|a, b| b.price.cmp(&a.price));
        asks.sort_by(|a, b| b.price.cmp(&a.price));

        let mut table = Table::new(bids);
        let mut table2 = Table::new(asks);
        table.with(Style::modern());
        table2.with(Style::modern());
        println!("Sell orders:\n{}\n\nBuy orders:\n{}", table, table2);

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
