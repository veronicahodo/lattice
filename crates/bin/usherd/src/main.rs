use crate::{
    cli::{Cli, Commands},
    config::UsherdConfig,
};
use base64::{Engine as _, engine};
use clap::Parser;
use std::{error::Error, path::PathBuf, str::FromStr};

pub mod cli;
pub mod client;
pub mod config;
pub mod firing;
pub mod process;
pub mod rebuild;
pub mod receive;
pub mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    println!("*****************************************************");
    println!("* LATTICE USHER DAEMON                              *");
    println!("*****************************************************");
    println!("Starting what is probably a terrible excuse for a server...");
    println!("Bro, welcome to the dungeon lol... ⚔️🐉");
    match cli.command {
        Commands::Listen {
            bind,
            port,
            rebuild,
            root,
            enclave,
            scopes,
            i_am,
            transform_registry,
            transform_store,
            usher_map,
            bootstrap,
            config,
            verbose,
        } => {
            // Load the config from the file specified file
            let usherd_config = if config.is_some() {
                UsherdConfig::from_file(&PathBuf::from(config.unwrap()))?
            } else {
                UsherdConfig::from_file(&PathBuf::from("./config.json"))?
            };

            // Overwrite config file values with the command line values.
            // Command line always takes precidence over the config file.
            let config = UsherdConfig {
                root_path: root.or(Some(usherd_config.root_path)).unwrap(),
                enclave: enclave.or(Some(usherd_config.enclave)).unwrap(),
                scopes: scopes.or(Some(usherd_config.scopes)).unwrap(),
                i_am: i_am.or(Some(usherd_config.i_am)).unwrap(),
                transform_registry: transform_registry
                    .or(Some(usherd_config.transform_registry))
                    .unwrap(),
                transform_store: transform_store
                    .or(Some(usherd_config.transform_store))
                    .unwrap(),
                usher_map: usher_map.or(Some(usherd_config.usher_map)).unwrap(),
                bind: bind.or(Some(usherd_config.bind)).unwrap(),
                port: port.or(Some(usherd_config.port)).unwrap(),
                rebuild: rebuild.is_some(),
                bootstrap: bootstrap.or(Some(usherd_config.bootstrap)).unwrap(),
                verbose,
            };

            // Run the actual server
            server::run(config).await?;
        }
        Commands::Send {
            usher,
            rhex_file,
            usher_map,
        } => {
            let usher_key = engine::general_purpose::URL_SAFE_NO_PAD.decode(usher)?;
            let usher_map = match usher_map {
                Some(usher_map) => usher_map,
                None => PathBuf::from_str("./ushers.cbor").unwrap(),
            };
            client::run(usher_key.try_into().unwrap(), rhex_file, usher_map).await?;
        }
    }
    Ok(())
}
