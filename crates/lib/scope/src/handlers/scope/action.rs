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
    if payload.token.len() == 0 {
        // Like really, I don't have anything to do to validate?
        // Once we get some sort of finance or ratio thing maybe?
        // I guess I'll mark it with a TODO:
        // bail!("Invalid token for `scope:request`")
    }
    let scope_create = RtScopeCreate {
        scope: payload.scope,
        creator: rhex.intent.author.clone(),
        ushers: payload.ushers,
    };

    let data = RhexData::Binary(scope_create.to_vec()?);

    let mut intent = RhexIntent::new();
    intent.prev = rhex.curr.clone();
    intent.scope = rhex.intent.scope.clone();
    intent.author = rhex.intent.usher.clone();
    intent.usher = rhex.intent.usher.clone();
    intent.schema = Some("rhex://schema.scope.create".to_string());
    intent.data_hash = Some(data.get_data_hash());
    Ok(Rhex::build_with(
        &intent,
        &None,
        &vec![],
        &Some(data.to_vec()?),
    ))
}
