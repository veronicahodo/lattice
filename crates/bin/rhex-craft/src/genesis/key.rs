use anyhow::Result;
use iroh::PublicKey;
use key::enclave::Enclave;
use scope::rhex::{
    Rhex,
    context::RhexContext,
    data::RhexData,
    intent::RhexIntent,
    signature::{AgileKey, RhexSigAlgo, RhexSignature, RhexSignatureType},
};
use serde_json::json;

pub fn first_key_grant(
    prev: &[u8; 32],
    master_key: &PublicKey,
    keys: &Vec<PublicKey>,
    enclave: &Enclave,
    time: &u64,
    output: &String,
) -> Result<[u8; 32]> {
    let json = json!({
        "groups": ["root"],
    });
    let data = RhexData::Mixed {
        meta: serde_json::to_vec(&json)?,
        binary: cbor2::to_canonical_vec(keys)?,
    };
    let mut intent = RhexIntent::new();

    intent.prev = Some(prev.clone());
    intent.scope = "".to_string();
    intent.author = master_key.clone();
    intent.usher = master_key.clone();
    intent.schema = Some("rhex://schema.key.grant/@0".to_string());
    intent.rt = "key:grant".to_string();
    intent.data_hash = Some(data.get_data_hash());

    let context = RhexContext {
        at: time.clone(),
        s: None,
    };

    let mut rhex = Rhex::build_with(&intent, &Some(context), &vec![], &Some(data.to_vec()?));
    let agile_key = AgileKey {
        algo: RhexSigAlgo::Ed25519,
        key_bytes: master_key.to_vec(),
    };

    let author_sig = RhexSignature {
        pk: agile_key.clone(),
        sig: enclave
            .sign(master_key, &rhex.get_hash(RhexSignatureType::Author))?
            .to_vec(),
        t: RhexSignatureType::Author,
    };
    rhex.sigs.push(author_sig);

    let usher_sig = RhexSignature {
        pk: agile_key.clone(),
        sig: enclave
            .sign(master_key, &rhex.get_hash(RhexSignatureType::Usher))?
            .to_vec(),
        t: RhexSignatureType::Usher,
    };
    rhex.sigs.push(usher_sig);

    let quorum_sig = RhexSignature {
        pk: agile_key.clone(),
        sig: enclave
            .sign(
                master_key,
                &rhex.get_hash(RhexSignatureType::Quorum(time.clone())),
            )?
            .to_vec(),
        t: RhexSignatureType::Quorum(time.clone()),
    };
    rhex.sigs.push(quorum_sig);

    rhex.curr = Some(rhex.calc_curr());
    rhex.single_disk_put(format!("{}/key_grant.rhex", output).as_str())?;

    Ok(rhex.curr.unwrap())
}
