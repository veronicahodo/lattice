use anyhow::{Result, bail};
use rhex::{Rhex, data::RhexData, intent::RhexIntent, rt_structs::scope::RtScopeCreate};

use crate::handlers;

pub fn action_scope_genesis(_rhex_data: &RhexData) -> Result<()> {
    // Stub. `scope:genesis` doesn't cause any action at this point

    Ok(())
}

pub fn action_scope_request(rhex: &Rhex) -> Result<Rhex> {
    // Like I feel like we should have checked for this by now, but just
    // in case so we don't accidentally approve scopes...
    let payload = handlers::scope::verify::verify_scope_request(rhex)?;
    if !payload
        .scope
        .starts_with(&format!("{}.", rhex.intent.scope))
    {
        bail!(
            "The `scope:request` is not for a sub scope of this one: {}",
            rhex.intent.scope
        )
    }
    let scope_create = RtScopeCreate {
        scope: payload.scope,
        creator: rhex.intent.author.clone(),
        ushers: payload.ushers,
    };

    let mut intent = RhexIntent::new();
    intent.scope = rhex.intent.scope.clone();
    intent.author = rhex.intent.usher.clone();
    intent.usher = rhex.intent.usher.clone();
    Ok(Rhex::build_with(
        &intent,
        &None,
        &vec![],
        &Some(scope_create.to_vec()?),
    ))
}
