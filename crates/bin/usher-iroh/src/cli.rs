use clap::{Parser, Subcommand};

use crate::config::UsherIrohConfig;

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[arg(
        short,
        long,
        global = true,
        value_name = "PATH",
        default_value = "./keys"
    )]
    pub enclave: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Listen {
        #[arg(short, long, default_value = UsherIrohConfig::DEFAULT_CONFIG)]
        config: String,

        #[arg(long, short)]
        bind: Option<String>,

        #[arg(long, short)]
        port: Option<u16>,

        #[arg(long, short)]
        root: Option<String>,

        #[arg(long, short)]
        i_am: Option<String>,

        #[arg(long, short)]
        scopes: Option<String>,

        #[arg(long, short)]
        usher_map: Option<String>,

        #[arg(long)]
        trans_registry: Option<String>,

        #[arg(long)]
        trans_store: Option<String>,

        #[arg(long)]
        bootstrap: Option<String>,

        #[arg(long)]
        rebuild: bool,

        #[arg(long, short)]
        observer: bool,
    },
    Submit {
        #[arg(short, long)]
        chatter: String,
    },
}
