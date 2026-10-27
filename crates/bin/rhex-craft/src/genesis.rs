use std::time;

use anyhow::Result;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use iroh::PublicKey;
use key::enclave::Enclave;
use scope::rhex::{
    self,
    context::RhexContext,
    data::RhexData,
    signature::{AgileKey, RhexSigAlgo::Ed25519, RhexSignature, RhexSignatureType},
};
use serde_json::json;

pub fn genesis(key: String, enclave_path: Option<String>, output: String) -> Result<()> {
    let enclave_path = match enclave_path {
        Some(ep) => ep,
        None => "./keys".to_string(),
    };
    let mut rhex = rhex::Rhex::new();
    //let key = key::Key::disk_get(&key);
    let key_conv: [u8; 32] = URL_SAFE_NO_PAD.decode(key)?.try_into().unwrap();
    let key_conv = PublicKey::from_bytes(&key_conv)?;
    let mut enclave = Enclave::new(Some(&enclave_path));
    let _ = enclave.populate();
    let binary_vec = vec![key_conv.clone()];
    let json = json!({
        "at": time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs(),
    })
    .to_string();

    // Build data
    let data = RhexData::Mixed {
        meta: json.as_bytes().to_vec(),
        binary: cbor2::to_canonical_vec(&binary_vec)?,
    };
    let buf = cbor2::to_canonical_vec(&data)?;
    let mut hasher = blake3::Hasher::new();
    hasher.update(&buf);
    let hash = hasher.finalize();
    // Build intent
    rhex.intent.prev = None;
    rhex.intent.scope = "".to_string();
    rhex.intent.author = PublicKey::from_bytes(&key_conv)?;
    rhex.intent.usher = PublicKey::from_bytes(&key_conv)?;
    rhex.intent.rt = "lattice:genesis".to_string();
    rhex.intent.schema = None;
    rhex.intent.data_hash = Some(*hash.as_bytes());

    // Add context since we are the usher too
    rhex.context = Some(RhexContext { at: 0, s: None });

    // Sign it like the dirty hoe we are.
    let key = AgileKey {
        algo: Ed25519,
        key_bytes: key_conv.as_bytes().try_into().unwrap(),
    };
    rhex.sigs.push(RhexSignature {
        pk: key.clone(),
        sig: enclave
            .sign(&key_conv, &rhex.get_hash(RhexSignatureType::Author))?
            .try_into()
            .unwrap(),
        t: RhexSignatureType::Author,
    });
    rhex.sigs.push(RhexSignature {
        pk: key.clone(),
        sig: enclave
            .sign(&key_conv, &rhex.get_hash(RhexSignatureType::Usher))?
            .try_into()
            .unwrap(),
        t: RhexSignatureType::Usher,
    });
    rhex.sigs.push(RhexSignature {
        pk: key.clone(),
        sig: enclave
            .sign(&key_conv, &rhex.get_hash(RhexSignatureType::Quorum(0)))?
            .try_into()
            .unwrap(),
        t: RhexSignatureType::Quorum(0),
    });

    // Mark us complete
    rhex.curr = Some(rhex.calc_curr());

    rhex.single_disk_put(&output);
    Ok(())
}
