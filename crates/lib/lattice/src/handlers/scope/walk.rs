use anyhow::Result;
use scope::{
    Scope,
    rhex::{Rhex, data::RhexData},
    ushers::{UsherRole, UsherStatus},
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

pub fn walk_scope_update(rhex: &Rhex, scope_path: &String, lattice: &mut Lattice) -> Result<()> {
    let scope_update = verify::verify_scope_update(rhex, scope_path)?;

    let scope = lattice.scopes.get_mut(&scope_update.subscope);
    let scope = if scope.is_none() {
        // See... I don't know how to feel about this here.
        // This gets executed if we have a `scope:update` but not
        // come across a `scope:create` for that scope.
        let new_scope = Scope::new(&scope_update.subscope, &rhex.intent.author)?;
        lattice.add_scope(&new_scope);
        lattice.scopes.get_mut(&scope_update.subscope).unwrap()
    } else {
        scope.unwrap()
    };

    let time = rhex.context.clone().unwrap().at;
    let actors = scope.ushers.entry(UsherRole::Actor).or_default();

    for usher in scope_update.ushers {
        actors.insert(
            usher.clone(),
            vec![UsherStatus {
                issued: time.clone(),
                eff: time.clone(),
                exp: 1_000_000_000_000_000,
                by: rhex.intent.author.clone(),
            }],
        );
    }
    Ok(())
}
