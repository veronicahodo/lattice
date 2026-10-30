use std::collections::HashMap;

use anyhow::{Result, bail};
use iroh::{EndpointId, PublicKey};
use rhex::{Rhex, data::RhexData};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Scope;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsherStatus {
    pub issued: u64,
    pub eff: u64,
    pub exp: u64,
    pub by: PublicKey,
}

impl Scope {
    /// # ushers_at(self, time)
    ///
    /// Returns the ushers and their assignments at a specified time
    ///
    pub fn ushers_at(
        &self,
        time: u64,
    ) -> Result<HashMap<UsherRole, HashMap<PublicKey, UsherStatus>>> {
        let mut output = HashMap::new();
        for (role, members_hm) in &self.ushers {
            let mut endpoint_latest = HashMap::new();
            for member in members_hm {
                let mut statuses = member.1.clone();
                statuses.retain(|s| s.issued <= time);
                if statuses.last().is_some() {
                    let last = statuses.last().unwrap();
                    if last.eff <= time && last.exp > time {
                        endpoint_latest.insert(PublicKey::from(member.0.clone()), last.clone());
                    }
                }
            }
            output.insert(role.clone(), endpoint_latest);
        }
        Ok(output)
    }

    /// # usher_index_at(self, time)
    ///
    /// This is similar to `Scope::ushers_at` except this doesn't
    /// return the UsherAssignment with the list of public keys
    ///
    pub fn usher_index_at(&self, time: u64) -> Result<HashMap<UsherRole, Vec<EndpointId>>> {
        let mut output: HashMap<UsherRole, Vec<PublicKey>> = HashMap::new();
        let ushers = self.ushers_at(time)?;
        for role in ushers {
            let mut endpoints = Vec::new();
            for usher in role.1 {
                endpoints.push(usher.0);
            }
            output.insert(role.0, endpoints);
        }
        Ok(output)
    }

    pub fn process_usher_assign(&mut self, rhex: &Rhex) -> Result<()> {
        if rhex.intent.rt != "usher:assign".to_string() {
            anyhow::bail!("Incorrect record type")
        }
        let payload: RhexData = cbor2::from_slice(&rhex.data)?;
        // FIXME: This shits harder that a night of Everclear and Taco Bell.
        // Do not allow all these unwraps!
        let (roles, eff, exp, keys) = match payload {
            RhexData::Mixed { meta, binary } => {
                let meta = serde_json::from_slice::<Value>(&meta)?;
                let out_keys: Vec<EndpointId> = cbor2::from_slice(&binary)?;
                let roles = meta.get("roles").unwrap().clone();
                let roles = roles.as_array().unwrap();
                let eff = meta.get("eff").unwrap().clone();
                let exp = meta.get("exp").unwrap().clone();
                (
                    roles.clone(),
                    eff.as_u64().unwrap(),
                    exp.as_u64().unwrap(),
                    out_keys,
                )
            }
            _ => anyhow::bail!("Wrong data type"),
        };
        let mut roles_array = Vec::new();
        for r in roles {
            match r.as_str().unwrap() {
                "actor" => roles_array.push(UsherRole::Actor),
                "cache" => roles_array.push(UsherRole::Cache),
                "mirror" => roles_array.push(UsherRole::Mirror),
                "observer" => roles_array.push(UsherRole::Observer),
                "other" => roles_array.push(UsherRole::Other),
                "quorum" => roles_array.push(UsherRole::Quorum),
                _ => anyhow::bail!("Invalid role"),
            }
        }
        for role in roles_array {
            let branch = self.ushers.entry(role).or_insert_with(HashMap::new);
            let status = UsherStatus {
                issued: rhex.context.clone().unwrap().at,
                eff,
                exp,
                by: rhex.intent.author.clone(),
            };
            for key in &keys {
                let usher = branch.get_mut(key);
                if usher.is_some() {
                    usher.unwrap().push(status.clone());
                } else {
                    branch.insert(key.clone(), vec![status.clone()]);
                }
            }
        }
        Ok(())
    }

    pub fn process_usher_revoke(&mut self, rhex: &Rhex) -> Result<()> {
        let data: RhexData = cbor2::from_slice(&rhex.data)?;
        let (roles, keys) = match data {
            RhexData::Mixed { meta, binary } => {
                let mut json_meta: Value = serde_json::from_slice(&meta)?;
                let roles_val = json_meta
                    .get_mut("roles")
                    .map(|v| v.take())
                    .filter(|v| v.is_array())
                    .ok_or_else(|| anyhow::anyhow!("No roles to revoke or not an array"))?;
                let roles: Vec<String> = serde_json::from_value(roles_val)?;

                let keys: Vec<PublicKey> = cbor2::from_slice(&binary)?;
                (roles, keys)
            }
            _ => anyhow::bail!("Wrong data type"),
        };

        for role in roles {
            let lookup = match role.as_str() {
                "actor" => UsherRole::Actor,
                "cache" => UsherRole::Cache,
                "mirror" => UsherRole::Mirror,
                "observer" => UsherRole::Observer,
                "other" => UsherRole::Other,
                "quorum" => UsherRole::Quorum,
                _ => bail!("Invalid usher role: {}", role.as_str()),
            };

            let list = self.ushers.get_mut(&lookup);
            let rhex_time = rhex.context.clone().unwrap().at;
            let status = UsherStatus {
                issued: rhex_time,
                eff: rhex_time,
                exp: 1_000_000_000_000_000,
                by: rhex.intent.author.clone(),
            };

            if let Some(map) = list {
                for (key, status_vec) in map.iter_mut() {
                    if keys.contains(key) {
                        status_vec.push(status.clone());
                    }
                }
            };
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsherAssignment {
    pub issued: u64,
    pub priority: u8,
    pub roles: Vec<UsherRole>,
    pub eff: u64,
    pub exp: u64,
    pub by: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum UsherRole {
    Actor,
    Mirror,
    Cache,
    Quorum,
    Observer,
    Other,
}
