use anyhow::Result;
use rhex::Rhex;

use crate::{Scope, handlers};

pub fn walk_alias_set(rhex: &Rhex, scope: &mut Scope, scopes_path: &String) -> Result<()> {
    let alias_set = handlers::alias::verify::verify_alias_set(
        rhex,
        &format!("{}{}/", scopes_path, scope.name),
    )?;
    scope.aliases.insert(alias_set.label, alias_set.address);
    Ok(())
}
