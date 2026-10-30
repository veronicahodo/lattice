use anyhow::{Result, bail};
use lattice::{Lattice, Rhex, rhex::data::RhexData, scope::policy::Policy};

pub fn execute_policy_set(rhex: &Rhex, lattice: &mut Lattice) -> Result<()> {
    let scope = match lattice.scopes.get_mut(&rhex.intent.scope) {
        None => bail!("Scope not available in execute policy:set"),
        Some(scope) => scope,
    };
    let data: RhexData = cbor2::from_slice(&rhex.data)?;
    let policy: Policy = match data {
        RhexData::Binary(bin) => cbor2::from_slice(&bin)?,
        _ => bail!("RhexData not binary in execute policy:set"),
    };
    scope
        .policy_map
        .push((rhex.context.clone().unwrap().at, policy.clone()));
    Ok(())
}
