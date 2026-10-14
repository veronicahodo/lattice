use anyhow::Result;
use iroh::PublicKey;

use crate::Scope;

#[derive(Debug, Clone, PartialEq)]
pub struct Membership {
    pub issued: u64,
    pub eff: u64,
    pub exp: u64,
    pub by: PublicKey,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MemberAccess {
    Valid,
    MembershipExpired(Membership),
    FutureMembership(Membership),
    NoEntry(PublicKey, String),
}

impl Scope {
    /// # membership_status
    /// I used to think this was a good idea, but now I feel like
    /// this is just a lot of work to keep the most current
    /// access rights.
    ///
    pub fn membership_status(&self, key: PublicKey, group: &String, time: u64) -> MemberAccess {
        let group_map = self.memberships.get(&(key, group.clone()));
        if group_map.is_none() {
            return MemberAccess::NoEntry(key, group.to_string());
        };
        let mut group_map = group_map.unwrap().clone();
        group_map.retain(|g| g.issued < time);
        let latest = &group_map.clone()[group_map.len()];
        if latest.eff < time {
            if latest.exp > time {
                return MemberAccess::Valid;
            } else {
                return MemberAccess::MembershipExpired(latest.clone());
            }
        } else {
            return MemberAccess::FutureMembership(latest.clone());
        }
    }

    /// # add_membership
    /// This updates the membership status for a set of keys in a
    /// group.
    ///
    pub fn add_membership(
        &mut self,
        group: String,
        keys: Vec<PublicKey>,
        membership: Membership,
    ) -> Result<()> {
        for key in keys {
            self.memberships
                .insert((key, group.clone()), vec![membership.clone()]);
        }
        Ok(())
    }

    /// # remove_membership
    /// Removes a set of keys from the set of groups
    ///
    pub fn remove_membership(&mut self, keys: Vec<PublicKey>, groups: Vec<String>) {
        for key in keys {
            for group in &groups {
                self.memberships.remove(&(key.clone(), group.clone()));
            }
        }
    }

    /// # member_of_at
    /// Gets a Vec of group names that the key is a member of at the
    /// given time interval.
    ///
    /// I feel like this could probably be optimized...?
    ///
    pub fn member_of_at(&self, key: PublicKey, time: u64) -> Result<Vec<String>> {
        let mut output = Vec::new();
        for ((map_key, group), memberships) in &self.memberships {
            if &key == map_key {
                let mut memberships = memberships.clone();
                memberships.retain(|m| m.issued < time);
                if memberships.len() > 0 {
                    let latest = memberships.last().unwrap();
                    if latest.eff < time && latest.exp > time {
                        output.push(group.to_string());
                    }
                }
            }
        }
        Ok(output)
    }
}
