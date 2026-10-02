use anyhow::Result;
use iam::IAm;
use key::enclave::Enclave;
use lattice::{
    Lattice, Rhex,
    rhex::{
        check::CheckStatus,
        context::RhexContext,
        data::RhexData,
        signature::{AgileKey, RhexSigAlgo::Ed25519, RhexSignature, RhexSignatureType},
    },
    usher::UsherSigResponse,
};
use time::MicroMarks;
use transform::{descriptor::DescriptorAction, registry::TransformRegistry};

use crate::{
    firing,
    receive::{
        ReceiveStatus,
        sign_out::sign_out,
        time_go::{TimeGoResponse, time_go, time_go_from_data},
    },
};

/// # recv_one_sig(...)
///
/// This is the entry point for all the heavy lifting to do with signing
/// as an usher over a R⬢'s context and author signature.
///
/// Note: Everything this is borrowed here. As it stands at this point
/// in the R⬢ cycle, nothing should be changeable, as we still need
/// quorum sign off, or even if not (zero quorum scopes), that gets
/// handled elsewhere.
///
pub fn recv_one_sig(
    rhex: &Rhex,
    enclave: &Enclave,
    lattice: &Lattice,
    iam: &IAm,
    trans_registry: &TransformRegistry,
) -> Result<(ReceiveStatus, Option<Vec<Rhex>>)> {
    // setup the useful things
    let mut output = Vec::new();
    let mut status = ReceiveStatus::Success;
    let mut out_rhex = Rhex::new();
    let mut intent_to_sign = Vec::new();
    out_rhex.intent = rhex.intent.clone();
    out_rhex.sigs = rhex.sigs.clone();
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    let time = time.as_micromarks();

    // Is the only sig an author signature
    if rhex.sigs[0].t != RhexSignatureType::Author {
        status = ReceiveStatus::FailedValidation(CheckStatus::SignatureInvalid(0));
    }

    // Fall out early for `time:go` R⬢. This is literally the worst
    // because this is the beginning of making all directive/query
    // `rt` requires hardcoding here. 🤮
    if rhex.intent.rt.as_str() == "time:go" {
        let hash = time_go_from_data(RhexData::from_vec(&rhex.data)?)?;
        let sig = time_go(enclave, &rhex.intent.usher, &hash, &time)?;
        let mut rout = Rhex::new();
        rout.intent.rt = "time:response".to_string();
        rout.intent.author = rhex.intent.usher;
        rout.intent.usher = rhex.intent.author;
        rout.intent.schema = Some("rhex://schema.time.response".to_string());
        let response = TimeGoResponse {
            time,
            sig: RhexSignature {
                pk: AgileKey {
                    algo: Ed25519,
                    key_bytes: rhex.intent.usher.try_into().unwrap(),
                },
                sig: sig.to_vec(),
                t: RhexSignatureType::Observer(time),
            },
        };
        let data = RhexData::Binary(response.to_vec()?);
        rout.intent.data_hash = Some(data.get_hash());
        let mut rout = sign_out(
            enclave,
            rout.intent,
            &AgileKey {
                algo: Ed25519,
                key_bytes: rhex.intent.usher.try_into().unwrap(),
            },
        )?;
        rout.data = data.to_vec()?;
        return Ok((ReceiveStatus::Success, Some(vec![rout])));
    }

    // Am I the usher in question?
    if status == ReceiveStatus::Success {
        let am = iam.am_i(&rhex.intent.usher)?;
        if !am {
            status = ReceiveStatus::FailedValidation(CheckStatus::InvalidUsher);
        }
    }

    // Do I manage this scope?
    let scope = if status == ReceiveStatus::Success {
        let scope = lattice.scopes.get(&rhex.intent.scope);
        if scope.is_none() {
            status = ReceiveStatus::FailedValidation(CheckStatus::NotUsherForThisScope);
        };
        scope
    } else {
        None
    };

    // Does this R⬢ pass the checks?
    if status == ReceiveStatus::Success {
        let mut check_status = scope.unwrap().full_check(rhex)?;
        if check_status[0] == CheckStatus::Success {
            let (transform_status, intents) =
                firing::fire_transforms(rhex, &trans_registry, DescriptorAction::Validate)?;
            if transform_status != CheckStatus::Success {
                check_status = vec![transform_status];
            } else {
                intent_to_sign.extend(intents.iter().cloned());
            }
        }
        if check_status[0] != CheckStatus::Success {
            status = ReceiveStatus::FailedValidation(check_status[0].clone());
        }
    }

    // Generate context
    if status == ReceiveStatus::Success {
        // TODO: One day... one day I'll sit down and solve how spacial
        // data works. Today is not that day.
        let at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis() as i64;
        out_rhex.context = RhexContext::new(at.as_micromarks(), None);
    }

    // Sign over author sig and context
    let sig = enclave.sign(
        &rhex.intent.usher,
        &out_rhex.get_hash(RhexSignatureType::Usher),
    )?;
    let rhex_sig = RhexSignature {
        pk: AgileKey {
            algo: Ed25519,
            key_bytes: rhex.intent.usher.try_into().unwrap(),
        },
        sig: sig.to_vec(),
        t: RhexSignatureType::Usher,
    };

    // Build response
    let mut out_rhex = Rhex::new();
    out_rhex.intent.prev = None;
    out_rhex.intent.scope = rhex.intent.scope.clone();
    out_rhex.intent.author = rhex.intent.usher.clone();
    out_rhex.intent.usher = rhex.intent.author.clone();
    out_rhex.intent.schema = Some("rhex://schema.usher.sig.response/@0".to_string());
    out_rhex.intent.rt = "usher:signature".to_string();
    let usher_sig_response = UsherSigResponse {
        context: out_rhex.context.clone(),
        sig: rhex_sig,
    };
    let data = RhexData::Binary(usher_sig_response.to_vec()?);
    out_rhex.data = data.to_vec()?;
    out_rhex.intent.data_hash = Some(data.get_hash());

    // Sign output
    let sig = enclave.sign(
        &rhex.intent.usher,
        &out_rhex.get_hash(RhexSignatureType::Author),
    )?;
    out_rhex.sigs.push(RhexSignature {
        pk: AgileKey {
            algo: Ed25519,
            key_bytes: rhex.intent.usher.try_into().unwrap(),
        },
        sig: sig.to_vec(),
        t: RhexSignatureType::Author,
    });

    // Return response
    output.push(out_rhex);
    let output_packed = if output.len() > 0 { Some(output) } else { None };
    Ok((status, output_packed))
}
