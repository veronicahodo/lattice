use std::collections::HashMap;

use anyhow::{Result, bail};
use rhex::{Rhex, data::RhexData, rt_structs::scope::RtScopeCreate};

use crate::{
    Scope,
    ushers::{UsherRole, UsherStatus},
};

pub fn spawn_scope(rhex: &Rhex) -> Result<Scope> {
    let scope_create = match RhexData::from_slice(&rhex.data).unwrap() {
        RhexData::Binary(bin) => RtScopeCreate::from_slice(&bin)?,
        _ => bail!("Invalid data type on spawn_scope"),
    };

    let mut scope = Scope::new(&scope_create.scope, &scope_create.creator)?;
    let mut actors = HashMap::new();
    let time = rhex.context.clone().unwrap().at;

    for usher in scope_create.ushers {
        actors.insert(
            usher,
            vec![UsherStatus {
                issued: time.clone(),
                eff: time.clone(),
                by: rhex.intent.author.clone(),
                exp: 1_000_000_000_000_000,
            }],
        );
    }
    scope.ushers.insert(UsherRole::Actor, actors);
    Ok(scope)
}
