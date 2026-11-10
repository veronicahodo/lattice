use anyhow::Result;
use rhex::Rhex;

use crate::{Scope, handlers, membership::Membership};

pub fn walk_key_grant(rhex: &Rhex, scope: &mut Scope) -> Result<()> {
    let key_grant = handlers::key::verify::verify_key_grant(rhex, &scope)?;
    for group in &key_grant.groups {
        let time = if rhex.context.is_some() {
            rhex.context.clone().unwrap().at
        } else {
            time::get_current_time()
        };
        let membership = Membership {
            issued: time.clone(),
            eff: key_grant.eff.clone(),
            exp: key_grant.exp.clone(),
            by: rhex.intent.author.clone(),
        };
        scope.add_membership(group.clone(), key_grant.keys.clone(), membership)?;
    }
    Ok(())
}

pub fn walk_key_revoke(rhex: &Rhex, scope: &mut Scope) -> Result<()> {
    let key_revoke = handlers::key::verify::verify_key_revoke(rhex)?;
    scope.remove_membership(&key_revoke.keys, &key_revoke.groups, &rhex.intent.author)?;
    Ok(())
}
