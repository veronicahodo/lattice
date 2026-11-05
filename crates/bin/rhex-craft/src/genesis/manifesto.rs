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

pub fn make_manifesto_entry(
    key: &PublicKey,
    prev: &[u8; 32],
    payload: &String,
    count: &usize,
    enclave: &Enclave,
    output: &String,
) -> Result<[u8; 32]> {
    let data = RhexData::Binary(payload.as_bytes().to_vec());
    let time = (count + 2) as u64;

    let mut intent = RhexIntent::new();
    intent.prev = Some(prev.clone());
    intent.scope = "".to_string();
    intent.author = key.clone();
    intent.usher = key.clone();
    intent.schema = Some("rhex://schema.lattice.manifesto/@0".to_string());
    intent.rt = "record:markdown".to_string();
    intent.data_hash = Some(data.get_data_hash());

    let context = RhexContext {
        at: time.clone(),
        s: None,
    };

    let mut rhex = Rhex::build_with(&intent, &Some(context), &vec![], &Some(data.to_vec()?));
    let agile_key = AgileKey {
        algo: RhexSigAlgo::Ed25519,
        key_bytes: key.as_bytes().to_vec(),
    };

    let author_sig = RhexSignature {
        pk: agile_key.clone(),
        sig: enclave
            .sign(key, &rhex.get_hash(RhexSignatureType::Author))?
            .to_vec(),
        t: RhexSignatureType::Author,
    };
    rhex.sigs.push(author_sig);

    let usher_sig = RhexSignature {
        pk: agile_key.clone(),
        sig: enclave
            .sign(key, &rhex.get_hash(RhexSignatureType::Usher))?
            .to_vec(),
        t: RhexSignatureType::Usher,
    };
    rhex.sigs.push(usher_sig);

    let quorum_sig = RhexSignature {
        pk: agile_key.clone(),
        sig: enclave
            .sign(key, &rhex.get_hash(RhexSignatureType::Quorum(time.clone())))?
            .to_vec(),
        t: RhexSignatureType::Quorum(time.clone()),
    };
    rhex.sigs.push(quorum_sig);

    rhex.curr = Some(rhex.calc_curr());

    let path = format!("{}/manifesto_{}.rhex", output, count);
    println!("Path: {}", path);
    rhex.single_disk_put(&path.as_str())?;
    Ok(rhex.curr.unwrap())
}
