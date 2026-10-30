use std::collections::HashMap;

use anyhow::{Result, bail};
use iroh::PublicKey;
use scope::{
    Scope,
    membership::Membership,
    policy::Policy,
    rhex::Rhex,
    ushers::{UsherRole, UsherStatus},
};
use transform::registry::TransformRegistry;
use util::verbose_println;

use crate::Lattice;

impl Lattice {
    pub fn build_from_disk(
        &mut self,
        path: &String,
        trans_reg: &TransformRegistry,
        verbose: bool,
    ) -> Result<()> {
        print!("Loading scopes from {}", path);
        let scope_dir_entries = std::fs::read_dir(path)?;
        verbose_println("".to_string(), verbose);
        for entry in scope_dir_entries {
            // Handle errs first
            if entry.is_err() {
                bail!(format!("DirEntry error {}", entry.err().unwrap()));
            }

            let entry = entry?;

            // skip if a dir
            if entry.path().is_dir() || !entry.path().ends_with(".rchain") {
                continue;
            }

            // Load rchain for the scope
            if verbose {
                print!("\t🌐 Loading scope: {}...", entry.file_name().display());
            } else {
                print!(".");
            }
            let scope_path = entry.path();
            let rchain_bin = std::fs::read(scope_path)?;
            let rchain: Vec<Rhex> = cbor2::from_slice(&rchain_bin)?;
            let scope = Scope::walk_rhex(
                &rchain[0].intent.scope.clone(),
                rchain[0].intent.author.clone(),
                &rchain,
                trans_reg,
                verbose,
            )?;
            self.scopes.insert(scope.name.clone(), scope);
            verbose_println("done!".to_string(), verbose);
        }

        // Failsafe if there's no scopes at all
        let len = self.scopes.len();
        if len == 0 {
            let genesis_key = PublicKey::from_bytes(&Lattice::GENESIS_KEY)?;
            let mut new_root = Scope::new(&"".to_string(), genesis_key.clone())?;
            /*new_root
            .policy_map
            .push((0, Policy::default_lattice_policy()));*/
            new_root.add_membership(
                "world_line_zero".to_string(),
                vec![genesis_key],
                Membership {
                    issued: 0,
                    eff: 0,
                    exp: 1_000_000_000_000_000,
                    by: genesis_key.clone(),
                },
            )?;
            let usher_status = UsherStatus {
                issued: 0,
                eff: 0,
                exp: 1_000_000_000_000_000,
                by: genesis_key.clone(),
            };
            let mut map = HashMap::new();
            map.insert(genesis_key.clone(), vec![usher_status]);
            new_root.ushers.insert(UsherRole::Actor, map.clone());
            new_root.head = None;
            new_root.updated = 0;
            self.add_scope(&new_root);
        }
        Ok(())
    }
}
