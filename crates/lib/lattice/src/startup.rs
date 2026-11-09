use std::{collections::HashMap, fs};

use anyhow::{Result, bail};
use iroh::PublicKey;
use scope::{
    Scope,
    membership::Membership,
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

            let (dir, scope_path) = if entry.path().is_dir() || entry.path().ends_with(".rchain") {
                if entry.path().is_dir() {
                    let path = format!("{}/genesis.rhex", entry.path().display());
                    if fs::exists(&path)? {
                        (
                            true,
                            format!(
                                "{}/",
                                entry.path().as_os_str().to_str().unwrap().to_string()
                            ),
                        )
                    } else {
                        continue;
                    }
                } else {
                    let path = entry.path().as_os_str().to_str().unwrap().to_string();
                    (false, path)
                }
            } else {
                continue;
            };

            if verbose {
                print!("\t🌐 Loading scope: {}...", scope_path);
            } else {
                print!(".");
            }
            let rchain = if dir {
                Rhex::chain_from_disk(&scope_path)?
            } else {
                let rchain_bin = std::fs::read(scope_path)?;
                cbor2::from_slice(&rchain_bin)?
            };

            let scope = Scope::walk_rhex(
                &rchain[0].intent.scope.clone(),
                rchain[0].intent.author.clone(),
                &rchain,
                trans_reg,
                verbose,
            )?;

            self.scopes.insert(scope.name.clone(), scope.clone());
            verbose_println("done!".to_string(), verbose);
            if verbose {
                println!("{:#?}", scope);
            }
        }

        // Failsafe if there's no scopes at all
        let len = self.scopes.len();
        if len == 0 {
            let genesis_key = PublicKey::from_bytes(&Lattice::GENESIS_KEY)?;
            let mut new_root = Scope::new(&"".to_string(), &genesis_key)?;
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
