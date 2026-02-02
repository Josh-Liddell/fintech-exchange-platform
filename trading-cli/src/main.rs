mod traits;

use std::error::Error;
use std::io::{self, Write};
use trading_common::Order;
use trading_common::requests::{AccountBalanceRequest, AccountUpdateRequest, SendRequest};
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

fn read_order() -> Result<Order, Box<dyn Error>> {
    let signer = read_from_stdin("Enter the account");
    let side = read_from_stdin("Enter the side").parse()?;
    let amount = read_from_stdin("Enter an amount").parse()?;
    let price = read_from_stdin("Enter a price").parse()?;
    let order = Order {
        price,
        amount,
        side,
        signer,
    };
    Ok(order)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    loop {
        let command = read_from_stdin("Enter a command");
        match command.as_str() {
            "deposit" | "d" => {
                let signer = read_from_stdin("Account");
                let raw_amount = read_from_stdin("Amount").parse();
                if let Ok(amount) = raw_amount {
                    let req = AccountUpdateRequest { signer, amount };
                    let resp = client.platform_post(&req, "account/deposit").await?;
                    println!("{resp}");
                } else {
                    eprintln!("Not a number: {:?}", raw_amount);
                }
            }
            "withdraw" | "w" => {
                let signer = read_from_stdin("Account");
                let raw_amount = read_from_stdin("Amount").parse();
                if let Ok(amount) = raw_amount {
                    let req = AccountUpdateRequest { signer, amount };
                    let resp = client.platform_post(&req, "account/withdraw").await?;
                    println!("{resp}");
                } else {
                    eprintln!("Not a number: {:?}", raw_amount);
                }
            }
            "send" | "s" => {
                let sender = read_from_stdin("Sender account");
                let recipient = read_from_stdin("Reciever account");
                let raw_amount = read_from_stdin("Amount").parse();
                if let Ok(amount) = raw_amount {
                    let req = SendRequest {
                        sender,
                        recipient,
                        amount,
                    };
                    let resp = client.platform_post(&req, "account/send").await?;
                    println!("{resp}");
                } else {
                    eprintln!("Not a number: {:?}", raw_amount);
                }
            }
            "order" | "o" => match read_order() {
                Ok(order) => {
                    let resp = client.platform_post(&order, "account/order").await?;
                    println!("{resp}");
                }
                Err(e) => {
                    eprintln!("Invalid order parameters: {e:?}");
                }
            },
            "orderbook" | "ob" => {
                client.print_orderbook().await?;
            }
            "txlog" => {
                client.print_txlog().await?;
            }
            "print" => {
                let signer = read_from_stdin("Enter the account");
                let req = AccountBalanceRequest { signer };
                let resp = client.platform_post(&req, "account").await?;
                println!("{resp}");
            }
            "quit" | "q" => break,
            _ => println!("Command '{command}' not found"),
        }
    }
    Ok(())
}
