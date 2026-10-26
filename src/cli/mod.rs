use clap::{ Parser, Subcommand };

#[derive(Debug, Parser)]
#[command(name = "rcfg")]
#[command(about = "A .rcfg file parser", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List the values of rconfig file
    List {
        #[arg(help = "Path to the .rcfg file")]
        path: String,
    },
    /// Get the specific value from the rconfig file
    Get {
        #[arg(help = "Path to the .rcfg file")]
        path: String,
        #[arg(help = "Key to retrieve")]
        search: String,
    },
}
