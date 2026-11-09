use anyhow::Result;
use scope::{
    Scope,
    rhex::{Rhex, data::RhexData},
};

use crate::{Lattice, handlers::scope::verify};

/// # walk_scope_create(rhex_data, lattice)
///
/// The walk function for `scope:create`. This requires a mutable
/// lattice so we can staple the new scope in the HashMap.
///
pub fn walk_scope_create(rhex_data: &RhexData, lattice: &mut Lattice) -> Result<()> {
    let scope_create = verify::verify_scope_create(rhex_data)?;

    let new_scope = Scope::new(&scope_create.scope, &scope_create.creator)?;
    lattice.add_scope(&new_scope);

    Ok(())
}

pub fn walk_scope_request(rhex: &Rhex) -> Result<()> {
    // Stub?  I don't think this actually needs to do anything.
    // Verifying basically checks the only issues, which is
    // a) formatting and b) the requested scope is a child of this one.
    verify::verify_scope_request(rhex)?;

    Ok(())
}
