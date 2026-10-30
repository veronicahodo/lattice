use anyhow::{Result, bail};
use iroh::PublicKey;
use lattice::{Lattice, Rhex, rhex::data::RhexData, scope::membership::Membership};
use serde::{Deserialize, Serialize};

pub fn execute_key_grant(rhex: &Rhex, lattice: &mut Lattice) -> Result<()> {
    let scope = match lattice.scopes.get_mut(&rhex.intent.scope) {
        None => bail!("Scope not available in execute key:grant"),
        Some(scope) => scope,
    };
    let data: RhexData = cbor2::from_slice(&rhex.data)?;
    let key_grant: KeyGrantData = match data {
        RhexData::Binary(bin) => cbor2::from_slice(&bin)?,
        _ => bail!("RhexData incorrect type for execute key:grant"),
    };
    for group in &key_grant.groups {
        scope.add_membership(
            group.clone(),
            key_grant.keys.clone(),
            key_grant.membership.clone(),
        )?;
    }
    Ok(())
}

pub fn execute_key_revoke(rhex: &Rhex, lattice: &mut Lattice) -> Result<()> {
    let scope = match lattice.scopes.get_mut(&rhex.intent.scope) {
        None => bail!("Scope not available in execute key:revoke"),
        Some(scope) => scope,
    };
    let data: RhexData = cbor2::from_slice(&rhex.data)?;
    let key_revoke: KeyRevokeData = match data {
        RhexData::Binary(bin) => cbor2::from_slice(&bin)?,
        _ => bail!("RhexData incorrect type for execute key:revoke"),
    };
    scope.remove_membership(key_revoke.keys, key_revoke.groups);
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyGrantData {
    keys: Vec<PublicKey>,
    groups: Vec<String>,
    membership: Membership,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRevokeData {
    keys: Vec<PublicKey>,
    groups: Vec<String>,
}
