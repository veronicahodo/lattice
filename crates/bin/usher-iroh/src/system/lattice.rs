use anyhow::Result;
use lattice::{Lattice, Rhex, scope::Scope};

pub fn execute_lattice_genesis(rhex: &Rhex, lattice: &mut Lattice) -> Result<()> {
    let scope = lattice.scopes.get_mut(&"".to_string());
    let scope = match scope {
        None => &mut Scope::new(&"".to_string(), &rhex.intent.author)?,
        Some(scope) => scope,
    };
    scope.process_scope_changes(rhex)?;
    Ok(())
}
