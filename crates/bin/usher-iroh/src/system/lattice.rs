use anyhow::{Result, bail};
use iroh::PublicKey;
use lattice::{
    Lattice, Rhex,
    rhex::data::RhexData,
    scope::{membership::Membership, policy::Policy, rule::Rule},
};
use serde_json::Value;

pub fn execute_lattice_genesis(rhex: &Rhex, lattice: &mut Lattice) -> Result<()> {
    let data: RhexData = cbor2::from_slice(&rhex.data)?;
    match data {
        RhexData::Mixed { meta, binary } => {
            let json: Value = serde_json::from_slice(&meta)?;
            let at = json.get("at");
            let at = if at.is_none() {
                bail!("Time not set in genesis");
            } else {
                at.unwrap().as_i64().unwrap()
            };
            let genesis_key: [u8; 32] = binary.try_into().unwrap();
            let genesis_key = PublicKey::from_bytes(&genesis_key)?;
            let scope = lattice.scopes.get_mut(&"".to_string()).unwrap();
            scope.add_membership(
                "world_line_zero".to_string(),
                vec![genesis_key.clone()],
                Membership {
                    issued: 0,
                    eff: 0,
                    exp: 1_000_000_000_000_000,
                    by: genesis_key.clone(),
                },
            )?;
            let _ = time::set_global_genesis(at);
            let policy = Policy {
                desc: "".to_string(),
                rules: vec![Rule {
                    append: vec!["world_line_zero".to_string()],
                    k: 1,
                    o: 1,
                    quorum: vec!["world_line_zero".to_string()],
                    delay: 1_000,
                    rt: vec!["policy:set".to_string()],
                    window: 1_000,
                }],
                eff: 0,
                exp: 1_000_000_000_000_000,
                tags: vec!["bootstrap".to_string(), "lattice".to_string()],
                issued: 0,
            };
            scope.policy_map.push((0, policy));
        }
        _ => bail!("Lattice Genesis set to wrong data type"),
    }
    Ok(())
}
