use std::fs;

use anyhow::{Result, bail};
use iroh_blobs::store::fs::FsStore;
use lattice::rhex::iroh::IrohRhex;
use tokio::io::AsyncReadExt;

pub async fn retrieve_store(hash: &[u8; 32]) -> Result<IrohRhex> {
    let store = FsStore::load("./store/").await?;
    let mut reader = store.reader(hash);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).await?;
    let rhex: IrohRhex = cbor2::from_slice(&buf)?;
    Ok(rhex)
}

pub async fn retrieve_disk(
    scope: &String,
    hash: &[u8; 32],
    scope_path: &String,
) -> Result<IrohRhex> {
    let scope = match scope.as_str() {
        "" => &"-root-".to_string(),
        _ => scope,
    };
    let path = format!("{}{}/{}.rhex", scope_path, scope, hex::encode(&hash));
    if !fs::exists(&path)? {
        bail!(format!("Tried to load {} and failed", &path));
    };
    let bin = fs::read(path)?;
    let iroh_rhex: IrohRhex = cbor2::from_slice(&bin)?;
    Ok(iroh_rhex)
}
