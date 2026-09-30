use std::path::Path;

use anyhow::Result;
use iam::IAm;
use key::enclave::Enclave;
use lattice::Lattice;
use transform::registry::TransformRegistry;

use crate::{config::UsherdConfig, rebuild};

/// # init_resource<T, FInit, FLoad>(...)
///
/// This loads a resource from file, creating it if necessary
///
pub fn init_resource<T, FInit, FLoad>(
    path: &str,
    label: &str,
    verbose: bool,
    rebuild_fn: FInit,
    load_fn: FLoad,
) -> Result<T>
where
    FInit: FnOnce() -> Result<()>,
    FLoad: FnOnce() -> Result<T>,
{
    if verbose {
        print!("Loading {}...", label);
    }

    if !Path::new(path).exists() {
        if verbose {
            print!(" rebuilding...");
        }
        rebuild_fn()?;
    }

    let resource = load_fn()?;
    if verbose {
        println!(" done!");
    }

    Ok(resource)
}

/// # init_lattice(config, registry)
///
/// This sets up the lattice as seen by this usher. It walks through
/// scopes and verifies everything is hunky dory before we kick off
/// the listener
///
pub fn init_lattice(config: &UsherdConfig, registry: &TransformRegistry) -> Result<Lattice> {
    let lattice = if config.rebuild {
        println!("Starting rebuild from bootstrap at {}...", config.bootstrap);
        rebuild::rebuild(config).unwrap()
    } else {
        let mut building_lattice = Lattice::new();
        if !config.verbose {
            print!("Loading scopes");
        }
        building_lattice.build_from_disk(&config.scopes, registry, config.verbose)?;
        building_lattice
    };
    println!("🧬 Lattice is live! {} scopes loaded", lattice.scopes.len());
    Ok(lattice)
}

/// # init_enclave(config, iam)
///
/// This takes each entry in the IAm that is set to Local and makes
/// sure we can in fact sign as those keys.
///
pub fn init_enclave(config: &UsherdConfig, iam: &IAm) -> Result<Enclave> {
    if config.verbose {
        print!("Enclave loading...");
    }

    let mut enclave = Enclave::new(Some(config.enclave.clone()));
    enclave.populate()?;

    if config.verbose {
        println!(" done! Loaded {} keys", enclave.keys.len());
        print!("Checking I Am against Enclave...");
    }

    enclave.check_map(iam.get_local()?)?;

    if config.verbose {
        println!(" done!");
    }

    Ok(enclave)
}
