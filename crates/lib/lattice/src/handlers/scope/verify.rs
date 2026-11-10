use anyhow::{Result, bail};
use scope::rhex::{
    Rhex,
    data::RhexData,
    rt_structs::scope::{RtScopeCreate, RtScopeRequest, RtScopeUpdate},
};

use crate::handlers;

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

pub fn verify_scope_update(rhex: &Rhex, scope_path: &String) -> Result<RtScopeUpdate> {
    let udata = RhexData::from_slice(&rhex.data);
    let update = match udata {
        Err(_) => bail!("verify `scope:update` failed CBOR decode"),
        Ok(data) => match data {
            RhexData::Binary(bin) => RtScopeUpdate::from_slice(&bin)?,
            _ => bail!("verify `scope:update` couldn't CBOR decode from ::Binary"),
        },
    };
    let chain = Rhex::chain_from_disk(scope_path)?;
    for c in chain {
        if c.intent.rt.as_str() == "scope:create" {
            let data = RhexData::from_slice(&c.data);
            let data = match data {
                Err(_) => bail!("`scope:create` data failed to decode in a `scope:update`"),
                Ok(data) => data,
            };
            let payload = handlers::scope::verify::verify_scope_create(&data)?;
            if payload.scope == update.subscope {
                return Ok(update);
            }
        }
    }
    bail!("Didn't find the matching `scope:create` on verify `scope:update`")
}
