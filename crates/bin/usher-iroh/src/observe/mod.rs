use anyhow::Result;
use comms::chatter::{ChatterResponse, ChatterResponseType};
use iroh::PublicKey;
use key::enclave::Enclave;
use lattice::rhex::signature::{AgileKey, RhexSigAlgo, RhexSignature, RhexSignatureType};

pub fn sign_as_observer(
    enclave: &Enclave,
    key: &PublicKey,
    bin: &[u8; 32],
) -> Result<ChatterResponse> {
    let time = time::get_current_time();
    let sig = enclave.sign(key, bin)?;
    let rhex_sig = RhexSignature {
        pk: AgileKey {
            algo: RhexSigAlgo::Ed25519,
            key_bytes: key.to_vec(),
        },
        sig: sig.to_vec(),
        t: RhexSignatureType::Observer(time),
    };
    Ok(ChatterResponse {
        version: 0,
        response: ChatterResponseType::ObserverSignResponse(rhex_sig),
        payload: vec![],
    })
}
