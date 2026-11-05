use ::key::enclave::Enclave;
use anyhow::{Result, bail};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use iroh::PublicKey;
use serde_json::Value;

pub mod genesis;
pub mod key;
pub mod manifesto;
pub mod policy;

pub fn do_genesis(config: String) -> Result<[u8; 32]> {
    let json_bin = std::fs::read(config)?;
    let json: Value = serde_json::from_slice(&json_bin)?;
    println!("{}", json);
    let key = json.get("master_key");
    let key: [u8; 32] = match key {
        None => bail!("master_key not set"),
        Some(key) => URL_SAFE_NO_PAD
            .decode(key.as_str().unwrap())?
            .try_into()
            .unwrap(),
    };

    let key = PublicKey::from_bytes(&key)?;
    println!("{:?}", key);

    let enclave_path = match json.get("enclave_path") {
        Some(ep) => ep.as_str().unwrap().to_string(),
        None => "./keys/".to_string(),
    };

    let mut enclave = Enclave::new(Some(&enclave_path));
    enclave.populate()?;

    let output = match json.get("output_dir") {
        None => "./".to_string(),
        Some(out) => out.as_str().unwrap().to_string(),
    };
    let prev = genesis::genesis(&key, &enclave, &output)?;
    let mut prev = policy::first_policy(&prev, &key, &enclave, &output)?;
    let mut count: usize = 0;
    let manifesto = json.get("manifesto");
    if manifesto.is_some() {
        let manifesto = manifesto.unwrap().as_array();
        println!("Manifesto from JSON: {:?}", manifesto);
        if manifesto.is_some() {
            for m in manifesto.unwrap() {
                let path = m.as_str().unwrap();
                if std::fs::exists(path)? {
                    let mano_payload = std::fs::read_to_string(path)?;
                    println!("{}", output);
                    prev = manifesto::make_manifesto_entry(
                        &key,
                        &prev,
                        &mano_payload,
                        &count,
                        &enclave,
                        &output,
                    )?;
                }
                count += 1;
            }
        }
    }
    let time = match manifesto {
        Some(m) => (m.as_array().unwrap().len() + 2) as u64,
        None => 3 as u64,
    };
    let root = json.get("root");
    let root_keys = match root {
        None => vec![],
        Some(root_entry) => root_entry.as_array().unwrap().clone(),
    };
    let mut out = Vec::new();
    for root in root_keys {
        let key_bin = URL_SAFE_NO_PAD.decode(root.as_str().unwrap())?;
        let key: [u8; 32] = key_bin.try_into().unwrap();
        out.push(PublicKey::from_bytes(&key)?);
    }
    prev = key::first_key_grant(&prev, &key, &out, &enclave, &time, &output)?;

    Ok(prev)
}
