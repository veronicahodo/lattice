use std::fs;

use anyhow::{Result, bail};

use crate::Rhex;

impl Rhex {
    pub fn chain_from_disk(path: &String) -> Result<Vec<Rhex>> {
        let mut output = Vec::new();
        let mut next = format!("{}genesis.rhex", path);
        if !fs::exists(&next)? {
            bail!("No such scope on disk");
        }
        while fs::exists(&next)? {
            let file = fs::read(&next)?;
            let rhex: Rhex = cbor2::from_slice(&file)?;
            let hash = hex::encode(rhex.curr.unwrap());
            output.push(rhex.clone());
            next = format!("{}{}.rhex", path, hash);
        }

        Ok(output)
    }

    pub fn chain_to_disk(path: &String, rhex: &Vec<Rhex>) -> Result<()> {
        if !fs::exists(path)? {
            fs::create_dir_all(path)?
        }
        for r in rhex {
            let filename = match r.intent.prev {
                None => format!("{}genesis.rhex", path),
                Some(prev) => format!("{}{}.rhex", path, hex::encode(prev)),
            };
            r.single_disk_put(&filename)?;
        }
        Ok(())
    }
}
