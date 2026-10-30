use anyhow::Result;
use iroh_blobs::store::fs::FsStore;
use lattice::rhex::iroh::IrohRhex;

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
