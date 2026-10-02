use crate::{
    cli::{Cli, Commands},
    config::UsherdConfig,
};
use base64::{Engine as _, engine};
use clap::Parser;
use std::{error::Error, path::PathBuf, str::FromStr};
use util::{verbose_print, verbose_println};

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
            observer,
            verbose,
        } => {
            // Load the config from the file specified file
            let config = if config.is_some() {
                config.unwrap()
            } else {
                "./config.json".to_string()
            };
            verbose_print(format!("Loading config file {}... ", config), verbose);
            let usherd_config = UsherdConfig::from_file(&PathBuf::from(config))?;

            // Overwrite config file values with the command line values.
            // Command line always takes precidence over the config file.
            let config = usherd_config.merge_config(
                root,
                enclave,
                scopes,
                i_am,
                transform_registry,
                transform_store,
                usher_map,
                bind,
                port,
                rebuild,
                bootstrap,
                observer,
                verbose,
            )?;
            verbose_println("done!".to_string(), verbose);

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
