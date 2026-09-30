use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "genesis")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Now {
        #[arg(long)]
        output: Option<String>,
        #[arg(long)]
        lat: Option<f64>,
        #[arg(long)]
        long: Option<f64>,
    },
}
