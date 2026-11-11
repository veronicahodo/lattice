use anyhow::{Result, bail};
use iroh::PublicKey;
use rhex::{
    Rhex,
    data::RhexData,
    rt_structs::usher::{RtUsherAssign, RtUsherDismiss, UsherRole},
};
use serde_json::Value;

pub fn verify_usher_assign(rhex: &Rhex) -> Result<RtUsherAssign> {
    let usher_assign = match RhexData::from_slice(&rhex.data) {
        Err(_) => bail!("verify `usher:assign` could not CBOR decode data"),
        Ok(data) => match data {
            RhexData::Binary(bin) => RtUsherAssign::from_slice(&bin)?,
            RhexData::Mixed { meta, binary } => process_json_usher_assign(&meta, &binary)?,
            _ => bail!("verify `usher:assign` could not CBOR decode RtUsherAssign from ::Binary"),
        },
    };

    Ok(usher_assign)
}

pub fn verify_usher_dismiss(rhex: &Rhex) -> Result<RtUsherDismiss> {
    let usher_dismiss = match RhexData::from_slice(&rhex.data) {
        Err(_) => bail!("verify `usher:dismiss` could not CBOR decode data"),
        Ok(data) => match data {
            RhexData::Binary(slice) => RtUsherDismiss::from_slice(&slice)?,
            RhexData::Mixed { meta, binary } => process_mixed_usher_dismiss(&meta, &binary)?,
            _ => bail!("verify `usher:dismiss` is not ::Binary or ::Mixed"),
        },
    };

    Ok(usher_dismiss)
}

fn process_json_usher_assign(json: &Vec<u8>, binary: &Vec<u8>) -> Result<RtUsherAssign> {
    let keys: Vec<PublicKey> = cbor2::from_slice(binary)?;
    let json = serde_json::from_slice::<Value>(json)?;
    let mut roles = Vec::new();

    match json.get("roles") {
        None => bail!("`usher:assign` doesn't specify roles in JSON"),
        Some(data) => match data.as_array() {
            None => bail!("`usher:assign` couldn't convert roles from JSON"),
            Some(data) => {
                for item in data {
                    let role = UsherRole::from_string(&item.as_str().unwrap().to_string())?;
                    roles.push(role)
                }
            }
        },
    };

    let eff = match json.get("eff") {
        None => 0 as u64,
        Some(eff) => eff.as_u64().unwrap(),
    };
    let exp = match json.get("exp") {
        None => 0 as u64,
        Some(exp) => exp.as_u64().unwrap(),
    };

    Ok(RtUsherAssign {
        roles,
        keys,
        eff,
        exp,
    })
}

fn process_mixed_usher_dismiss(meta: &Vec<u8>, binary: &Vec<u8>) -> Result<RtUsherDismiss> {
    let json = serde_json::from_slice::<Value>(meta)?;
    let keys: Vec<PublicKey> = cbor2::from_slice(binary)?;
    let mut roles_json = Vec::new();

    match json.get("roles") {
        None => bail!("could not get `usher:dismiss` roles from ::Mixed"),
        Some(roles) => match roles.as_array() {
            None => bail!("roles defined, but not array for verify `usher:dismiss`"),
            Some(roles) => {
                for role in roles {
                    roles_json.push(UsherRole::from_string(&role.as_str().unwrap().to_string())?);
                }
            }
        },
    };

    Ok(RtUsherDismiss {
        roles: roles_json,
        keys,
    })
}
