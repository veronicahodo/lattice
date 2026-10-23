use std::path::PathBuf;

use anyhow::{Result, bail};
use scope::rhex;

pub fn view(input: String) -> Result<()> {
    if input.ends_with(".rhex") {
        let rhex = rhex::Rhex::single_disk_get(&input);
        println!("{}", rhex.pretty_print());
        let valid = rhex.validate()?;
        match valid {
            true => println!("✅ Valid"),
            false => println!("❌ Invalid"),
        }
    } else if input.ends_with(".rchain") {
        let rhex = rhex::Rhex::chain_from_disk(&PathBuf::from(input))?;
        let mut count = 0;
        for r in rhex {
            println!("R⬢ #{} in chain:", count);
            println!("{}", r.pretty_print());
            if r.validate()? {
                println!("✅ Valid");
            } else {
                println!("❌ Invalid");
            }
            count += 1;
        }
    } else {
        bail!("Invalid file extension (must end in 'rhex' or 'rchain')")
    };
    Ok(())
}
