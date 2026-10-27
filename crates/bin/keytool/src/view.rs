use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use key::Key;

pub fn view(key: &String, enclave_path: String, show_secret: bool, show_rust: bool) {
    let mut enclave = key::enclave::Enclave::new(Some(&enclave_path));
    let _ = enclave.populate();
    if key.as_str() == "" {
        let mut count: usize = 0;
        for k in enclave.keys {
            println!("{count}: {}", URL_SAFE_NO_PAD.encode(&k));
            count += 1;
        }
    } else {
        let key = Key::disk_get(key);
        println!("{}", key.pretty_format(show_secret));
        if show_rust {
            println!("Rust Array: {:?}", key.pk.unwrap());
        }
    }
}
