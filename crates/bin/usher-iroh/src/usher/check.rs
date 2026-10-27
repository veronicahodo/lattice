use iam::IAm;
use iroh::PublicKey;
use lattice::{Rhex, rhex::check::CheckStatus, scope::Scope};

/// # am_i(iam, key)
///
/// Checks to see if we are in the IAm table and returns the appropriate
/// CheckStatus. Yes, this is a dumb wrapper.
///
pub fn am_i(iam: &IAm, key: &PublicKey) -> Vec<CheckStatus> {
    let am = iam.am_i(key).unwrap();
    if !am {
        vec![CheckStatus::NotMe]
    } else {
        vec![CheckStatus::Success]
    }
}

/// # control_scope(scope)
///
/// Checks to see if we control the scope. If this doesn't fail we
/// can safely unwrap the scope after.
///
pub fn control_scope(scope: Option<&Scope>) -> Vec<CheckStatus> {
    if scope.is_none() {
        vec![CheckStatus::NotUsherForThisScope]
    } else {
        vec![CheckStatus::Success]
    }
}

/// # get_ko(lattice, rhex)
///
/// Pulls the scope, gets the policy and figures out what K (quorum)
/// and O (observer) counts are required
///
pub fn get_ko(scope: &Scope, rhex: &Rhex) -> Result<(u16, u16), String> {
    let policy = scope.get_policy_at(&time::get_current_time());
    let groups = scope
        .member_of_at(&rhex.intent.author, &time::get_current_time())
        .unwrap_or(vec![]);
    let k = policy.get_k(&rhex.intent.rt, &groups).unwrap_or(9999);
    let o = policy.get_o(&rhex.intent.rt, &groups).unwrap_or(9999);
    Ok((k, o))
}
