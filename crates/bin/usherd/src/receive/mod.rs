use anyhow::Result;
use iam::IAm;
use key::enclave::Enclave;
use lattice::{Lattice, Rhex, rhex::check::CheckStatus, usher::UsherMap};
use transform::registry::TransformRegistry;

use crate::{
    config::UsherdConfig,
    receive::{quorum::recv_two_sigs, usher::recv_one_sig},
};

pub mod append;
pub mod final_sub;
pub mod quorum;
pub mod sign_out;
pub mod usher;

pub fn receive(
    config: &UsherdConfig,
    rhex: &Rhex,
    trans_registry: &mut TransformRegistry,
    lattice: &mut Lattice,
    me: &mut IAm,
    enclave: &mut Enclave,
    _usher_map: &mut UsherMap,
) -> Result<(ReceiveStatus, Option<Vec<Rhex>>)> {
    // Process the Rhex to see where we are
    match rhex.sigs.len() {
        // No sigs = No author... this I guess could at one point offer
        // to sign as the author if it was say in your private pool of
        // nodes
        0 => Ok((ReceiveStatus::MissingSignature("author".to_string()), None)),
        // One sig = We are submitting this to this usher for validation
        // and context creation. Returns a signature payload in the outputting
        // R⬢
        1 => {
            let (status, outputs) = recv_one_sig(rhex, enclave, lattice, me, trans_registry)?;
            Ok((status, outputs))
        }
        // Two sigs = We are looking for quorum. We figure out who we are
        // in the scope and sign with the chosen key, returning a R⬢ with
        // the signature payload.
        2 => {
            let (status, output) =
                recv_two_sigs(rhex, config, lattice, enclave, me, trans_registry)?;
            Ok((status, output))
        }
        3.. => {
            // We have quorum assembled and are submitting for final
            // recording
            let scope = lattice.scopes.get(&rhex.intent.scope).unwrap();
            let policy = scope.get_policy_at(rhex.context.at.clone());
            let groups = scope.member_of_at(rhex.intent.author, rhex.context.at)?;
            let k = policy.get_k(&rhex.intent.rt, &groups)?;
            if usize::from(k) < (rhex.sigs.len() - 2) {
                anyhow::bail!("Not enough signatures");
            };
            Ok((
                ReceiveStatus::FailedValidation(CheckStatus::InvalidUsher),
                None,
            ))
            // TODO: Check sigs to make sure they're all ushers in quorum.
            // TODO: Check make sure we can still submit against the policy.
            // TODO: Make sure we are an admission point.
        }
    }
}

#[derive(PartialEq)]
pub enum ReceiveStatus {
    Success,
    FailedValidation(CheckStatus),
    MissingSignature(String),
}
