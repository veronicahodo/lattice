use anyhow::Result;
use iroh::PublicKey;
use key::enclave::Enclave;
use scope::{
    policy::Policy,
    rhex::{
        Rhex,
        context::RhexContext,
        data::RhexData,
        intent::RhexIntent,
        signature::{AgileKey, RhexSigAlgo, RhexSignature, RhexSignatureType},
    },
    rule::Rule,
};

pub fn first_policy(
    prev: &[u8; 32],
    key: &[u8; 32],
    enclave: &Enclave,
    output: &String,
) -> Result<[u8; 32]> {
    let policy = Policy {
        desc: "Inital Lattice Policy Set".to_string(),
        rules: vec![Rule {
            append: vec!["world_line_zero".to_string()],
            k: 1,
            o: 1,
            quorum: vec!["world_line_zero".to_string(), "root_group".to_string()],
            delay: 1_000,
            rt: vec![
                "policy:*".to_string(),
                "key:*".to_string(),
                "usher:*".to_string(),
                "scope:*".to_string(),
                "record:markdown".to_string(),
            ],
            window: 10_000,
        }],
        eff: 1,
        exp: 1_000_000_000_000_000,
        tags: vec![],
        issued: 1,
    };

    let data = RhexData::Binary(policy.to_vec()?);
    let pubkey = PublicKey::from_bytes(key)?;

    let mut intent = RhexIntent::new();
    intent.prev = Some(prev.clone());
    intent.scope = "".to_string();
    intent.author = pubkey.clone();
    intent.usher = pubkey.clone();
    intent.schema = Some("rhex://schema.scopes.policy.set/@0".to_string());
    intent.rt = "policy:set".to_string();
    intent.data_hash = Some(data.get_data_hash());

    let context = RhexContext { at: 1, s: None };

    let mut rhex = Rhex::build_with(&intent, &Some(context), &vec![], &Some(data.to_vec()?));
    let agile_key = AgileKey {
        algo: RhexSigAlgo::Ed25519,
        key_bytes: pubkey.as_bytes().to_vec(),
    };
    let author_sig = RhexSignature {
        pk: agile_key.clone(),
        sig: enclave
            .sign(&pubkey, &rhex.get_hash(RhexSignatureType::Author))?
            .to_vec(),
        t: RhexSignatureType::Author,
    };
    rhex.sigs.push(author_sig);
    let usher_sig = RhexSignature {
        pk: agile_key.clone(),
        sig: enclave
            .sign(&pubkey, &rhex.get_hash(RhexSignatureType::Usher))?
            .to_vec(),
        t: RhexSignatureType::Usher,
    };
    rhex.sigs.push(usher_sig);
    let quorum_sig = RhexSignature {
        pk: agile_key.clone(),
        sig: enclave
            .sign(&pubkey, &rhex.get_hash(RhexSignatureType::Quorum(1)))?
            .to_vec(),
        t: RhexSignatureType::Quorum(1),
    };
    rhex.sigs.push(quorum_sig);
    rhex.curr = Some(rhex.calc_curr());
    rhex.single_disk_put(format!("{}/first_policy.rhex", output).as_str())?;

    Ok(rhex.curr.unwrap())
}
