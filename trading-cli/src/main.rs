mod traits;

use std::io::{self, Write};
use trading_common::requests::{AccountBalanceRequest, AccountUpdateRequest, SendRequest};
use trading_common::{Order, Side};
use traits::Trading;

fn read_from_stdin(label: &str) -> String {
    print!("{label}> ");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    buffer.trim().to_string().to_lowercase()
}

// FIGURE OUT ERROR HANDLING
#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    loop {
        let command = read_from_stdin("Enter a command");
        match command.as_str() {
            "deposit" | "d" => {
                let signer = read_from_stdin("Enter an account");
                let amount: u64 = read_from_stdin("Enter an amount")
                    .parse()
                    .expect("Enter an integer");
                let req = AccountUpdateRequest { signer, amount };
                client.platform_post(&req, "account/deposit").await;
            }
            "withdraw" | "w" => {
                let signer = read_from_stdin("Enter an account");
                let amount: u64 = read_from_stdin("Enter an amount")
                    .parse()
                    .expect("Enter an integer");
                let req = AccountUpdateRequest { signer, amount };
                client.platform_post(&req, "account/withdraw").await;
            }
            "send" => {
                let sender = read_from_stdin("Enter the sender account");
                let recipient = read_from_stdin("Enter the reciever account");
                let amount: u64 = read_from_stdin("Enter an amount to send")
                    .parse()
                    .expect("Enter an integer");
                let req = SendRequest {
                    sender,
                    recipient,
                    amount,
                };
                client.platform_post(&req, "account/send").await;
            }
            "order" => {
                let signer = read_from_stdin("Enter the account");
                let side: Side = read_from_stdin("Enter the side")
                    .parse()
                    .expect("Enter buy or sell");
                let amount: u64 = read_from_stdin("Enter an amount")
                    .parse()
                    .expect("Enter an integer");
                let price: u64 = read_from_stdin("Enter a price")
                    .parse()
                    .expect("Enter an integer");
                let req = Order {
                    price,
                    amount,
                    side,
                    signer,
                };
                client.platform_post(&req, "account/order").await;
            }
            "orderbook" => {
                client.print_orderbook().await;
            }
            "txlog" => {
                client.print_txlog().await;
            }
            "print" => {
                let signer = read_from_stdin("Enter the account");
                let req = AccountBalanceRequest { signer };
                client.platform_post(&req, "account").await;
            }
            "quit" | "q" => break,
            _ => println!("Command '{command}' not found"),
        }
    }
}
