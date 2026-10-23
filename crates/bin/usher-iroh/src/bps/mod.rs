use anyhow::Result;
use key::enclave::Enclave;
use lattice::{
    Rhex,
    rhex::{
        data::RhexData,
        intent::RhexIntent,
        signature::{AgileKey, RhexSigAlgo::Ed25519, RhexSignature, RhexSignatureType},
    },
};

/// # bps(intent, enclave)
///
/// BUILD - PACK - SHIP baby...
///
/// I don't know why I called it this... but I did.
/// It takes a R⬢ intent and builds a full R⬢, signs it and
/// returns it
pub fn bps(intent: &RhexIntent, data: &Option<RhexData>, enclave: &Enclave) -> Result<Rhex> {
    // set up the new R⬢
    let mut rhex = Rhex::new();
    rhex.intent = intent.clone();

    // calc data_hash and populate the data.
    if let Some(d) = data {
        rhex.data = d.to_vec()?;
        rhex.intent.data_hash = Some(d.get_data_hash());
    };
    let sig = enclave.sign(
        &rhex.intent.author,
        &rhex.get_hash(RhexSignatureType::Author),
    )?;
    rhex.sigs.push(RhexSignature {
        pk: AgileKey {
            algo: Ed25519,
            key_bytes: intent.author.to_vec(),
        },
        sig: sig.to_vec(),
        t: RhexSignatureType::Author,
    });

    Ok(rhex)
}
