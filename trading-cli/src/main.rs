mod args;
mod orderform;

use args::{Args, Commands};
use clap::Parser;
use trading_common::PartialOrder;
use trading_common::requests::*;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let client = reqwest::Client::new();

    if let Some(signer) = args.balance {
        let req = AccountBalanceRequest { signer };

        let res = client
            .post("http://localhost:8080/account")
            .json(&req)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        println!("{res}");
    }

    if args.orderbook {
        let res = reqwest::get("http://localhost:8080/orderbook")
            .await
            .unwrap()
            .json::<Vec<PartialOrder>>()
            .await
            .unwrap();

        println!("{:#?}", res);
    }

    match args.command {
        Some(Commands::Deposit { signer, amount }) => {
            let req = AccountUpdateRequest { signer, amount };

            let res = client
                .post("http://localhost:8080/account/deposit")
                .json(&req)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            println!("Response: {}", res);
        }
        Some(Commands::Order) => {
            orderform::showform();
        }
        _ => {}
    }
}

// #![allow(unused_variables, unused_imports, dead_code)]
// mod accounting;
// mod matching;
// mod trading_platform;

// use accounting::Accounts;
// use std::io::{self, Write};
// use trading_common::{Order, Side};
// use trading_platform::TradingPlatform;

// fn read_from_stdin(label: &str) -> String {
//     print!("{label}> ");
//     io::stdout().flush().unwrap();
//     let mut buffer = String::new();
//     io::stdin()
//         .read_line(&mut buffer)
//         .expect("Failed to read line");
//     buffer.trim().to_string()
// }

// fn main() {
//     let mut platform = TradingPlatform::new();

//     loop {
//         let command = read_from_stdin("Enter a command");

//         match command.as_str() {
//             "deposit" => {
//                 let acct = read_from_stdin("Enter an account");
//                 let amt_res = read_from_stdin("Enter an amount to deposit").parse();
//                 if let Ok(amt) = amt_res {
//                     match platform.deposit(&acct, amt) {
//                         Ok(v) => println!("Transaction successful: {v:#?}"),
//                         Err(e) => eprintln!("There was a problem depositing: {e:?}"),
//                     }
//                 } else {
//                     eprintln!("Not a number: '{:?}'", amt_res);
//                 }
//             }
//             "withdraw" => {
//                 let acct = read_from_stdin("Enter an account");
//                 let amt_res = read_from_stdin("Enter an amount to withdraw").parse();
//                 if let Ok(amt) = amt_res {
//                     match platform.withdraw(&acct, amt) {
//                         Ok(v) => println!("Transaction successful: {v:#?}"),
//                         Err(e) => eprintln!("There was a problem withdrawing: {e:?}"),
//                     }
//                 } else {
//                     eprintln!("Not a number: '{:?}'", amt_res);
//                 }
//             }
//             "send" => {
//                 let acct1 = read_from_stdin("Enter the sender account");
//                 let acct2 = read_from_stdin("Enter the reciever account");
//                 let amt_res = read_from_stdin("Enter an amount to send").parse();
//                 if let Ok(amt) = amt_res {
//                     match platform.send(&acct1, &acct2, amt) {
//                         Ok(v) => println!("Transaction successful: {v:#?}"),
//                         Err(e) => eprintln!("There was a problem sending: {e:?}"),
//                     }
//                 } else {
//                     eprintln!("Not a number: '{:?}'", amt_res);
//                 }
//             }
//             "order" => {
//                 let signer = read_from_stdin("Enter the account to place order");
//                 let side: Side = read_from_stdin("Buy or Sell order?").parse().unwrap();
//                 let amount = read_from_stdin("Amount?").parse().unwrap();
//                 let price = read_from_stdin("What is the price?").parse().unwrap();

//                 if let Err(e) = platform.order(Order {
//                     price,
//                     amount,
//                     side,
//                     signer,
//                 }) {
//                     eprintln!("Error processing order: {e:?}");
//                 }
//             }
//             "orderbook" => println!("{:#?}", platform.orderbook()),
//             "txlog" => println!("{:#?}", platform.transaction_log),
//             "print" => println!("{:#?}", platform.accounts.accounts),
//             "quit" => break,
//             _ => println!("Command '{command}' not found"),
//         }
//     }
// }
