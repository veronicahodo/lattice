use std::fs;

use anyhow::Result;
use iroh::SecretKey;
use key::KeyInstanced;

pub fn vanity(
    sigil_prefix: String,
    name: Option<String>,
    time: Option<u64>,
    expires: Option<u64>,
    output: String,
) -> Result<()> {
    let mut checkpoint: u64 = 0;
    let mut done = false;
    let instanced = if time.is_some() {
        KeyInstanced::Generated(time.unwrap())
    } else {
        KeyInstanced::Unknown
    };
    let sigil_prefix = sigil_prefix.to_uppercase();
    while !done {
        let key_bytes: [u8; 32] = rand::random();
        let key = key::Key::new(
            &SecretKey::from_bytes(&key_bytes),
            &name.clone(),
            &instanced.clone(),
            &expires,
        );
        let sigil_id = key.sigid_id();
        if sigil_id.starts_with(&sigil_prefix) {
            println!("Sigil ID: {}", sigil_id);
            println!("Writing key to {}", output);
            let _ = fs::write(output.clone(), &key.to_vec()?);
            done = true;
        } else {
            checkpoint += 1;
            if checkpoint % 10_000 == 0 {
                println!("Checkpoint {}: {}", checkpoint, sigil_id);
            }
        }
    }
    Ok(())
}
