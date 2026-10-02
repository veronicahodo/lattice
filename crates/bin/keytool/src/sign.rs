use anyhow::Result;
use base64::engine::{Engine, general_purpose::URL_SAFE_NO_PAD};
use key::enclave;
use rhex::signature::{AgileKey, RhexSigAlgo::Ed25519, RhexSignature};

pub fn sign(
    key: &String,
    enclave_path: &String,
    sig_type: &String,
    rhexfile: &String,
    output: &Option<String>,
    time: &Option<u64>,
) -> Result<()> {
    // Load the rhex
    let mut rhex = rhex::Rhex::single_disk_get(rhexfile);
    // Build the signature
    let sig_type = match sig_type.as_str() {
        "author" => rhex::signature::RhexSignatureType::Author,
        "usher" => rhex::signature::RhexSignatureType::Usher,
        "quorum" => rhex::signature::RhexSignatureType::Quorum(time.unwrap().clone()),
        "observer" => rhex::signature::RhexSignatureType::Observer(time.unwrap().clone()),
        "other" => rhex::signature::RhexSignatureType::Other,
        _ => {
            println!("Invalid signature type");
            anyhow::bail!("Invalid signature type");
        }
    };
    let hash = rhex.get_hash(sig_type.clone());
    // Load the enclave and sign
    let mut enclave = enclave::Enclave::new(Some(&enclave_path.clone()));
    enclave.populate()?;
    let key = URL_SAFE_NO_PAD.decode(key).unwrap();
    let key: [u8; 32] = key.try_into().unwrap();
    let sig = enclave.sign(&key.clone(), &hash)?;
    // Add the signature to the rhex
    rhex.sigs.push(RhexSignature {
        pk: AgileKey {
            algo: Ed25519,
            key_bytes: key.try_into().unwrap(),
        },
        sig: sig.to_vec(),
        t: sig_type.clone(),
    });
    // store the rhex
    let output = match output {
        Some(o) => o,
        None => rhexfile,
    };
    rhex.single_disk_put(output);

    Ok(())
}
