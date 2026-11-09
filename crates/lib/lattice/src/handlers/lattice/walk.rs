use anyhow::{Result, bail};
use iroh::PublicKey;
use scope::{
    Scope,
    membership::Membership,
    policy::Policy,
    rhex::{Rhex, data::RhexData},
    rule::Rule,
};

use crate::{Lattice, handlers::lattice::verify};

impl Lattice {
    pub fn walk_lattice_genesis(&mut self, rhex: &Rhex) -> Result<PublicKey> {
        let policy = Policy {
            desc: "Initial Genesis Policy".to_string(),
            rules: vec![Rule {
                append: vec!["world_line_zero".to_string()],
                k: 1,
                o: 1,
                quorum: vec!["world_line_zero".to_string()],
                delay: 1_000,
                rt: vec!["policy:set".to_string()],
                window: 10_000,
            }],
            eff: 0,
            exp: 1_000_000_000_000_000,
            tags: vec!["lattice".to_string(), "bootstrap".to_string()],
            issued: 0,
        };
        self.reset();
        let genesis = match RhexData::from_slice(&rhex.data) {
            Err(e) => bail!("Genesis RhexData returned {:?}", e),
            Ok(data) => verify::verify_lattice_genesis(&data)?,
        };
        let scope_name = "".to_string();
        let genesis_pubkey = PublicKey::from_bytes(&genesis.genesis_key)?;
        let mut new_scope = Scope::new(&scope_name, &genesis_pubkey)?;
        new_scope.add_membership(
            "world_line_zero".to_string(),
            vec![genesis_pubkey.clone()],
            Membership {
                issued: 0,
                eff: 0,
                exp: 1_000_000_000_000_000,
                by: genesis_pubkey.clone(),
            },
        )?;

        let _ = time::set_global_genesis(genesis.at);
        new_scope.policy_map.push((0, policy));
        self.add_scope(&new_scope);

        Ok(genesis_pubkey)
    }
}
