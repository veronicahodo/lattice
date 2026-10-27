use anyhow::Result;
use base64::engine::{Engine as _, general_purpose::URL_SAFE_NO_PAD};
use iroh::SecretKey;

pub fn import(
    enclave_path: &String,
    name: &Option<String>,
    input: &String,
    time: &Option<u64>,
    expires: &Option<u64>,
) -> Result<()> {
    let mut enclave = key::enclave::Enclave::new(Some(&enclave_path.clone()));
    let _ = enclave.populate();
    let secret_key = URL_SAFE_NO_PAD.decode(input).unwrap();
    let secret_key: [u8; 32] = secret_key.try_into().unwrap();
    let key = enclave
        .import(&SecretKey::from_bytes(&secret_key), name, time, expires)
        .unwrap();
    println!("{}", key.pretty_format(true));
    Ok(())
}
