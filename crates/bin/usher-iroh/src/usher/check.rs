use iam::IAm;
use iroh::PublicKey;
use lattice::{
    Rhex,
    rhex::{check::CheckStatus, signature::RhexSignatureType},
    scope::Scope,
};

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

/// # check_authors(rhex)
///
/// This steps each author sig in the R⬢ to make sure it is a) valid,
/// and b) has authority to add that record type to the scope.
///
pub fn check_authors(rhex: &mut Rhex, scope: &Scope) -> Vec<CheckStatus> {
    let mut output = Vec::new();
    let time = time::get_current_time();
    let mut pos: u8 = 0;

    let author_sigs = rhex.get_sigs(&RhexSignatureType::Author).unwrap();
    if author_sigs.len() < rhex.sigs.len() {
        return vec![CheckStatus::WrongStage {
            attempted: RhexSignatureType::Usher,
        }];
    }

    for sig in &rhex.sigs {
        if sig.t != RhexSignatureType::Author {
            output.push(CheckStatus::SignatureInvalid(pos));
        } else {
            let valid = sig
                .validate(&rhex.get_hash(RhexSignatureType::Author))
                .unwrap();
            if !valid {
                output.push(CheckStatus::SignatureInvalid(pos));
            } else {
                output.push(
                    scope
                        .check_rt_access_per(
                            &rhex.intent.rt,
                            &PublicKey::from_bytes(&sig.pk.key_bytes.clone().try_into().unwrap())
                                .unwrap(),
                            &Some(time),
                        )
                        .unwrap(),
                );
            }
        }
        pos += 1;
    }
    output
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
