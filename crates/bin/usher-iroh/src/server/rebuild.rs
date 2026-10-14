use std::fs;

use anyhow::Result;
use iroh::PublicKey;
use lattice::{Lattice, Rhex, usher};

use crate::config::UsherIrohConfig;

/// # Lattice Rebuild
/// A core set of ushers and the current root scope are always
/// provided with the distribution, so this resets everything to
/// that bootstrapped state.
pub fn rebuild(config: &UsherIrohConfig) -> Result<Lattice> {
    let scope_path = &config.scopes;
    let usher_map_in = format!("{}usher_map.cbor", &config.bootstrap);
    let usher_map_out = format!("{}usher_map.cbor", &config.root);
    let root_scope_bootstrap = format!("{}root_scope.rchain", &config.bootstrap);

    // First, torch {path}/scopes and {path}/usher_map.cbor
    fs::remove_dir_all(&scope_path)?;
    fs::remove_file(&usher_map_out)?;
    fs::create_dir_all(&scope_path)?;

    // Load the bootstrap data
    let ushers = usher::map::disk_from(&usher_map_in);
    usher::map::disk_to(&usher_map_out, ushers);
    let root_scope = fs::read(&root_scope_bootstrap)?;
    let root_rhex: Vec<Rhex> = cbor2::from_slice(&root_scope)?;

    // Save the bootstrapped root scope to {config.scopes}/-root-.rhex
    fs::write(format!("{}-root-.rchain", &config.scopes), &root_scope)?;

    // TODO: Prolly should actually run verification across the chain
    // otherwise someone could modify the bootstrap chain and we'd
    // just happily set our head to something invalid.
    //
    // Actually, the more I think about it, it needs to run across
    // the chain anyways to build a policy for the root scope.

    // Make the root scope an object
    let mut root_scope = lattice::scope::Scope::new(
        &"".to_string(),
        PublicKey::from_bytes(&lattice::Lattice::GENESIS_KEY)?,
    )?;
    // ...and populate it
    root_scope.head = Some(root_rhex[root_rhex.len() - 1].calc_curr());

    let mut lattice = lattice::Lattice::new();
    lattice.add_scope(&root_scope.clone());
    Ok(lattice)
}
