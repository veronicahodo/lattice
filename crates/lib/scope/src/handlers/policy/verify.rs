use anyhow::{Result, bail};
use rhex::{Rhex, data::RhexData, rt_structs::policy::RtPolicySet};

pub fn verify_policy_set(rhex: &Rhex) -> Result<RtPolicySet> {
    let policy_set = match RhexData::from_slice(&rhex.data) {
        Err(_) => bail!("verify `policy:set` data failed CBOR decode"),
        Ok(data) => match data {
            RhexData::Binary(bin) => RtPolicySet::from_slice(&bin)?,
            _ => bail!("verify `policy:set` RtPolicySet CBOR decode failed"),
        },
    };

    Ok(policy_set)
}
