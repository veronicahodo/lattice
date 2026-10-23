use anyhow::Result;
use iroh::PublicKey;
use key::enclave::Enclave;
use lattice::{
    Rhex,
    rhex::signature::{AgileKey, RhexSigAlgo, RhexSignature, RhexSignatureType},
};

pub fn do_quorum_sig(
    enclave: &Enclave,
    rhex: &Rhex,
    key: &PublicKey,
    time: &u64,
) -> Result<RhexSignature> {
    let hash = rhex.get_hash(RhexSignatureType::Quorum(time.clone()));
    let sig = enclave.sign(key, &hash)?;
    Ok(RhexSignature {
        pk: AgileKey {
            algo: RhexSigAlgo::Ed25519,
            key_bytes: key.to_vec(),
        },
        sig: sig.to_vec(),
        t: RhexSignatureType::Quorum(time.clone()),
    })
}
