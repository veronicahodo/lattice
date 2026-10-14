use anyhow::Result;
use iam::IAm;
use key::enclave::Enclave;
use lattice::Lattice;

use crate::config::UsherIrohConfig;

pub mod appstate;
pub mod connection;
pub mod init;
pub mod rebuild;

pub fn run(config: &UsherIrohConfig) -> Result<()> {
    // Initialize
    let mut enclave = Enclave::new(Some(&config.enclave));
    enclave.populate()?;
    let mut iam = IAm::disk_from(&config.i_am)?;
    let mut lattice = Lattice::new();

    // Check our IAm map to see if all the local keys are in the
    // enclave
    enclave.check_map(iam.get_local()?)?;

    // Set up listener

    Ok(())
}
