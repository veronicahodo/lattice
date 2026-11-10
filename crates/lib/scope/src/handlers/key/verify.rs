use anyhow::{Result, bail};
use rhex::{
    Rhex,
    data::RhexData,
    rt_structs::key::{RtKeyGrant, RtKeyRevoke},
};

use crate::Scope;

pub fn verify_key_grant(rhex: &Rhex, scope: &Scope) -> Result<RtKeyGrant> {
    let data = match RhexData::from_slice(&rhex.data) {
        Err(_) => bail!("verify `key:grant` failed CBOR on decode"),
        Ok(data) => data,
    };
    let key_grant = match data {
        RhexData::Binary(bin) => RtKeyGrant::from_slice(&bin)?,
        _ => bail!("Wrong data type on verify `key:grant`"),
    };

    // Is all this really necessary?
    let time = match &rhex.context {
        Some(context) => context.at,
        None => time::get_current_time(),
    };
    let policy = scope.get_policy_at(&time);
    let membership = scope.member_of_at(&rhex.intent.author, &time)?;
    if policy.can_submit("key:grant", &membership) {
        Ok(key_grant)
    } else {
        bail!("verify `key:grant` cannot submit this data successfully")
    }
}

pub fn verify_key_revoke(rhex: &Rhex) -> Result<RtKeyRevoke> {
    let key_revoke = match RhexData::from_slice(&rhex.data) {
        Err(_) => bail!("verify `key:revoke` failed CBOR on decode"),
        Ok(data) => match data {
            RhexData::Binary(bin) => RtKeyRevoke::from_slice(&bin)?,
            _ => bail!("verify `key:revoke` failed CBOR decode on RtKeyRevoke"),
        },
    };
    Ok(key_revoke)
}
