use anyhow::{Result, anyhow, bail};
use scope::{Scope, rhex::Rhex};
use transform::registry::TransformRegistry;

use crate::Lattice;

impl Lattice {
    pub fn startup(&mut self, path: &String) -> Result<()> {
        let scopes_dir_entries = std::fs::read_dir(path)?;
        for entry in scopes_dir_entries {
            let entry = entry?;
            // skip if its a dir
            if entry.path().is_dir() || !entry.path().ends_with(".rchain") {
                continue;
            }
            print!("\t🌐 Loading scope: {}...", entry.file_name().display());
            let scope_path = entry.path();
            let scope_path = scope_path
                .file_prefix()
                .ok_or(anyhow!("No prefix"))?
                .to_str()
                .ok_or(anyhow!("Failed to convert prefix"))?;
            let mut scope = Scope::new(&scope_path.to_string(), [0; 32]);
            let _scope_rhex = scope.slurp_scope(
                entry
                    .file_name()
                    .to_str()
                    .ok_or(anyhow!("Couldn't convert filename"))?
                    .to_string(),
            )?;
            self.add_scope(&scope);
            println!("done");
        }
        Ok(())
    }

    pub fn build_from_disk(
        &mut self,
        path: &String,
        trans_reg: &TransformRegistry,
        verbose: bool,
    ) -> Result<()> {
        if verbose {
            println!("Reading scopes in {}...", path);
        }
        let scope_dir_entries = std::fs::read_dir(path)?;
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
            let rchain: Vec<Rhex> = minicbor::decode(&rchain_bin)?;
            let scope = Scope::walk_rhex(
                &rchain[0].intent.scope.clone(),
                rchain[0].intent.author.clone(),
                &rchain,
                trans_reg,
                verbose,
            )?;
            self.scopes.insert(scope.name.clone(), scope);
            if verbose {
                println!("done!");
            }
        }
        Ok(())
    }
}
