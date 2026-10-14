use std::fs;

use anyhow::Result;
use scope::rhex::Rhex;

pub fn finalize(input: String, output: String, use_curr: bool) -> Result<()> {
    let file = fs::read(input)?;
    let mut rhex: Rhex = cbor2::from_slice(&file)?;
    if rhex.sigs.len() < 2 {
        println!("[Warning] finalizing an incomplete record, this will be rejected by any usher.")
    }
    rhex.sort_sigs();
    rhex.curr = Some(rhex.calc_curr());
    println!("R⬢ Finalized!");
    println!("{}", rhex.pretty_print());
    let filename = if use_curr {
        format!("{}/{}.rhex", output, hex::encode(rhex.curr.unwrap()))
    } else {
        output.clone()
    };
    let bin = cbor2::to_canonical_vec(&rhex)?;
    fs::write(filename, bin)?;
    Ok(())
}
