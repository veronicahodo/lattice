use anyhow::Result;
use key::enclave::Enclave;
use lattice::{
    Rhex,
    rhex::{
        intent::RhexIntent,
        signature::{AgileKey, RhexSignature, RhexSignatureType},
    },
};

/// # `sign_out(enclave, intents, pk)`
///
/// Basically takes a Vec of RhexIntent and signs over all of them
/// with the same key, and then returns the author-signed Rhex
///
pub fn sign_out(enclave: &Enclave, intent: RhexIntent, pk: &AgileKey) -> Result<Rhex> {
    let mut r = Rhex::new();
    r.intent = intent;
    let sig = enclave.sign(
        &pk.key_bytes.clone().try_into().unwrap(),
        &r.get_hash(RhexSignatureType::Author),
    )?;
    r.sigs.push(RhexSignature {
        pk: pk.clone(),
        sig: sig.to_vec(),
        t: RhexSignatureType::Author,
    });

    Ok(r)
}
