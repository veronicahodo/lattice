use std::collections::HashMap;

use anyhow::{Result, bail};
use iroh::PublicKey;
use lattice::{
    Lattice, Rhex,
    rhex::data::RhexData,
    scope::ushers::{UsherRole, UsherStatus},
};
use serde_json::Value::{self, Number};

pub fn execute_usher_assign(rhex: &Rhex, lattice: &mut Lattice) -> Result<()> {
    let scope = match lattice.scopes.get_mut(&rhex.intent.scope) {
        None => bail!("Scope not available in execute usher:assign"),
        Some(scope) => scope,
    };
    let data: RhexData = cbor2::from_slice(&rhex.data)?;
    let usher_assign = match data {
        RhexData::Mixed { meta, binary } => {
            let json: Value = serde_json::from_slice(&meta)?;
            let roles = json.get("roles");
            let roles = match roles {
                None => bail!("Roles not defined in execute usher:assign"),
                Some(roles) => {
                    let keys: Vec<PublicKey> = cbor2::from_slice(&binary)?;
                    if roles.as_array().is_some() {
                        let mut output = Vec::new();
                        let roles = roles.as_array().unwrap();
                        for role in roles {
                            let usher_role = match role.as_str().unwrap() {
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
                        let eff = json.get("eff").unwrap();
                        let eff = eff.as_u64().unwrap();
                        let exp = json.get("exp").unwrap();
                        let exp = exp.as_u64().unwrap();
                        for role in output {
                            for key in &keys {
                                // FIXME: This should look up to see if we
                                // already have this key, but like... you
                                // know what, I'm not even explaining because
                                // I'm tired. Always
                                let map = &mut HashMap::new();
                                let usher_pool = scope.ushers.get_mut(&role).unwrap_or_else(|| {
                                    map.insert(
                                        key.clone(),
                                        vec![UsherStatus {
                                            issued: rhex.context.clone().unwrap().at,
                                            eff,
                                            exp,
                                            by: rhex.intent.author.clone(),
                                        }],
                                    );
                                    map
                                });
                            }
                            let entry = scope.ushers.entry(role);
                        }
                    } else {
                        bail!("Roles not defined")
                    };
                }
            };
        }
        _ => bail!("RhexData not mixed for execute usher:assign"),
    };

    Ok(())
}
