use anyhow::Result;
use lattice::{Lattice, Rhex};

use crate::system;

pub fn process_walk(rhex: &Rhex, lattice: &mut Lattice) -> Result<()> {
    match rhex.intent.rt.as_str() {
        "lattice:genesis" => system::lattice::execute_lattice_genesis(rhex, lattice),
        _ => Ok(()),
    }
}
