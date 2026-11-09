use anyhow::{Result, bail};
use scope::rhex::{
    Rhex,
    data::RhexData,
    rt_structs::scope::{RtScopeCreate, RtScopeRequest},
};

/// # verify_scope_create(rhex_data)
///
/// Verifies and returns the `scope:create` from `::Binary`
///
pub fn verify_scope_create(rhex_data: &RhexData) -> Result<RtScopeCreate> {
    let scope_create = match rhex_data {
        RhexData::Binary(bin) => RtScopeCreate::from_slice(&bin)?,
        _ => bail!("Invalid data type in verify `scope:create`"),
    };

    if scope_create.scope.as_str() == "" {
        bail!("Invalid scope name. Only lattice:genesis can be root.");
    }

    Ok(scope_create)
}

/// # verify_scope_request(rhex)
///
/// Verifies and returns the `scope:request` from either `::Binary`
/// or `::Json`
///
pub fn verify_scope_request(rhex: &Rhex) -> Result<RtScopeRequest> {
    let rhex_data = RhexData::from_slice(&rhex.data);
    let scope_request = match rhex_data {
        Err(_) => bail!("verify `scope:request` failed CBOR to RhexData"),
        Ok(data) => match data {
            RhexData::Binary(bin) => RtScopeRequest::from_slice(&bin)?,
            RhexData::Json(json) => serde_json::from_slice(&json)?,
            _ => bail!("Wrong data type for verify `scope:request`"),
        },
    };

    if scope_request.scope.as_str() == "" {
        bail!("Can't `scope:request` for the root scope. Duh.")
    }

    if !scope_request
        .scope
        .starts_with(&format!("{}.", rhex.intent.scope))
    {
        bail!("Requested scope is not a child of the parent")
    }

    Ok(scope_request)
}
