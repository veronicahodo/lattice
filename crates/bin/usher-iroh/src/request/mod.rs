use anyhow::{Result, bail};
use key::enclave::Enclave;
use lattice::{
    Lattice, Rhex,
    rhex::{
        data::RhexData,
        intent::RhexIntent,
        signature::{AgileKey, RhexSigAlgo, RhexSignature, RhexSignatureType},
    },
    scope::Scope,
};

use crate::config::UsherIrohConfig;

pub fn process_request(
    lattice: &Lattice,
    rhex: &Rhex,
    config: &UsherIrohConfig,
    enclave: &Enclave,
) -> Result<Vec<Rhex>> {
    let scope = lattice.scopes.get(&rhex.intent.scope);
    let scope_filename = if scope.is_none() {
        bail!("We don't have this scope")
    } else {
        if scope.unwrap().name.as_str() == "" {
            "-root-/".to_string()
        } else {
            format!("{}/", scope.unwrap().name)
        }
    };
    let scope = scope.unwrap();
    Ok(match rhex.intent.rt.as_str() {
        "request:rhex" => request_rhex(&format!("{}{}", &config.scopes, &scope_filename), &scope)?,
        "request:policy" => vec![request_policy(scope, rhex, enclave)?],
        _ => vec![],
    })
}

fn request_rhex(path: &String, scope: &Scope) -> Result<Vec<Rhex>> {
    // FIXME: This should really process filters provided by the
    // submitted R⬢
    Ok(scope.slurp_scope(path)?)
}

fn request_policy(scope: &Scope, rhex: &Rhex, enclave: &Enclave) -> Result<Rhex> {
    let time = time::get_current_time();
    let mut intent = RhexIntent::new();
    intent.scope = scope.name.clone();
    intent.author = rhex.intent.usher.clone();
    // Actually should be the sender's key, not the authors.
    // FIXME: later
    intent.usher = rhex.intent.author.clone();
    intent.schema = Some("rhex://schema.scope.policy/@0".to_string());
    intent.rt = "respone:policy".to_string();
    let data = RhexData::Binary(scope.get_policy_at(&time).to_vec()?).to_vec()?;
    intent.data_hash = Some(blake3::hash(&data).as_bytes().clone());
    let rhex = Rhex::build_with(
        &intent,
        &None,
        &vec![RhexSignature {
            pk: AgileKey {
                algo: RhexSigAlgo::Ed25519,
                key_bytes: rhex.intent.usher.to_vec(),
            },
            sig: enclave
                .sign(
                    &rhex.intent.usher,
                    &rhex.get_hash(RhexSignatureType::Author),
                )?
                .to_vec(),
            t: RhexSignatureType::Author,
        }],
        &Some(data),
    );
    Ok(rhex)
}
