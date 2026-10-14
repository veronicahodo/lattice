use anyhow::Result;
use iroh::PublicKey;
use lattice::scope::{Scope, ushers::UsherRole};

use crate::IAm;

impl IAm {
    /// # pick(self, scope, time, role)
    /// Ok, so I don't like this. We pick a key based on which one
    /// we have with the highest priority. This will work in most
    /// cases but if someone holds multiple keys on the same scope
    /// it will always pick the one with the lowest priority score.
    ///
    /// This can obviously be abused to front run, along with making
    /// any key past the first one inaccessable.
    ///
    pub fn pick(&self, scope: &Scope, time: u64, role: UsherRole) -> Result<Option<PublicKey>> {
        let roles = scope.ushers_at(time)?;
        let mut master = Vec::new();
        let ushers = roles.get(&role);
        for key in ushers.unwrap() {
            let pubkey = self.entries.get(key.0);
            if pubkey.is_some() {
                master.push(key.0.clone());
            }
        }
        if master.len() > 0 {
            Ok(Some(master[0].clone()))
        } else {
            Ok(None)
        }
    }
}
