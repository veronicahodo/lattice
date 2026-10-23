use std::path::PathBuf;

use anyhow::Result;
use rhex::Rhex;

use crate::Scope;

impl Scope {
    /// # check_nonce_reused
    /// Quickly skims the chain to see if this nonce has been used
    /// before. Returns `true` if it already exists in the scope
    ///
    pub fn check_nonce_reused(&self, nonce: [u8; 16], scope_path: &PathBuf) -> Result<bool> {
        let rhex = Rhex::chain_from_disk(scope_path)?;
        let used = rhex
            .iter()
            .filter(|r| r.intent.nonce == nonce.clone())
            .collect::<Vec<&Rhex>>()
            .len()
            > 0;
        Ok(used)
    }

    /// # latest_time
    /// Returns the latest `context.at` of the last R⬢ in the scope.
    /// FIXME: This just returns `self.updated` which SHOULD be set to
    /// the latest record, unless somehow there was a modification
    /// outside the chain, which we don't allow, but it still feels wrong
    /// blindly pulling here
    ///
    pub fn latest_time(&self) -> u64 {
        self.updated
    }
}
