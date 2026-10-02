use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "usherd")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value = "config.json")]
    pub config: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Listen {
        /// Config file path. Default: ./config.json
        #[arg(long, short)]
        config: Option<String>,

        /// IP address to bind the server to. Default: 0.0.0.0
        #[arg(short, long)]
        bind: Option<String>,

        /// Port to listen on. Default: 1984
        #[arg(short, long)]
        port: Option<u16>,

        /// Starts the bootstrap process and replaces the scopes,
        /// I Am map, Usher map, and Transform Registry, basically
        /// making it a fresh install.
        /// Default: false
        #[arg(long)]
        rebuild: Option<bool>,

        /// Root path. Default: ./
        #[arg(long)]
        root: Option<String>,

        /// Enclave path. Default: ./keys/
        #[arg(long)]
        enclave: Option<String>,

        /// Scopes path. Default: ./scopes/
        #[arg(long)]
        scopes: Option<String>,

        /// I Am map path. Default: ./i-am.cbor
        #[arg(long)]
        i_am: Option<String>,

        /// Transform registry path. Default: ./trans_registry.cbor
        #[arg(long)]
        transform_registry: Option<String>,

        /// Transform store path. Default: ./trans_store/
        #[arg(long)]
        transform_store: Option<String>,

        /// Usher map path. Default: ./usher_map.cbor
        #[arg(long)]
        usher_map: Option<String>,

        /// Default: ./bootstrap/
        #[arg(long)]
        bootstrap: Option<String>,

        /// Observer toggle
        #[arg(short, long)]
        observer: bool,

        /// Verbose toogle
        #[arg(short, long)]
        verbose: bool,
    },
    Send {
        #[arg(short, long)]
        usher: String,

        #[arg(short, long)]
        rhex_file: PathBuf,

        #[arg(long)]
        usher_map: Option<PathBuf>,
    },
}
