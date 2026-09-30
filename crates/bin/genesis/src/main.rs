use std::process::exit;

use clap::Parser;
use rhex::data::RhexData;
use serde_json::json;

use crate::cli::{Cli, Commands};

pub mod cli;

fn get_time() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn check_pos(lat: Option<f64>, long: Option<f64>) -> Option<(f64, f64)> {
    if lat.is_none() || long.is_none() {
        None
    } else {
        Some((lat.unwrap(), long.unwrap()))
    }
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Now { output, lat, long } => {
            let path = if output.is_none() {
                "./genesis-payload.rdata".to_string()
            } else {
                output.unwrap()
            };
            let pos = check_pos(lat, long);
            let at = get_time();
            let json = if pos.is_none() {
                json!({
                    "at": at,
                })
            } else {
                json!({
                    "at": at,
                    "lat": pos.unwrap().0,
                    "long": pos.unwrap().1,
                })
            };
            let data = RhexData::Json(serde_json::to_vec(&json).unwrap());

            print!("Writing genesis rdata to {}...", path);
            let status = std::fs::write(path, data.to_vec().unwrap());
            if status.is_err() {
                println!(" FAILED! {}", status.err().unwrap());
                exit(1);
            }
            println!(" done!");
        }
    }
}
