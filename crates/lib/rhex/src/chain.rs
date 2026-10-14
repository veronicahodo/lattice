use std::{fs, path::PathBuf};

use anyhow::Result;

use crate::Rhex;

impl Rhex {
    pub fn chain_from_disk(path: &PathBuf) -> Result<Vec<Rhex>> {
        let file = fs::read(path)?;
        let rhex: Vec<Rhex> = cbor2::from_slice(&file)?;
        Ok(rhex)
    }

    pub fn chain_to_disk(path: PathBuf, rhex: Vec<Rhex>) -> Result<()> {
        let bin = cbor2::to_canonical_vec(&rhex)?;
        fs::write(path, bin)?;
        Ok(())
    }
}
