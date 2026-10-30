use anyhow::Result;
use iroh::PublicKey;
use key::enclave::Enclave;
use lattice::{
    Lattice, Rhex,
    rhex::{
        check::CheckStatus,
        context::RhexContext,
        intent::RhexIntent,
        signature::{AgileKey, RhexSigAlgo, RhexSignature, RhexSignatureType},
    },
};

use comms::chatter::{ChatterResponse, ChatterResponseType};

pub mod sign;

pub fn sign_as_quorum(
    address: &PublicKey,
    enclave: &Enclave,
    intent: &RhexIntent,
    context: &RhexContext,
    sigs: &Vec<RhexSignature>,
    lattice: &Lattice,
) -> Result<ChatterResponse> {
    let rhex = Rhex::build_with(intent, &Some(context.clone()), sigs, &None);
    let scope = lattice
        .scopes
        .get(&rhex.intent.scope)
        .expect("Not a member of scope");
    let time = time::get_current_time();
    let status = scope.full_check(&rhex)?;

    if status.len() != 1 && status[0] != CheckStatus::Success {
        return Ok(ChatterResponse {
            version: 0,
            response: ChatterResponseType::Rejected(vec![]),
            payload: vec![],
        });
    };
    let sig_hash = rhex.get_hash(RhexSignatureType::Quorum(time));
    let sig_bytes = enclave.sign(address, &sig_hash)?;
    Ok(ChatterResponse {
        version: 0,
        response: ChatterResponseType::QuorumSignResponse {
            sig_hash,
            sig: RhexSignature {
                pk: AgileKey {
                    algo: RhexSigAlgo::Ed25519,
                    key_bytes: address.to_vec(),
                },
                sig: sig_bytes.to_vec(),
                t: RhexSignatureType::Quorum(time),
            },
        },
        payload: vec![],
    })
}
