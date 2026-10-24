use clap::Parser;
use rconfig::{ Cli, Commands, commands::list };

fn main() {
    match Cli::parse().command {
        Commands::List { path } => {
            if let Err(e) = list(&path) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }

        Commands::Get { path, key } => { todo!() }
        Commands::Validate { path } => { todo!() }
    }
}
