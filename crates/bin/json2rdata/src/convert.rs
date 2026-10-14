use std::{fs, path::PathBuf};

use anyhow::Result;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use lattice::rhex;

pub fn run(input: String, output: String) -> Result<()> {
    println!("Converting {} to {}", input, output);
    let json_str = fs::read_to_string(input).unwrap();
    let json: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    println!("data_type: {}", json["data_type"]);
    let data_type = json["data_type"].as_str().unwrap();
    let rhex_data = match data_type {
        "none" => rhex::data::RhexData::None,
        "json" => {
            let data = json["data"].clone().to_string();
            rhex::data::RhexData::Json(data.into_bytes())
        }
        "binary" => {
            let data = resolve_bytes(json["data"].as_str().unwrap()).unwrap();
            rhex::data::RhexData::Binary(data.clone())
        }
        "mixed" => {
            let meta = json["meta"].clone().to_string();
            let binary = json["binary"]
                .as_array()
                .unwrap()
                .iter()
                .map(|b| resolve_bytes(b.as_str().unwrap()).unwrap());
            let mut bin_chain: Vec<Vec<u8>> = Vec::new();
            for b in binary {
                bin_chain.push(b.clone());
            }
            rhex::data::RhexData::Mixed {
                meta: meta.into_bytes(),
                binary: minicbor::to_vec(&bin_chain).unwrap(),
            }
        }
        _ => panic!("Unknown data type: {}", data_type),
    };

    println!("rhex_data: {:?}", rhex_data);
    let buf = cbor2::to_canonical_vec(&rhex_data)?;
    fs::write(output, buf).unwrap();
    println!("Done!");
    Ok(())
}

fn resolve_bytes(input: &str) -> Result<Vec<u8>> {
    let path = PathBuf::from(input);
    if path.exists() && path.is_file() {
        let bytes = std::fs::read(path)?;
        return Ok(bytes);
    };

    let clean_input: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    Ok(URL_SAFE_NO_PAD.decode(clean_input.as_bytes())?)
}
