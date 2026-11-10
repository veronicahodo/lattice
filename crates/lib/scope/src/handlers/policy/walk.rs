use anyhow::Result;
use rhex::Rhex;

use crate::{Scope, handlers};

pub fn walk_policy_set(rhex: &Rhex, scope: &mut Scope) -> Result<()> {
    let policy_set = handlers::policy::verify::verify_policy_set(rhex)?;
    let time = if rhex.context.is_some() {
        rhex.context.clone().unwrap().at
    } else {
        time::get_current_time()
    };
    scope.policy_map.push((time.clone(), policy_set));
    Ok(())
}
