use clap::Parser;
use rconfig::{ Cli, Commands, commands::{ Error, format, get, list, validate } };

fn main() {
    match Cli::parse().command {
        Commands::List { path } => {
            match list(&path) {
                Ok(result) => {
                    println!("{result:#?}");
                }
                Err(Error::File(e)) => {
                    eprintln!("File error: {e}");
                    std::process::exit(1);
                }
                Err(Error::Parser(e)) => {
                    eprintln!("Syntax error: {e}");
                    std::process::exit(1);
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::Format { path } => {
            match format(&path) {
                Ok(_) => {
                    println!("Format completed");
                }
                Err(Error::File(e)) => {
                    eprintln!("File error: {e}");
                    std::process::exit(1);
                }
                Err(Error::Parser(e)) => {
                    eprintln!("Syntax error: {e}");
                    std::process::exit(1);
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::Validate { path } => {
            match validate(&path) {
                Ok(_) => {
                    println!("The config is valid!");
                }
                Err(Error::File(e)) => {
                    eprintln!("File error: {e}");
                    std::process::exit(1);
                }
                Err(Error::Parser(e)) => {
                    eprintln!("Syntax error: {e}");
                    std::process::exit(1);
                }
                Err(e) => {
                    eprintln!("Error: {e}");
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
                Err(Error::File(e)) => {
                    eprintln!("File error: {e}");
                    std::process::exit(1);
                }
                Err(Error::Parser(e)) => {
                    eprintln!("Syntax error: {e}");
                    std::process::exit(1);
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
        }
    }
}
