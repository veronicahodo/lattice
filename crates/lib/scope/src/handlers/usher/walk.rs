use anyhow::Result;
use rhex::{Rhex, rt_structs::usher::UsherStatus};

use crate::{Scope, handlers};

pub fn walk_usher_assign(rhex: &Rhex, scope: &mut Scope) -> Result<()> {
    let usher_assign = handlers::usher::verify::verify_usher_assign(rhex)?;

    for role in &usher_assign.roles {
        let usher_role = scope.ushers.entry(role.clone()).or_default();

        let time = rhex.context.clone().unwrap().at.clone();
        let status = UsherStatus {
            issued: time,
            eff: usher_assign.eff.clone(),
            exp: usher_assign.exp.clone(),
            by: rhex.intent.author.clone(),
        };

        for key in &usher_assign.keys {
            let key_entry = usher_role.entry(key.clone()).or_default();
            key_entry.push(status.clone())
        }
    }

    Ok(())
}

pub fn walk_usher_dismiss(rhex: &Rhex, scope: &mut Scope) -> Result<()> {
    let usher_dismiss = handlers::usher::verify::verify_usher_dismiss(rhex)?;
    for role in usher_dismiss.roles {
        let role_map = match scope.ushers.get_mut(&role) {
            None => continue,
            Some(role) => role,
        };
        for key in &usher_dismiss.keys {
            if role_map.contains_key(key) {
                role_map.remove(key);
            }
        }
    }
    Ok(())
}
