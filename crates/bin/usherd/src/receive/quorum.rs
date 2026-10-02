use anyhow::{Ok, Result};
use iam::IAm;
use key::enclave::Enclave;
use lattice::{
    Lattice, Rhex,
    rhex::{
        check::CheckStatus,
        data::RhexData,
        signature::{AgileKey, RhexSigAlgo::Ed25519, RhexSignature, RhexSignatureType},
    },
    scope::{Scope, ushers::UsherRole},
};
use rand::seq::IndexedRandom;
use time::MicroMarks;
use transform::{descriptor::DescriptorAction, registry::TransformRegistry};

use crate::{
    config::UsherdConfig,
    firing,
    receive::{ReceiveStatus, sign_out::sign_out},
};

/// # recv_two_sigs(...)
///
/// Fired when we receive a R⬢ with an author and usher signature,
/// for what is presumed to be quorum signature. The only time a R⬢ has
/// two signatures at the time of admission is when the k = 0 in the
/// policy of the scope, which should have been handled when the author
/// submitted to the original usher. Observational signatures are
/// handled as an `usher:observe` record type.
///
pub fn recv_two_sigs(
    rhex: &Rhex,
    _config: &UsherdConfig,
    lattice: &mut Lattice,
    enclave: &mut Enclave,
    iam: &mut IAm,
    trans_registry: &mut TransformRegistry,
) -> Result<(ReceiveStatus, Option<Vec<Rhex>>)> {
    // Set up the goodies
    let mut output = Vec::new();
    let mut status = ReceiveStatus::Success;

    // Get the scope in question
    let scope = lattice.scopes.get(&rhex.intent.scope);
    if scope.is_none() {
        status = ReceiveStatus::FailedValidation(CheckStatus::InvalidUsher);
    }

    // Get the time
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    let time = time.as_micromarks();

    // Are we at least a quorum usher for this scope?
    let mut usher: [u8; 32] = [0; 32];
    if status == ReceiveStatus::Success {
        usher = select_quorum(scope.unwrap(), iam, time)?;
    }

    // Does the R⬢ pass validation?
    let statuses = scope.unwrap().full_check(rhex)?;

    if statuses[0] != CheckStatus::Success {
        status = ReceiveStatus::FailedValidation(statuses[0].clone());
    } else {
        // fire validation transforms
        let (trans_status, out_intent) =
            firing::fire_transforms(rhex, trans_registry, DescriptorAction::Validate)?;
        if trans_status != CheckStatus::Success {
            status = ReceiveStatus::FailedValidation(trans_status);
        }
        // tack transform output to the output going to
        for intent in out_intent {
            output.push(sign_out(
                enclave,
                intent.clone(),
                &AgileKey {
                    algo: Ed25519,
                    key_bytes: intent.author.try_into().unwrap(),
                },
            )?);
        }
    }

    // Generate quorum signature
    let sig = enclave.sign(&usher, &rhex.get_hash(RhexSignatureType::Quorum(time)))?;
    let sig_package = RhexSignature {
        pk: AgileKey {
            algo: Ed25519,
            key_bytes: usher.try_into().unwrap(),
        },
        sig: sig.to_vec(),
        t: RhexSignatureType::Quorum(time),
    };

    // Package output
    let mut new_rhex = Rhex::new();
    new_rhex.intent.prev = None;
    new_rhex.intent.scope = rhex.intent.scope.clone();
    new_rhex.intent.author = usher.clone();
    new_rhex.intent.usher = rhex.intent.author.clone();
    new_rhex.intent.schema = Some("rhex://schema.rhex.quorum.signature".to_string());
    new_rhex.intent.rt = "signature:quorum".to_string();
    let data = RhexData::Binary(sig_package.to_vec()?);
    new_rhex.intent.data_hash = Some(data.get_hash());
    new_rhex.data = data.to_vec()?;

    // Sign output
    let author_hash = new_rhex.get_hash(RhexSignatureType::Author);
    let author_sig = enclave.sign(&usher, &author_hash)?;
    let out_sig = RhexSignature {
        pk: AgileKey {
            algo: Ed25519,
            key_bytes: usher.try_into().unwrap(),
        },
        sig: author_sig.to_vec(),
        t: RhexSignatureType::Author,
    };
    new_rhex.sigs.push(out_sig);
    output.push(new_rhex);

    // Return
    Ok((status, Some(output)))
}

/// # select_quorum(scope, local_keys)
///
/// Selects a quorum member to return their PK so we can sign as that
/// key before returning the data
///
pub fn select_quorum(scope: &Scope, iam: &IAm, time: u64) -> Result<[u8; 32]> {
    let ushers = scope.ushers_by_role(time)?;
    let mut working = Vec::new();
    for usher in ushers {
        if usher.0 == UsherRole::Quorum || usher.0 == UsherRole::Actor {
            working.push(usher.1);
        }
    }
    let ushers = iam.am_i_any(working, true)?;
    let random_selection = ushers.choose(&mut rand::rng());
    Ok(random_selection.unwrap().clone())
}

/*

    Old shit. Will remove soon.

pub fn recv_quorum_sig(
    scope: &Scope,
    rhex: &Rhex,
    enclave: &Enclave,
    me: &IAm,
) -> Result<(CheckStatus, Option<RhexSignature>)> {
    if rhex.check_sig(0)? == CheckStatus::Success && rhex.check_sig(1)? == CheckStatus::Success {
        let policy = scope.get_policy_at(rhex.context.at);
        let quorum_key = me.pick(scope, rhex.context.at, UsherRole::Quorum)?;
        let quorum_key = if quorum_key.is_some() {
            quorum_key.unwrap()
        } else {
            return Ok((CheckStatus::NotUsherForThisScope, None));
        };
        let window = policy.get_window(
            &rhex.intent.rt.to_string(),
            &scope.member_of_at(rhex.intent.author, rhex.context.at)?,
        )?;
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis() as i64;
        if (time.as_micromarks() - window) < rhex.context.at {
            // Our sig request was received in the appropriate window
            let sig = enclave.sign(
                &quorum_key,
                &rhex.get_hash(RhexSignatureType::Quorum(time.as_micromarks())),
            )?;
            let sig_rec = {
                RhexSignature {
                    pk: quorum_key.clone(),
                    sig,
                    t: RhexSignatureType::Quorum(time.as_micromarks()),
                }
            };
            return Ok((CheckStatus::Success, Some(sig_rec)));
        };
    }
    Ok((CheckStatus::SignatureInvalid(0), None))
}
 */
