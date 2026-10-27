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
        Commands::Genesis {
            key,
            output,
            enclave_path,
        } => {
            let _ = genesis::genesis(key, enclave_path, output);
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
