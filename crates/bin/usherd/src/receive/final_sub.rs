use std::path::PathBuf;

use anyhow::Result;
use iam::IAm;
use key::enclave::Enclave;
use lattice::{
    Lattice, Rhex,
    rhex::{
        check::CheckStatus,
        data::RhexData,
        signature::{AgileKey, RhexSigAlgo::Ed25519, RhexSignature, RhexSignatureType},
    },
};
use serde_json::json;
use time::MicroMarks;

use crate::{config::UsherdConfig, receive::ReceiveStatus};

pub fn recv_three_plus_sigs(
    rhex: &Rhex,
    lattice: &mut Lattice,
    iam: &mut IAm,
    enclave: &mut Enclave,
    config: &UsherdConfig,
) -> Result<(ReceiveStatus, Option<Vec<Rhex>>)> {
    // Setup the basics
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_millis() as i64;

    // Do I have this scope?
    let scope = lattice.scopes.get_mut(&rhex.intent.scope);
    if scope.is_none() {
        return Ok((
            ReceiveStatus::FailedValidation(CheckStatus::NotUsherForThisScope),
            None,
        ));
    }
    let scope = scope.unwrap();

    // Am I this usher?
    let am_i = iam.am_i_any(vec![rhex.intent.usher.clone()], true)?;
    if am_i.len() == 0 {
        return Ok((
            ReceiveStatus::FailedValidation(CheckStatus::NotUsherForThisScope),
            None,
        ));
    }

    // Does the R⬢ validate?
    let valid_result = scope.final_check(rhex, &time.as_micromarks())?;
    if valid_result[0] != CheckStatus::Success {
        return Ok((
            ReceiveStatus::FailedValidation(valid_result[0].clone()),
            None,
        ));
    }

    // Append
    let scope_name = scope.name.clone();
    let disk_name = match scope_name.as_str() {
        "" => "-root-".to_string(),
        _ => scope_name.clone(),
    };
    let scope_path = PathBuf::from(format!("{}/{}.rchain", config.scopes, disk_name));
    scope.append(rhex, time.as_micromarks(), &scope_path)?;

    // Fire Action Transforms
    // TODO: Fire transforms. I have work in 45 min and I'm not even
    // approaching that right now.

    // Collect status
    // TODO: This is processing the transform output so like... kinda
    // have to have the above step completed to do this part

    // Build response
    let mut response = Rhex::new();
    response.intent.prev = None;
    response.intent.scope = rhex.intent.scope.clone();
    response.intent.author = rhex.intent.usher.clone();
    response.intent.usher = rhex.intent.author.clone();
    response.intent.schema = Some("rhex://schema.scope.append.confirmation".to_string());
    response.intent.rt = "append:response".to_string();
    let meta = json!({
        "status": "success",
        "scope": scope_name.clone(),
        "bin": "new scope head"
    });
    let data = RhexData::Mixed {
        meta: serde_json::to_vec(&meta)?,
        binary: rhex.curr.unwrap().to_vec(),
    };
    response.intent.data_hash = Some(data.get_hash());
    response.data = data.to_vec()?;
    let author_sig = enclave.sign(
        &response.intent.author,
        &response.get_hash(RhexSignatureType::Author),
    )?;
    let sig = RhexSignature {
        pk: AgileKey {
            algo: Ed25519,
            key_bytes: response.intent.author.try_into().unwrap(),
        },
        sig: author_sig.to_vec(),
        t: RhexSignatureType::Author,
    };
    response.sigs.push(sig);
    let mut output = Vec::new();
    output.push(response);

    // Output
    Ok((ReceiveStatus::Success, Some(output)))
}
