use anyhow::Result;
use iroh_blobs::store::fs::FsStore;
use lattice::rhex::iroh::RhexEnvelope;

pub async fn append_store(envelope: &RhexEnvelope) -> Result<()> {
    let store = FsStore::load("./store/").await?;
    let bin = envelope.to_vec();
    let outcome = store.blobs().add_bytes(bin).await?;
    let record_hash = outcome.hash;
    let tag = outcome.name;
    println!("Success!");
    println!("-> Canonical BLAKE3 Hash:> {}", record_hash);
    println!("-> Local Storage Tag: {:?}", tag);
    Ok(())
}
