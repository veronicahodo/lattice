use anyhow::Result;
use key::enclave::Enclave;
use lattice::{
    Rhex,
    rhex::signature::{AgileKey, RhexSigAlgo, RhexSignature, RhexSignatureType},
};

pub fn do_usher_sig(enclave: &Enclave, rhex: &Rhex) -> Result<RhexSignature> {
    let hash = rhex.get_hash(RhexSignatureType::Usher);
    let sig = enclave.sign(&rhex.intent.usher, &hash)?;
    Ok(RhexSignature {
        pk: AgileKey {
            algo: RhexSigAlgo::Ed25519,
            key_bytes: rhex.intent.usher.to_vec(),
        },
        sig: sig.to_vec(),
        t: RhexSignatureType::Usher,
    })
}
