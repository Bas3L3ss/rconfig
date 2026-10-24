use clap::Parser;
use rconfig::{ Cli, Commands, file::open };

fn main() {
    match Cli::parse().command {
        Commands::List { path } => {
            let contents = match open(&path) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            };

            // tokenizing

            // validating

            // parsing
        }
        Commands::Get { path, key } => { todo!() }
        Commands::Validate { path } => { todo!() }
    }
}
