use anyhow::{Result, bail};
use rhex::{Rhex, check::CheckStatus, data::RhexData, rt_structs::key::RtKeyGrant};

use crate::Scope;

pub fn verify_key_grant(rhex: &Rhex, scope: &Scope) -> Result<CheckStatus> {
    let data = match RhexData::from_slice(&rhex.data) {
        Err(_) => return Ok(CheckStatus::DataFailedCborDecode),
        Ok(data) => data,
    };
    let _ = match data {
        RhexData::Binary(bin) => RtKeyGrant::from_slice(&bin)?,
        _ => bail!("Wrong data type on verify `key:grant`"),
    };

    let time = match &rhex.context {
        Some(context) => context.at,
        None => time::get_current_time(),
    };
    let policy = scope.get_policy_at(&time);

    Ok(CheckStatus::Success)
}
