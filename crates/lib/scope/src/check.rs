use std::path::PathBuf;

use anyhow::Result;
use iroh::PublicKey;
use rhex::{
    Rhex,
    check::CheckStatus,
    signature::RhexSignatureType::{self, Observer, Quorum},
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
    pub fn check_nonce(self, nonce: [u8; 16], path: &PathBuf) -> Result<CheckStatus> {
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
        let groups = self.member_of_at(&rhex.intent.author, &rhex.context.clone().unwrap().at)?;
        if groups.len() == 0 {
            return Ok(CheckStatus::AccessDenied);
        }
        let time = if let Some(context) = &rhex.context {
            context.at
        } else {
            time::get_current_time()
        };
        let policy = self.get_policy_at(&time);
        let submittable = policy.can_submit(&rhex.intent.rt, &groups);
        if !submittable {
            return Ok(CheckStatus::RtNotAllowed);
        }
        Ok(CheckStatus::Success)
    }

    pub fn check_rt_access_per(
        &self,
        rt: &String,
        key: &PublicKey,
        time: &Option<u64>,
    ) -> Result<CheckStatus> {
        let time = if let Some(time) = time {
            time.clone()
        } else {
            time::get_current_time()
        };
        let groups = self.member_of_at(key, &time)?;
        if groups.len() == 0 {
            return Ok(CheckStatus::AccessDenied);
        }
        let policy = self.get_policy_at(&time);
        let submittable = policy.can_submit(rt, &groups);
        if !submittable {
            return Ok(CheckStatus::RtNotAllowed);
        }
        Ok(CheckStatus::Success)
    }

    /// # check_authors(self, rhex, time)
    ///
    /// Checks to make sure each author has valid access to the scope
    /// at that time
    ///
    pub fn check_authors(&self, rhex: &Rhex, time: &u64) -> Result<Vec<CheckStatus>> {
        let mut output = Vec::new();
        let mut pos: u8 = 0;

        let author_sigs = rhex.get_sigs(&RhexSignatureType::Author).unwrap();
        if author_sigs.len() < rhex.sigs.len() {
            return Ok(vec![CheckStatus::WrongStage {
                attempted: RhexSignatureType::Usher,
            }]);
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
                        self.check_rt_access_per(
                            &rhex.intent.rt,
                            &PublicKey::from_bytes(&sig.pk.key_bytes.clone().try_into().unwrap())
                                .unwrap(),
                            &Some(time.clone()),
                        )
                        .unwrap(),
                    );
                }
            }
            pos += 1;
        }
        Ok(output)
    }

    /// # check_quorum
    ///
    /// Checks the quorum count, and then iterates over the sigs to
    /// make sure they are valid for this submission
    ///
    pub fn check_quorum_and_observers(&self, rhex: &Rhex, time: &u64) -> Result<CheckStatus> {
        let all_ushers = self.usher_index_at(time.clone())?;
        let quorum_ushers = all_ushers.get(&UsherRole::Quorum);
        let quorum_ushers = if quorum_ushers.is_some() {
            quorum_ushers.unwrap()
        } else {
            &vec![]
        };
        let policy = self.get_policy_at(time);
        let groups = self.member_of_at(&rhex.intent.author, &time)?;

        // Get the window so we know if quorum sigs are within it
        let window = policy.get_window(&rhex.intent.rt, &groups)?;

        // Make separate vecs for the quorum and observer signatures
        let mut pos: u8 = 0;
        let mut quorum_sigs = Vec::new();
        let mut observer_sigs = Vec::new();
        for sig in rhex.sigs.iter() {
            match sig.t {
                Quorum(time) | Observer(time) => {
                    let rhex_time = rhex.context.clone().unwrap().at;
                    if time < rhex_time {
                        return Ok(CheckStatus::SignatureInvalid(pos));
                    }
                    if (time - rhex_time) < window {
                        match sig.t {
                            Quorum(_) => {
                                if quorum_ushers.contains(&PublicKey::from_bytes(
                                    &sig.pk.key_bytes.clone().try_into().unwrap(),
                                )?) {
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
        let current_usher = self.ushers_at(rhex.context.clone().unwrap().at)?;
        // We can only get away with a blind unwrap here because a scope
        // without actors is an invalid scope and cannot be created
        let usher = current_usher
            .get(&UsherRole::Actor)
            .unwrap()
            .get(&rhex.intent.usher);
        if usher.is_none() {
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
        if latest > rhex.context.clone().unwrap().at {
            return Ok(CheckStatus::TimeReversal {
                presented: rhex.context.clone().unwrap().at,
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
    pub fn full_check(&self, rhex: &mut Rhex) -> Result<Vec<CheckStatus>> {
        let mut outputs = Vec::new();
        // outputs.push(rhex.check_data_size()?);
        outputs.push(rhex.check_schema()?);
        outputs.push(self.check_same_scope(rhex)?);
        // FIXME: check_priv needs to be ok with certain requests
        // having no prev set.
        outputs.push(self.check_prev(rhex)?);
        // Changed this up... I think it's redundant with another
        // function but I'm pressed for time so we're just gonna do it
        // here:
        let time = time::get_current_time();
        outputs.extend(self.check_authors(rhex, &time)?);
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
    pub fn final_check(&self, rhex: &mut Rhex, time: &u64) -> Result<Vec<CheckStatus>> {
        let mut outputs = Vec::new();
        let full = self.full_check(rhex)?;
        outputs.extend(full);
        outputs.push(self.check_quorum_and_observers(rhex, time)?);
        outputs.push(rhex.check_curr_hash()?);
        // FIXME: It *SHOULD* check the total size, but since data
        // is only presented to the usher, we can't use this for
        // quorum otherwise.
        //outputs.push(rhex.check_total_size()?);

        outputs.retain(|o| *o != CheckStatus::Success);

        if outputs.len() > 0 {
            Ok(outputs)
        } else {
            Ok(vec![CheckStatus::Success])
        }
    }
}
