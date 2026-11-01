use anyhow::{Result, bail};
use iroh::PublicKey;
use lattice::{
    Lattice, Rhex,
    rhex::data::RhexData,
    scope::ushers::{UsherRole, UsherStatus},
};
use serde_json::Value;

pub fn execute_usher_assign(rhex: &Rhex, lattice: &mut Lattice) -> Result<()> {
    let scope = match lattice.scopes.get_mut(&rhex.intent.scope) {
        None => bail!("Scope not available in execute usher:assign"),
        Some(scope) => scope,
    };

    let (meta, binary) = match cbor2::from_slice(&rhex.data)? {
        RhexData::Mixed { meta, binary } => (meta, binary),
        _ => bail!("RhexData not mixed for execute usher:assign"),
    };

    let json: Value = serde_json::from_slice(&meta)?;

    let roles_array = json
        .get("roles")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow::anyhow!("Roles not defined in execute usher:assign"))?;

    let keys: Vec<PublicKey> = cbor2::from_slice(&binary)?;

    let mut output = Vec::new();
    for role in roles_array {
        let role_str = role.as_str().unwrap_or_default();
        let usher_role = match role_str {
            "actor" => UsherRole::Actor,
            "quorum" => UsherRole::Quorum,
            "mirror" => UsherRole::Mirror,
            "cache" => UsherRole::Cache,
            "observer" => UsherRole::Observer,
            "" => UsherRole::Other,
            _ => bail!("Role assignment sucks on execute usher:assign"),
        };
        output.push(usher_role);
    }

    let eff = json.get("eff").and_then(|v| v.as_u64()).unwrap_or_default();
    let exp = json.get("exp").and_then(|v| v.as_u64()).unwrap_or_default();
    let issued = rhex.context.as_ref().map(|c| c.at).unwrap_or_default();
    let author = rhex.intent.author.clone();

    // Populate the hash map using .entry() to handle lookups & inserts cleanly
    for role in output {
        let inner_map = scope.ushers.entry(role).or_default();
        for key in &keys {
            inner_map.entry(key.clone()).or_default().push(UsherStatus {
                issued,
                eff,
                exp,
                by: author.clone(),
            });
        }
    }

    Ok(())
}

pub fn execute_usher_dismiss(rhex: &Rhex, lattice: &mut Lattice) -> Result<()> {
    let scope = match lattice.scopes.get_mut(&rhex.intent.scope) {
        None => bail!("Scope not available in execute usher:dismiss"),
        Some(scope) => scope,
    };

    let data: RhexData = cbor2::from_slice(&rhex.data)?;
    let keys: Vec<PublicKey> = match data {
        RhexData::Binary(bin) => cbor2::from_slice(&bin)?,
        _ => bail!("RhexData not binary in execute usher:dismiss"),
    };

    let issued = rhex.context.as_ref().map(|c| c.at).unwrap_or_default();
    let author = rhex.intent.author.clone();

    // Iterate through all role hash maps in the scope and append a zeroed status to revoke access
    for inner_map in scope.ushers.values_mut() {
        for key in &keys {
            if let Some(statuses) = inner_map.get_mut(key) {
                statuses.push(UsherStatus {
                    issued,
                    eff: 0,
                    exp: 0,
                    by: author.clone(),
                });
            }
        }
    }

    Ok(())
}
