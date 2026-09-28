use anyhow::{Result, anyhow};
use rhex::{Rhex, check::CheckStatus};
use transform::registry::TransformRegistry;

use crate::Scope;

impl Scope {
    pub fn from_chain(
        rhex: &Vec<Rhex>,
        creator: [u8; 32],
        _trans: &TransformRegistry,
    ) -> Result<Self> {
        let mut scope = Scope::new(&rhex[0].intent.scope, creator.clone());
        for r in rhex {
            let check = scope.final_check(r, &r.context.at)?;
            if check.len() != 1 || check[0] != CheckStatus::Success {
                return Err(anyhow!("Failed checks"));
            }

            match r.intent.rt.as_str() {
                "usher:assign" => scope.process_usher_assign(r)?,
                "usher:revoke" => scope.process_usher_revoke(r)?,
                "key:grant" => scope.process_key_grant(r)?,
                "key:revoke" => scope.process_key_revoke(r)?,
                _ => continue,
            }
        }
        Ok(scope)
    }
}
