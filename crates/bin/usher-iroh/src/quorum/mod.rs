use anyhow::Result;
use lattice::{
    Lattice, Rhex,
    rhex::{
        check::CheckStatus,
        context::RhexContext,
        intent::RhexIntent,
        signature::{AgileKey, RhexSigAlgo, RhexSignature, RhexSignatureType},
    },
};

use crate::comms::chatter::{ChatterResponse, ChatterResponseType};

pub mod sign;

pub fn sign_as_quorum(
    intent: &RhexIntent,
    context: &RhexContext,
    sigs: &Vec<RhexSignature>,
    lattice: &Lattice,
) -> Result<ChatterResponse> {
    let mut rhex = Rhex::build_with(intent, &Some(context.clone()), sigs, &None);
    let scope = lattice
        .scopes
        .get(&rhex.intent.scope)
        .expect("Not a member of scope");

    let status = scope.full_check(&mut rhex)?;

    if status.len() != 1 && status[0] != CheckStatus::Success {
        return Ok(ChatterResponse {
            version: 0,
            response: ChatterResponseType::Rejected(vec![]),
            payload: vec![],
        });
    };

    let sig_bytes = [0u8; 64];
    let time = time::get_current_time();
    let sig_hash = rhex.get_hash(RhexSignatureType::Quorum(time));
    Ok(ChatterResponse {
        version: 0,
        response: ChatterResponseType::QuorumSignResponse {
            sig_hash,
            sig: RhexSignature {
                pk: AgileKey {
                    algo: RhexSigAlgo::Ed25519,
                    key_bytes: [0u8, 32].to_vec(),
                },
                sig: sig_bytes.to_vec(),
                t: RhexSignatureType::Quorum(time),
            },
        },
        payload: vec![],
    })
}
