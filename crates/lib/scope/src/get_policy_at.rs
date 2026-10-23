use crate::{Scope, policy::Policy};

impl Scope {
    /// # get_policy_at
    /// This returns the current policy at the specified micromark.
    /// This is necessary because a scope accumilates future policies
    /// and we need to know which one is effective at a certain point.
    ///
    pub fn get_policy_at(&self, time: &u64) -> Policy {
        let mut latest_policy_time = 0;
        let mut latest_policy = Policy::new("".to_string());
        for (issued, policy) in &self.policy_map {
            if time > issued {
                if &latest_policy_time < issued {
                    latest_policy_time = issued.clone();
                    latest_policy = policy.clone();
                }
            }
        }
        latest_policy
    }
}
