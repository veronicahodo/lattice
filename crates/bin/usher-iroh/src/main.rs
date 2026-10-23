use std::{path::PathBuf, process::exit, sync::Arc};

use crate::{
    cli::{Cli, Commands},
    comms::chatter::ChatterRequest,
    config::UsherIrohConfig,
};
use clap::Parser;
use util::{verbose_print, verbose_println};

pub mod append;
pub mod author;
pub mod bps;
pub mod check;
pub mod cli;
pub mod comms;
pub mod config;
pub mod gossip;
pub mod process;
pub mod quorum;
pub mod retrieve;
pub mod server;
pub mod trans;
pub mod usher;

#[tokio::main]
async fn main() {
    println!("*****************************************************");
    println!("* LATTICE USHER DAEMON (w/ Iroh!)                   *");
    println!("*****************************************************");
    println!("Starting what is probably a terrible excuse for a server...");
    println!("Bro, welcome to the dungeon lol... ⚔️🐉");
    let cli = Cli::parse();
    match cli.command {
        Commands::Listen {
            config,
            bind,
            port,
            root,
            i_am,
            scopes,
            usher_map,
            trans_registry,
            trans_store,
            bootstrap,
            rebuild,
            observer,
        } => {
            let enclave = cli.enclave;
            let verbose = cli.verbose;
            verbose_print(format!("Loading config ({})...", config), verbose);
            let config_string = config.clone();
            let config = UsherIrohConfig::from_disk(config);
            if config.is_err() {
                println!("Failed to load config. Error: {}", config.err().unwrap());
                exit(1);
            }
            let mut config = config.unwrap();
            let merge_status = config.merge(
                config_string,
                bind,
                port,
                root,
                enclave,
                i_am,
                scopes,
                usher_map,
                trans_registry,
                trans_store,
                bootstrap,
                rebuild,
                observer,
                verbose,
            );
            if merge_status.is_err() {
                println!("Failed to merge config data with cli");
                exit(2);
            }

            verbose_println("Beginning LISTEN...".to_string(), verbose);
            let run_status = server::run(Arc::new(config)).await;
            if run_status.is_err() {
                println!("Server failed with error: {}", run_status.err().unwrap());
                exit(3);
            }
        }
        Commands::Submit { chatter } => {
            let chatter: Vec<ChatterRequest> =
                cbor2::from_slice(&std::fs::read(&PathBuf::from(chatter)).unwrap()).unwrap();
            let _ = comms::submit::submit(chatter).await;
        }
    }
}
