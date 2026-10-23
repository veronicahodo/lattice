use lattice::{Rhex, rhex::check::CheckStatus, scope::Scope};

pub fn check_pre_context(scope: &Scope, rhex: &mut Rhex) -> Vec<CheckStatus> {
    let status = scope.full_check(rhex).unwrap();

    status
}
