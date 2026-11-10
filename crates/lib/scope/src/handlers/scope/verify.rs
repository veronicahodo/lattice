use anyhow::{Result, bail};
use rhex::{Rhex, data::RhexData, rt_structs::scope::RtScopeGenesis};

pub fn verify_scope_genesis(rhex: &Rhex) -> Result<RtScopeGenesis> {
    let data = match RhexData::from_slice(&rhex.data) {
        Err(_) => bail!("verify `scope:genesis` failed CBOR decode"),
        Ok(data) => data,
    };
    let scope_genesis = match data {
        RhexData::Binary(bin) => RtScopeGenesis::from_slice(&bin)?,
        _ => bail!("Wrong data type on verify `scope:genesis`"),
    };

    Ok(scope_genesis)
}
