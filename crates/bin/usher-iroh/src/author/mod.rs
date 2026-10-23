use anyhow::Result;
use key::enclave::Enclave;
use lattice::rhex::{
    intent::RhexIntent,
    signature::{AgileKey, RhexSigAlgo, RhexSignature, RhexSignatureType},
};

use crate::comms::chatter::{ChatterResponse, ChatterResponseType};

pub fn sign_as_author(
    intent: &RhexIntent,
    _token: &[u8],
    enclave: &Enclave,
) -> Result<ChatterResponse> {
    let intent_hash = intent.get_hash();
    // FIXME: This literally does nothing with the token which is supposed
    // to be our authentication method.
    // CRITICAL!

    // FIXME: Should also, you know... check IAm to see if we can even
    // sign as the author.
    let sig = enclave.sign(&intent.author, &intent_hash)?;
    let out_sig = RhexSignature {
        pk: AgileKey {
            algo: RhexSigAlgo::Ed25519,
            key_bytes: intent.author.to_vec(),
        },
        sig: sig.to_vec(),
        t: RhexSignatureType::Author,
    };
    Ok(ChatterResponse {
        version: 0,
        response: ChatterResponseType::AuthorSignResponse {
            intent_hash,
            sig: out_sig,
        },
        payload: vec![],
    })
}
