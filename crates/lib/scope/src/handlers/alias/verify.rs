use anyhow::{Result, bail};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rhex::{Rhex, data::RhexData, rt_structs::alias::RtAliasSet};

pub fn verify_alias_set(rhex: &Rhex, scope_path: &String) -> Result<RtAliasSet> {
    let alias_set = match RhexData::from_slice(&rhex.data) {
        Err(_) => bail!("verify `alias:set` failed CBOR decode"),
        Ok(data) => match data {
            RhexData::Binary(bin) => RtAliasSet::from_slice(&bin)?,
            _ => bail!("verify `alias:set` not ::Binary"),
        },
    };
    let chain = Rhex::chain_from_disk(scope_path)?;
    for rhex in chain {
        if rhex.curr.unwrap() == alias_set.address {
            return Ok(alias_set);
        }
    }
    bail!(
        "Could not find target for verifying `alias:set`: {}",
        URL_SAFE_NO_PAD.encode(&alias_set.address)
    );
}
