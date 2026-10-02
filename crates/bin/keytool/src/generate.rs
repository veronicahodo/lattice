use anyhow::Result;
use key::enclave::Enclave;

pub fn generate(
    name: Option<String>,
    output: String,
    time: Option<u64>,
    expires: Option<u64>,
) -> Result<()> {
    println!("Generating key...");
    let mut enclave = Enclave::new(Some(&output.clone()));
    let key = enclave.generate(name, time, expires)?;
    println!("{}", key.pretty_format(true));
    Ok(())
}
