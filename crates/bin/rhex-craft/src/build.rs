use std::fs;

use anyhow::{Ok, Result};
use base64::Engine as _;
use iroh::PublicKey;
use scope::rhex::{Rhex, data::RhexData};

pub fn build(
    prev: Option<String>,
    scope: String,
    author: String,
    usher: String,
    schema: Option<String>,
    rt: String,
    data: Option<String>,
    output: String,
) -> Result<()> {
    // First, make sure we can get the data payload from the string
    let data_slice = match &data {
        Some(d) => fs::read(d)?,
        None => cbor2::to_canonical_vec(&RhexData::None)?,
    };
    let data_hash = if data_slice.len() > 0 && data != None {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&data_slice);
        Some(hasher.finalize().as_bytes().clone())
    } else {
        None
    };
    let mut rhex = Rhex::new();
    let prev = match prev {
        // Take the base64 of the previous hash and decode it
        Some(p) => Some(
            base64::engine::general_purpose::URL_SAFE_NO_PAD
                .decode(p)
                .unwrap()
                .try_into()
                .unwrap(),
        ),
        None => None,
    };
    // Same for author's and usher's public keys
    let author = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(author)
        .unwrap();
    let author = PublicKey::from_bytes(author[..].try_into()?)?;
    let usher = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(usher)
        .unwrap();
    let usher = PublicKey::from_bytes(usher[..].try_into()?)?;
    rhex.intent.prev = prev;
    rhex.intent.scope = scope;
    rhex.intent.author = author;
    rhex.intent.usher = usher;
    rhex.intent.schema = schema;
    rhex.intent.rt = rt;
    rhex.intent.data_hash = data_hash;
    rhex.data = data_slice;
    println!("R⬢ crafted.");
    rhex.single_disk_put(&output);
    println!("Wrote to {}.", output);
    Ok(())
}
