use anyhow::{Ok, Result};
use rhex::Rhex;

use crate::{Scope, policy::Policy};

impl Scope {
    /// # process_scope_changes(self, rhex)
    ///
    /// This takes a R⬢ and sees if it makes any modifications to
    /// the scope's access, ushers, and status.
    ///
    pub fn process_scope_changes(&mut self, rhex: &Rhex) -> Result<ScopeUpdateStatus> {
        let status = match rhex.intent.rt.as_str() {
            "key:grant" => self.process_key_grant(rhex),
            "key:revoke" => self.process_key_revoke(rhex),
            "usher:assign" => self.process_usher_assign(rhex),
            "usher:revoke" => self.process_usher_revoke(rhex),
            "policy:set" => {
                let policy = Policy::process_policy_set(rhex)?;
                self.policy_map
                    .push((rhex.context.clone().unwrap().at, policy));
                Ok(())
            }
            _ => return Ok(ScopeUpdateStatus::NotUsed),
        };
        Ok(ScopeUpdateStatus::from(status))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScopeUpdateStatus {
    Success,
    NotUsed,
    Failed(String),
}

impl<T> From<anyhow::Result<T>> for ScopeUpdateStatus {
    fn from(res: anyhow::Result<T>) -> Self {
        match res {
            std::result::Result::Ok(_) => ScopeUpdateStatus::Success,
            std::result::Result::Err(err) => ScopeUpdateStatus::Failed(err.to_string()),
        }
    }
}
