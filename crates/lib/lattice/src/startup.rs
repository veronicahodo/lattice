use anyhow::{Result, bail};
use scope::{Scope, rhex::Rhex};
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
            let rchain: Vec<Rhex> = minicbor::decode(&rchain_bin)?;
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
        Ok(())
    }
}
