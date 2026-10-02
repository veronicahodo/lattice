use std::path::PathBuf;

use anyhow::Result;
use rhex::{
    Rhex,
    check::CheckStatus,
    signature::RhexSignatureType::{Observer, Quorum},
};

use crate::{Scope, ushers::UsherRole};

impl Scope {
    /// # check_same_scope
    /// Checks to make sure this Rhex is for this Scope.
    ///
    pub fn check_same_scope(&self, rhex: &Rhex) -> Result<CheckStatus> {
        // See if this record is even for this scope
        if self.name != rhex.intent.scope {
            return Ok(CheckStatus::NotThisScope {
                presented: rhex.intent.scope.to_string(),
                expected: self.name.to_string(),
            });
        };
        Ok(CheckStatus::Success)
    }

    /// # check_prev
    /// Makes sure the presented `intent.prev` matches the scope head,
    /// or `curr` of the last Rhex in the Scope
    pub fn check_prev(&self, rhex: &Rhex) -> Result<CheckStatus> {
        // Does previous match head?
        if rhex.intent.prev != self.head {
            return Ok(CheckStatus::PrevHashMismatch {
                presented: rhex.intent.prev,
                expected: self.head,
            });
        };
        Ok(CheckStatus::Success)
    }

    /// # check_nonce
    /// Checks to see if the `intent.nonce` has been reused for this
    /// Scope
    ///
    pub fn check_nonce(self, nonce: [u8; 32], path: &PathBuf) -> Result<CheckStatus> {
        // skim for nonce reuse
        if self.check_nonce_reused(nonce, path).unwrap() {
            return Ok(CheckStatus::NonceReused);
        };
        Ok(CheckStatus::Success)
    }

    /// # check_rt_access
    /// check to make sure we can append this record as the author
    /// at this coordinate.
    ///    
    pub fn check_rt_access(&self, rhex: &Rhex) -> Result<CheckStatus> {
        let groups = self.member_of_at(rhex.intent.author.clone(), rhex.context.at.clone())?;
        if groups.len() == 0 {
            return Ok(CheckStatus::AccessDenied);
        }
        let policy = self.get_policy_at(rhex.context.at.clone());
        let submittable = policy.can_submit(&rhex.intent.rt, &groups);
        if !submittable {
            return Ok(CheckStatus::RtNotAllowed);
        }
        Ok(CheckStatus::Success)
    }

    /// # check_quorum
    ///
    /// Checks the quorum count, and then iterates over the sigs to
    /// make sure they are valid for this submission
    ///
    pub fn check_quorum_and_observers(&self, rhex: &Rhex, time: &u64) -> Result<CheckStatus> {
        let all_ushers = self.ushers_by_role(time.clone())?;
        let mut quorum_ushers = Vec::new();
        for usher in all_ushers {
            if usher.0 == UsherRole::Quorum {
                quorum_ushers.push(usher.1);
            }
        }
        let policy = self.get_policy_at(time.clone());
        let groups = self.member_of_at(rhex.intent.author.clone(), time.clone())?;

        // Get the window so we know if quorum sigs are within it
        let window = policy.get_window(&rhex.intent.rt, &groups)?;

        // Make separate vecs for the quorum and observer signatures
        let mut pos: u8 = 0;
        let mut quorum_sigs = Vec::new();
        let mut observer_sigs = Vec::new();
        for sig in rhex.sigs.iter() {
            match sig.t {
                Quorum(time) | Observer(time) => {
                    if time < rhex.context.at {
                        return Ok(CheckStatus::SignatureInvalid(pos));
                    }
                    if (time - rhex.context.at) < window {
                        match sig.t {
                            Quorum(_) => {
                                if quorum_ushers
                                    .contains(&sig.pk.key_bytes.clone().try_into().unwrap())
                                {
                                    quorum_sigs.push(sig.clone());
                                }
                            }
                            Observer(_) => observer_sigs.push(sig.clone()),
                            _ => continue,
                        }
                    } else {
                        return Ok(CheckStatus::SignatureNotInWindow(pos));
                    }
                }
                _ => continue,
            }
            pos += 1;
        }

        // Check to see if the number of sigs even matches what we
        // provided in the R⬢
        let k = policy.get_k(&rhex.intent.rt, &groups)? as usize;
        let o = policy.get_o(&rhex.intent.rt, &groups)? as usize;
        let quorum_count = quorum_sigs.len();
        let observer_count = quorum_sigs.len() + observer_sigs.len();
        if quorum_count < k {
            return Ok(CheckStatus::QuorumCountUnder {
                provided: quorum_count,
                k,
            });
        }
        if observer_count < o {
            return Ok(CheckStatus::ObserverCountUnder {
                provided: observer_count,
                o,
            });
        }

        Ok(CheckStatus::Success)
    }

    /// # check_usher
    /// Currently just checks to see if the Usher is listed as valid
    /// in the Scope. Future this may actually do more.
    ///
    pub fn check_usher(&self, rhex: &Rhex) -> Result<CheckStatus> {
        // see if usher specified is available.
        let mut current_usher = self.ushers_at(rhex.context.at.clone())?;
        current_usher.retain(|u| u.0 == rhex.intent.usher);
        if current_usher.len() == 0 {
            return Ok(CheckStatus::InvalidUsher);
        }
        Ok(CheckStatus::Success)
    }

    /// # check_time_reversal
    /// Basically just makes sure we're not trying to creep in an
    /// 'earlier' Rhex
    ///
    pub fn check_time_reversal(self, rhex: &Rhex) -> Result<CheckStatus> {
        // Have we gone backwards in time?
        let latest = self.latest_time();
        if latest > rhex.context.at {
            return Ok(CheckStatus::TimeReversal {
                presented: rhex.context.at.clone(),
                prev: latest,
            });
        }
        Ok(CheckStatus::Success)
    }

    /// # full_check
    /// Checks all the possible common errors. This basically checks
    /// each Rhex field for validity. (Minus nonce reuse, because that
    /// actually calls for pulling the whole chain in, which is not
    /// something we want on quick checks)
    pub fn full_check(&self, rhex: &Rhex) -> Result<Vec<CheckStatus>> {
        let mut outputs = Vec::new();
        outputs.push(rhex.check_data_size()?);
        outputs.push(rhex.check_schema()?);
        outputs.push(self.check_same_scope(rhex)?);
        outputs.push(self.check_prev(rhex)?);
        outputs.push(self.check_rt_access(rhex)?);
        outputs.push(self.check_usher(rhex)?);
        for i in 0..rhex.sigs.len() {
            outputs.push(rhex.check_sig(i)?);
        }

        outputs.retain(|o| *o != CheckStatus::Success);

        if outputs.len() > 0 {
            Ok(outputs)
        } else {
            Ok(vec![CheckStatus::Success])
        }
    }

    /// # final_check
    /// This is similar to `self.full_check` except this also checks
    /// `rhex.curr` to see if it's correct and the overall record size
    ///
    pub fn final_check(&self, rhex: &Rhex, time: &u64) -> Result<Vec<CheckStatus>> {
        let mut outputs = Vec::new();
        let mut full = self.full_check(rhex)?;
        outputs.append(&mut full);
        outputs.push(self.check_quorum_and_observers(rhex, time)?);
        outputs.push(rhex.check_curr_hash()?);
        outputs.push(rhex.check_total_size()?);

        outputs.retain(|o| *o != CheckStatus::Success);

        if outputs.len() > 0 {
            Ok(outputs)
        } else {
            Ok(vec![CheckStatus::Success])
        }
    }
}
