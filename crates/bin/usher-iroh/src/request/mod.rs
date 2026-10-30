use anyhow::{Result, bail};
use lattice::{Lattice, Rhex, scope::Scope};

use crate::config::UsherIrohConfig;

pub fn process_request(
    lattice: &Lattice,
    rhex: &Rhex,
    config: &UsherIrohConfig,
) -> Result<Vec<Rhex>> {
    let scope = lattice.scopes.get(&rhex.intent.scope);
    let scope_filename = if scope.is_none() {
        bail!("We don't have this scope")
    } else {
        if scope.unwrap().name.as_str() == "" {
            "-root-".to_string()
        } else {
            scope.unwrap().name.clone()
        }
    };
    let scope = scope.unwrap();
    Ok(match rhex.intent.rt.as_str() {
        "request:rhex" => request_rhex(
            &format!("{}{}.rchain", &config.scopes, &scope_filename),
            &scope,
        )?,
        _ => vec![],
    })
}

fn request_rhex(path: &String, scope: &Scope) -> Result<Vec<Rhex>> {
    // FIXME: This should really process filters provided by the
    // submitted R⬢
    Ok(scope.slurp_scope(path)?)
}
