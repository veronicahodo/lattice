use anyhow::{Result, anyhow, bail};
use rhex::{Rhex, data::RhexData};
use serde::{Deserialize, Serialize};

use crate::rule::Rule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub desc: String,
    pub rules: Vec<Rule>,
    pub eff: u64,
    pub exp: u64,
    pub tags: Vec<String>,
    pub issued: u64,
}

impl Policy {
    pub fn new(desc: String) -> Self {
        Self {
            desc,
            rules: vec![],
            eff: 0,
            exp: 0,
            tags: vec![],
            issued: 0,
        }
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn add_tag(&mut self, tag: String) {
        self.tags.push(tag);
    }

    /// # check
    /// Checks to see if the rule for that `rt` (record type) exists
    /// and if a group is specified, that group is a member of that
    /// rule.
    ///
    pub fn check(&self, rt: &String, group: &Option<String>) -> bool {
        let prefix = match rt.ends_with(":*") {
            true => rt,
            false => &rt.split(":").next().unwrap().to_string(),
        };
        let mut has = false;
        let mut contains = false;
        for rule in &self.rules {
            if rule.rt.contains(rt) || rule.rt.contains(prefix) {
                has = true;
                if (group.is_some() && rule.append.contains(&group.clone().unwrap()))
                    || group.is_none()
                {
                    contains = true;
                }
                break;
            }
        }

        has && contains
    }

    /// This is the initial policy for the whole lattice.
    /// It dictates my key is the only one to start everything.
    pub fn default_lattice_policy() -> Self {
        Self {
            desc: "Lattice Default Policy".to_string(),
            rules: vec![Rule {
                append: vec!["world_line_zero".to_string()],
                k: 1,
                o: 1,
                quorum: vec!["world_line_zero".to_string()],
                delay: 1_000_000_000,
                rt: vec!["lattice:genesis".to_string()],
                window: 1_000,
            }],
            eff: 0,
            exp: 1_000_000_000_000,
            tags: vec!["lattice".to_string(), "bootstrap".to_string()],
            issued: 0,
        }
    }

    /// This is the initial policy for any scope below the root ("").
    /// The key used in the scope:request is automatically assigned
    /// to "creator" group.
    pub fn default_scope_policy() -> Self {
        Self {
            desc: "Scope Default Policy".to_string(),
            rules: vec![Rule {
                append: vec!["creator".to_string()],
                k: 1,
                o: 1,
                quorum: vec!["creator".to_string()],
                delay: 1_000_000_000_000,
                rt: vec!["scope:genesis".to_string()],
                window: 1_000,
            }],
            eff: 0,
            exp: 1_000_000_000_000,
            tags: vec![],
            issued: 0,
        }
    }

    /// # is_dangerous
    /// This basically just does a quick check to make sure this
    /// policy will allow a) updates via new `policy:set` and
    /// b) allows for key management (either key:* or key:grant/revoke)
    /// If the policy has the tag "dangerous" then we just return true
    /// because we assume the person is being deliberate. Or dumb.
    pub fn is_dangerous(&self) -> bool {
        if self.tags.contains(&"dangerous".to_string()) {
            return false;
        };

        let has_policy_set = self.check(&"policy:set".to_string(), &None);
        let has_key_grant = self.check(&"key:grant".to_string(), &None);
        let has_key_revoke = self.check(&"key:revoke".to_string(), &None);

        !(has_policy_set && has_key_grant && has_key_revoke)
    }

    /// # build_safe_policy
    /// Why did I make this? Because I code drunk a lot, and like...
    /// I'm trying to build in a parachute for me to like, I dunno,
    /// pull that makes it so I can't do super dumb ass shit™️
    pub fn build_safe_policy(&mut self, group: &String) -> bool {
        let has_policy_set = self.check(&"policy:set".to_string(), &Some(group.clone()));
        let has_key_grant = self.check(&"key:grant".to_string(), &Some(group.clone()));
        let has_key_revoke = self.check(&"key:revoke".to_string(), &Some(group.clone()));

        if !has_policy_set {
            let policy_set_rule = Rule {
                append: vec![group.clone()],
                k: 1,
                o: 1,
                quorum: vec![group.clone()],
                delay: 1_000_000_000,
                rt: vec!["policy:*".to_string()],
                window: 1_000,
            };

            self.rules.push(policy_set_rule);
        }
        if !has_key_grant && !has_key_revoke {
            let key_rule = Rule {
                append: vec![group.clone()],
                k: 1,
                o: 1,
                quorum: vec![group.clone()],
                delay: 1_000_000_000,
                rt: vec!["key:*".to_string()],
                window: 1_000,
            };
            self.rules.push(key_rule);
        }
        !self.is_dangerous()
    }

    pub fn get_k(&self, rt: &String, groups: &Vec<String>) -> Result<u16> {
        for rule in &self.rules {
            if rule.valid(rt, groups)? {
                return Ok(rule.k);
            }
        }
        Err(anyhow::anyhow!("Not a valid rule axis"))
    }

    /// # get_o(self, rt, groups)
    ///
    /// Gets the `o` value from the rules
    ///
    pub fn get_o(&self, rt: &String, groups: &Vec<String>) -> Result<u16> {
        for rule in &self.rules {
            if rule.valid(rt, groups)? {
                return Ok(rule.o);
            }
        }
        Err(anyhow!("Not a valid rule axis"))
    }

    pub fn get_window(&self, rt: &String, groups: &Vec<String>) -> anyhow::Result<u64> {
        for rule in &self.rules {
            if rule.valid(rt, groups)? {
                return Ok(rule.window);
            }
        }
        Err(anyhow::anyhow!("Not a valid rule axis"))
    }

    /// # process_policy_set(rhex)
    ///
    /// This takes a R⬢ and pulls the data out to reconstitue back
    /// into a policy that we can just set.
    pub fn process_policy_set(rhex: &Rhex) -> Result<Self> {
        let data: RhexData = cbor2::from_slice(&rhex.data)?;
        let policy: Policy = match data {
            RhexData::Binary(b) => cbor2::from_slice(&b)?,
            _ => bail!("Invalid RhexData type on policy:set"),
        };
        Ok(policy)
    }
}
