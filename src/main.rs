use clap::Parser;
use rconfig::{ Cli, Commands, commands::{ get, list } };

fn main() {
    match Cli::parse().command {
        Commands::List { path } => {
            match list(&path) {
                Ok(result) => {
                    println!("{result:#?}");
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Get { path, search } => {
            match get(&path, &search) {
                Ok(val) => {
                    match val {
                        Some(v) => println!("{search}: {v:#?}"),
                        None => {
                            eprintln!("Error: Key not found");
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Validate { path } => { todo!() }
    }
}
