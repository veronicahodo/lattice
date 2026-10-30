use anyhow::Result;
use lattice::{Lattice, Rhex};

pub fn process_verify(rhex: &Rhex, _lattice: &mut Lattice) -> Result<()> {
    match rhex.intent.rt.as_str() {
        _ => Ok(()),
    }
}
