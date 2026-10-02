use key::Key;

pub fn view(key: &String, enclave_path: String, show_secret: bool, show_rust: bool) {
    let mut enclave = key::enclave::Enclave::new(Some(&enclave_path));
    let _ = enclave.populate();
    let key = Key::disk_get(key);
    println!("{}", key.pretty_format(show_secret));
    if show_rust {
        println!("Rust Array: {:?}", key.pk.unwrap());
    }
}
