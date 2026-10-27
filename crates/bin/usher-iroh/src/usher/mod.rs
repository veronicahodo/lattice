use anyhow::Result;
use iam::IAm;
use key::enclave::Enclave;
use lattice::{
    Lattice, Rhex,
    rhex::{
        check::{CheckStatus, ProcessChecks},
        context::RhexContext,
        intent::RhexIntent,
        signature::{RhexSignature, RhexSignatureType},
    },
};
use transform::{descriptor::DescriptorAction, registry::TransformRegistry};

use crate::{
    comms::chatter::{ChatterResponse, ChatterResponseType},
    trans,
};
use time;

pub mod check;
pub mod sign;

pub fn sign_as_usher(
    intent: &RhexIntent,
    sigs: &Vec<RhexSignature>,
    enclave: &Enclave,
    iam: &IAm,
    trans_registry: &TransformRegistry,
    lattice: &Lattice,
) -> Result<ChatterResponse> {
    let mut intents = Vec::new();
    let mut output = Vec::new();

    let mut rhex = Rhex::build_with(intent, &None, sigs, &None);

    // Do the checks!
    let check_output = run_checks(iam, &mut rhex, lattice);

    if let Ok(_) = check_output {
        let (trans_status, trans_intents) =
            trans::fire_transforms(&rhex, trans_registry, DescriptorAction::Validate)?;
        if trans_status != CheckStatus::Success {
            output.push(trans_status);
        } else {
            intents.extend(trans_intents.iter().cloned());
        }
    }

    // Gen context
    // FIXME: I know. This is like the third, maybe tenth redo of the
    // usher signing and we have YET to figure out where spacial data
    // is gonna come from. And YET again, I choose to not care right now.
    let time = time::get_current_time();
    let context = RhexContext { at: time, s: None };
    rhex.context = Some(context.clone());

    // Sign
    let usher_sig = sign::do_usher_sig(enclave, &rhex)?;
    rhex.sigs.push(usher_sig.clone());

    // build response
    let out = ChatterResponse {
        version: 0,
        response: ChatterResponseType::UsherSignResponse {
            usher_hash: rhex.get_hash(RhexSignatureType::Usher),
            context,
            sig: usher_sig.clone(),
        },
        payload: vec![],
    };

    // Return output
    Ok(out)
}

fn run_checks(
    iam: &IAm,
    rhex: &mut Rhex,
    lattice: &Lattice,
) -> Result<Vec<CheckStatus>, Vec<CheckStatus>> {
    let mut output = Vec::new();
    check::am_i(iam, &rhex.intent.author).check_and_push_into(&mut output)?;
    let scope = lattice.scopes.get(&rhex.intent.scope);
    check::control_scope(scope).check_and_push_into(&mut output)?;
    let scope = scope.unwrap();
    let time = time::get_current_time();
    scope
        .check_authors(rhex, &time)
        .unwrap_or_else(|_| vec![CheckStatus::SignatureInvalid(0)])
        .check_and_push_into(&mut output)?;
    scope
        .full_check(rhex)
        .unwrap()
        .check_and_push_into(&mut output)?;
    Ok(output)
}
