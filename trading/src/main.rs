#![allow(unused_variables, unused_imports, dead_code)]
mod accounting;
mod core;
mod errors;
mod trading_platform;
mod tx;

use crate::accounting::Accounts;
use std::io::{self, Write};

fn read_from_stdin(label: &str) -> String {
    print!("{label}> ");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    buffer.trim().to_string()
}

// fix the error handling here
fn main() {
    let mut ledger = Accounts::new();

    loop {
        let command = read_from_stdin("Enter a command");

        match command.as_str() {
            "deposit" => {
                let acct = read_from_stdin("Enter an account");
                let amt_res = read_from_stdin("Enter an amount to deposit").parse();
                if let Ok(amt) = amt_res {
                    match ledger.deposit(&acct, amt) {
                        Ok(v) => println!("Transaction successful: {v:#?}"),
                        Err(e) => eprintln!("There was a problem depositing: {e:?}"),
                    }
                } else {
                    eprintln!("Not a number: '{:?}'", amt_res);
                }
            }
            "withdraw" => {
                let acct = read_from_stdin("Enter an account");
                let amt_res = read_from_stdin("Enter an amount to withdraw").parse();
                if let Ok(amt) = amt_res {
                    match ledger.withdraw(&acct, amt) {
                        Ok(v) => println!("Transaction successful: {v:#?}"),
                        Err(e) => eprintln!("There was a problem withdrawing: {e:?}"),
                    }
                } else {
                    eprintln!("Not a number: '{:?}'", amt_res);
                }
            }
            "send" => {
                let acct1 = read_from_stdin("Enter the sender account");
                let acct2 = read_from_stdin("Enter the reciever account");
                let amt_res = read_from_stdin("Enter an amount to send").parse();
                if let Ok(amt) = amt_res {
                    match ledger.send(&acct1, &acct2, amt) {
                        Ok(v) => println!("Transaction successful: {v:#?}"),
                        Err(e) => eprintln!("There was a problem sending: {e:?}"),
                    }
                } else {
                    eprintln!("Not a number: '{:?}'", amt_res);
                }
            }
            "print" => println!("{ledger:#?}"),
            "quit" => break,
            _ => println!("Command '{command}' not found"),
        }
    }
}

// println!("Hello, accounting world!");

// // We are using simple &str instances as keys
// // for more sophisticated keys (e.g. hashes)
// // the data type could remain the same
// let bob = "bob";
// let alice = "alice";
// let charlie = "charlie";
// let initial_amount = 100;

// // Creates the basic ledger and a tx log container
// let mut ledger = Accounts::new();
// let mut tx_log = vec![];

// // Deposit an amount to each account
// for signer in &[bob, alice, charlie] {
//     let status = ledger.deposit(*signer, initial_amount);
//     println!("Depositing {} for {}: {:?}", signer, initial_amount, status);
//     // Add the resulting transaction to a list of transactions
//     // .unwrap() will crash the program if the status is an error.
//     tx_log.push(status.unwrap());
// }

// // Send currency from one account (bob) to the other (alice)
// let send_amount = 10_u64;
// let status = ledger.send(bob, alice, send_amount);
// println!(
//     "Sent {} from {} to {}: {:?}",
//     send_amount, bob, alice, status
// );

// // Add both transactions to the transaction log
// let (tx1, tx2) = status.unwrap();
// tx_log.push(tx1);
// tx_log.push(tx2);

// // Withdraw everything from the accounts
// let tx = ledger.withdraw(charlie, initial_amount).unwrap();
// tx_log.push(tx);
// let tx = ledger
//     .withdraw(alice, initial_amount + send_amount)
//     .unwrap();
// tx_log.push(tx);

// // Here we are withdrawing too much and there won't be a transaction
// println!(
//     "Withdrawing {} from {}: {:?}",
//     initial_amount,
//     bob,
//     ledger.withdraw(bob, initial_amount)
// );
// // Withdrawing the expected amount results in a transaction
// let tx = ledger.withdraw(bob, initial_amount - send_amount).unwrap();
// tx_log.push(tx);

// // {:?} prints the Debug implementation, {:#?} pretty-prints it
// println!("Ledger empty: {:?}", ledger);
// println!("The TX log: {:#?}", tx_log);
