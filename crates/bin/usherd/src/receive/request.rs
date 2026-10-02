use std::path::PathBuf;

use anyhow::{Result, bail};
use key::enclave::Enclave;
use lattice::{
    Lattice, Rhex,
    rhex::{
        data::RhexData,
        signature::{AgileKey, RhexSigAlgo::Ed25519, RhexSignature, RhexSignatureType},
    },
};
use serde_json::json;

use crate::{
    config::UsherdConfig,
    receive::time_go::{TimeGoResponse, time_go, time_go_from_data},
};

pub fn process_request(
    config: &UsherdConfig,
    rhex: &Rhex,
    enclave: &Enclave,
    lattice: &Lattice,
    time: &u64,
) -> Result<Vec<Rhex>> {
    let mut out = Vec::new();
    match rhex.intent.rt.as_str() {
        "time:go" => {
            let data = time_go_from_data(RhexData::from_vec(&rhex.data)?)?;
            let sig = time_go(enclave, &rhex.intent.usher, &data.try_into().unwrap(), time)?;
            let response = TimeGoResponse {
                time: time.clone(),
                sig: RhexSignature {
                    pk: AgileKey {
                        algo: Ed25519,
                        key_bytes: rhex.intent.usher.try_into().unwrap(),
                    },
                    sig: sig.to_vec(),
                    t: RhexSignatureType::Observer(time.clone()),
                },
            };
            let new_data = RhexData::Binary(response.to_vec()?);

            let reply = Rhex::reply_to(
                rhex,
                &"time:resonse".to_string(),
                &Some("rhex://schema.time.response".to_string()),
                &new_data,
            )?;

            out.push(reply);
        }
        "request:scope" => {
            let scope = lattice.scopes.get(&rhex.intent.scope);
            if scope.is_none() {
                // FIXME: Yeah... this is terrible. this basically causes us
                // to crash because of client input. lulz.
                bail!("Invalid scope");
            }
            let path = if rhex.intent.scope.as_str() == "" {
                format!("{}-root-.rchain", config.scopes)
            } else {
                format!("{}{}.rchain", config.scopes, rhex.intent.scope)
            };
            let rchain = Rhex::chain_from_disk(&PathBuf::from(path))?;
            let json = json!({
                "status": "success",
                "records_following": rchain.len()
            });
            let data = &RhexData::Json(serde_json::to_vec(&json)?);
            let response = Rhex::reply_to(
                rhex,
                &"response:rhex".to_string(),
                &Some("rhex://schema.response.rhex".to_string()),
                data,
            )?;
            out.push(response);
            out.extend(rchain);
        }
        "request:head" => {
            let scope = lattice.scopes.get(&rhex.intent.scope);
            let data = if scope.is_none() {
                RhexData::Mixed {
                    meta: serde_json::to_vec(&json!({
                        "status": "failure",
                        "message": "That scope is not available here."
                    }))?,
                    binary: vec![],
                }
            } else {
                RhexData::Mixed {
                    meta: serde_json::to_vec(&json!({
                        "status": "success",
                        "slot0": "head"
                    }))?,
                    binary: scope.unwrap().head.unwrap().to_vec(),
                }
            };
            let reply = Rhex::reply_to(
                rhex,
                &"response:head".to_string(),
                &Some("rhex://schema.response.head".to_string()),
                &data,
            )?;

            out.push(reply);
        }
        "request:cid" => {}
        _ => {}
    };
    Ok(out)
}
