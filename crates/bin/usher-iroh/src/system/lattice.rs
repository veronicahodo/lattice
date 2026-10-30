use anyhow::{Result, bail};
use iroh::PublicKey;
use lattice::{Lattice, Rhex, rhex::data::RhexData, scope::membership::Membership};
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
        }
        _ => bail!("Lattice Genesis set to wrong data type"),
    }
    Ok(())
}
