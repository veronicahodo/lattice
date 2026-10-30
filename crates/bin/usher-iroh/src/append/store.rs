use std::fs::{self};

use anyhow::Result;
use iroh_blobs::store::fs::FsStore;
use lattice::{Rhex, rhex::iroh::IrohRhex};

pub async fn append_store(rhex_iroh: &IrohRhex) -> Result<[u8; 32]> {
    let store = FsStore::load("./store/").await?;
    let bin = rhex_iroh.to_vec()?;
    let outcome = store.blobs().add_bytes(bin).await?;
    let record_hash = outcome.hash;
    let tag = outcome.name;
    println!("Success!");
    println!("-> Canonical BLAKE3 Hash:> {:?}", hex::decode(record_hash));
    println!("-> Local Storage Tag: {:?}", tag);
    Ok(record_hash.as_bytes().clone())
}

pub async fn append_disk(rhex: &Rhex, scope_path: &String) -> Result<()> {
    let scope_name = match rhex.intent.scope.as_str() {
        "" => &"-root-".to_string(),
        _ => &rhex.intent.scope,
    };
    let filename = match rhex.intent.prev {
        None => "genesis.rhex".to_string(),
        Some(hash) => hex::encode(hash),
    };
    let path = format!("{}{}/{}.rhex", scope_path, scope_name, filename);
    if !fs::exists(&path)? {
        fs::create_dir(format!("{}{}", scope_path, scope_name))?;
    }
    let bin = rhex.to_iroh()?.to_vec()?;
    fs::write(path, bin)?;
    Ok(())
}
