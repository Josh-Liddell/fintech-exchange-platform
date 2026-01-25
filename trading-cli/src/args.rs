use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    #[arg(short, long)]
    pub orderbook: bool,

    #[arg(short, long)]
    pub balance: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

// if needed can create subcommands and match them to functions etc.
#[derive(Subcommand, Debug)]
pub enum Commands {
    Deposit {
        #[arg(short, long)]
        signer: String,

        #[arg(short, long, default_value_t = 10000)]
        amount: u64,
    },

    Withdraw {
        #[arg(short, long)]
        signer: String,

        #[arg(short, long)]
        amount: u64,
    },
    Order,
}
