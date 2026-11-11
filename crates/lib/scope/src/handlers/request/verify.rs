use anyhow::{Result, bail};
use rhex::{Rhex, data::RhexData, rt_structs::request::RtRequestRhex};

pub fn verify_request_rhex(rhex: &Rhex) -> Result<RtRequestRhex> {
    let request_rhex = match RhexData::from_slice(&rhex.data) {
        Err(_) => bail!("verify `request:rhex` could not CBOR decode data"),
        Ok(data) => match data {
            RhexData::Binary(bin) => RtRequestRhex::from_slice(&bin)?,
            RhexData::Json(json) => serde_json::from_slice::<RtRequestRhex>(&json)?,
            _ => bail!("verify `request:rhex` wrong data type (either ::Json or ::Binary"),
        },
    };

    Ok(request_rhex)
}
