use clap::Parser;
use rconfig::{ Cli, Commands };

fn main() {
    match Cli::parse().command {
        Commands::List { path } => { todo!() }
        Commands::Get { path, key } => { todo!() }
        Commands::Validate { path } => { todo!() }
    }
}
