use std::time;

use anyhow::Result;
use iroh::PublicKey;
use key::enclave::Enclave;
use scope::rhex::{
    self,
    context::RhexContext,
    data::RhexData,
    signature::{AgileKey, RhexSigAlgo::Ed25519, RhexSignature, RhexSignatureType},
};
use serde_json::json;

pub fn genesis(key: &[u8; 32], enclave: &Enclave, output: &String) -> Result<[u8; 32]> {
    let mut rhex = rhex::Rhex::new();
    //let key = key::Key::disk_get(&key);
    let key_conv = PublicKey::from_bytes(&key)?;

    let json = json!({
        "at": &time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_millis(),
    })
    .to_string();

    // Build data
    let data = RhexData::Mixed {
        meta: json.as_bytes().to_vec(),
        binary: key.to_vec(),
    };
    let buf = cbor2::to_canonical_vec(&data)?;
    let mut hasher: blake3::Hasher = blake3::Hasher::new();
    hasher.update(&buf);
    let hash = hasher.finalize();
    rhex.data = data.to_vec()?;

    // Build intent
    rhex.intent.prev = None;
    rhex.intent.scope = "".to_string();
    rhex.intent.author = key_conv.clone();
    rhex.intent.usher = key_conv.clone();
    rhex.intent.rt = "lattice:genesis".to_string();
    rhex.intent.schema = None;
    rhex.intent.data_hash = Some(hash.as_bytes().clone());

    // Add context since we are the usher too
    rhex.context = Some(RhexContext { at: 0, s: None });

    // Sign it like the dirty hoe we are.
    let key = AgileKey {
        algo: Ed25519,
        key_bytes: key_conv.as_bytes().to_vec(),
    };
    rhex.sigs.push(RhexSignature {
        pk: key.clone(),
        sig: enclave
            .sign(&key_conv, &rhex.get_hash(RhexSignatureType::Author))?
            .to_vec(),
        t: RhexSignatureType::Author,
    });
    rhex.sigs.push(RhexSignature {
        pk: key.clone(),
        sig: enclave
            .sign(&key_conv, &rhex.get_hash(RhexSignatureType::Usher))?
            .to_vec(),
        t: RhexSignatureType::Usher,
    });
    rhex.sigs.push(RhexSignature {
        pk: key.clone(),
        sig: enclave
            .sign(&key_conv, &rhex.get_hash(RhexSignatureType::Quorum(0)))?
            .to_vec(),
        t: RhexSignatureType::Quorum(0),
    });

    // Mark us complete
    rhex.curr = Some(rhex.calc_curr());

    rhex.single_disk_put(format!("{}/genesis.rhex", &output).as_str())?;

    Ok(rhex.curr.unwrap())
}
