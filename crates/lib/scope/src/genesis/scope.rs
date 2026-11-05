use anyhow::Result;
use rhex::Rhex;

use crate::{Scope, membership::Membership, policy::Policy};

impl Scope {
    pub fn process_scope_genesis(&mut self, rhex: &Rhex) -> Result<()> {
        let policy = Policy::default_scope_policy();
        let time = match &rhex.context {
            None => time::get_current_time(),
            Some(context) => context.at,
        };
        self.add_membership(
            "creator".to_string(),
            vec![rhex.intent.author.clone()],
            Membership {
                issued: time,
                eff: time,
                exp: 1_000_000_000_000_000,
                by: rhex.intent.author.clone(),
            },
        )?;
        self.policy_map.push((time, policy));
        Ok(())
    }
}
