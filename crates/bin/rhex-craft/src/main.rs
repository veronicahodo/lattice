use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use clap::Parser;

use cli::Commands;

mod build;
mod cli;
mod finalize;
mod genesis;
mod submit;
mod view;

#[tokio::main]
async fn main() {
    println!("Lattice R⬢ Crafting Tool {}", env!("CARGO_PKG_VERSION"));
    let cli = cli::Cli::parse();
    match cli.command {
        Commands::Build {
            prev,
            scope,
            author,
            usher,
            schema,
            rt,
            data,
            output,
        } => {
            let status = build::build(prev, scope, author, usher, schema, rt, data, output);
            if let Err(e) = status {
                eprintln!("{e}");
            }
        }
        Commands::Finalize {
            input,
            output,
            use_curr,
        } => {
            let _ = finalize::finalize(input, output, use_curr);
        }
        Commands::Genesis { config } => {
            let config = match config {
                None => "./config.json".to_string(),
                Some(cfg) => cfg,
            };
            let status = genesis::do_genesis(config);
            match status {
                Ok(prev) => println!("Last Hash: {}", URL_SAFE_NO_PAD.encode(prev)),
                Err(e) => eprintln!("{e}"),
            }
        }
        Commands::View { input } => {
            let _ = view::view(input);
        }
        Commands::Submit {
            rhex,
            address,
            request,
        } => {
            let status = submit::submit(rhex, address, request).await;
            if let Err(s) = status {
                println!("{:?}", s);
            }
        }
    }
}
