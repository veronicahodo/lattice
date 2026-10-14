use std::process::exit;

use crate::{
    cli::{Cli, Commands},
    config::UsherIrohConfig,
};
use clap::Parser;
use util::{verbose_print, verbose_println};

pub mod cli;
pub mod config;
pub mod gossip;
pub mod retrieve;
pub mod server;

fn main() {
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
        } => {
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
            let run_status = server::run(&config);
            if run_status.is_err() {
                println!("Server failed with error: {}", run_status.err().unwrap());
                exit(3);
            }
        }
    }
}
