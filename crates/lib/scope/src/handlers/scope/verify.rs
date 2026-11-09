use anyhow::{Result, bail};
use rhex::{Rhex, check::CheckStatus, data::RhexData, rt_structs::scope::RtScopeGenesis};

pub fn verify_scope_genesis(rhex: &Rhex) -> Result<CheckStatus> {
    let data = match RhexData::from_slice(&rhex.data) {
        Err(_) => return Ok(CheckStatus::DataFailedCborDecode),
        Ok(data) => data,
    };
    let _ = match data {
        RhexData::Binary(bin) => RtScopeGenesis::from_slice(&bin)?,
        _ => bail!("Wrong data type on verify `scope:genesis`"),
    };

    Ok(CheckStatus::Success)
}
