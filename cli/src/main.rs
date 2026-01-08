use clap::Parser;

struct Cli {
    args: Args,
    // more data for the app here
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

// if needed can create subcommands and match them to functions etc.

impl Cli {
    fn new() -> Self {
        let args = Args::parse();
        Self { args }
    }

    fn run(self) {
        for _ in 0..self.args.count {
            println!("Hello {}!", self.args.name);
        }
    }
}

fn main() {
    let app = Cli::new(); // parse args and create app
    app.run() // execute logic based on args
}
