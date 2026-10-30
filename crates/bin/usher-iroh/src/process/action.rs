use anyhow::Result;
use lattice::{Lattice, Rhex};

use crate::system::lattice::execute_lattice_genesis;

/// # process_action(rhex, lattice)
///
/// In the world of unholy functions, this is the Devil himself.
/// If this is fired without validation shit starts breaking REAL
/// fucking fast. DO NOT USE until you have completely verified the R⬢
/// and confirmed access controls for this to get fired by an append
///
pub fn process_action(rhex: &Rhex, lattice: &mut Lattice) -> Result<()> {
    match rhex.intent.rt.as_str() {
        "lattice:genesis" => execute_lattice_genesis(rhex, lattice),
        _ => Ok(()),
    }?;
    Ok(())
}
