use anyhow::{Ok, Result};
use iroh::PublicKey;
use rhex::{Rhex, data::RhexData};
use serde_json::Value;

use crate::{Scope, membership::Membership};

impl Scope {
    pub fn process_key_grant(&mut self, rhex: &Rhex) -> Result<()> {
        let data: RhexData = cbor2::from_slice(&rhex.data)?;
        let (meta, keys) = match data {
            RhexData::Mixed { meta, binary } => {
                let meta_breakout: Value = serde_json::from_slice(&meta)?;
                let keys: Vec<PublicKey> = cbor2::from_slice(&binary)?;
                (meta_breakout, keys)
            }
            _ => anyhow::bail!("Wrong data type"),
        };

        // Get names
        let name_value = meta.get("names");
        let mut names = Vec::new();
        if name_value.is_some() {
            names = util::value_to_string_arr(name_value.unwrap().clone())?;
        }
        if names.len() > 0 && names.len() != keys.len() {
            anyhow::bail!("Must have the same number of names as keys");
        }

        // Get groups
        let group_value = meta.get("groups");
        let groups = if group_value.is_some() {
            let arr = util::value_to_string_arr(group_value.unwrap().clone())?;
            if arr.len() == 0 {
                anyhow::bail!("No groups specified")
            }
            arr
        } else {
            anyhow::bail!("No groups specified")
        };

        // Set eff/exp
        let eff = meta
            .get("eff")
            .and_then(|v| v.as_u64())
            .unwrap_or_else(|| 0);
        let exp = meta
            .get("exp")
            .and_then(|v| v.as_u64())
            .unwrap_or_else(|| 1_000_000_000_000_000);

        for group in groups {
            self.add_membership(
                group,
                keys.clone(),
                Membership {
                    issued: rhex.context.at.clone(),
                    eff: eff.clone(),
                    exp: exp.clone(),
                    by: rhex.intent.author.clone(),
                },
            )?;
        }
        Ok(())
    }

    pub fn process_key_revoke(&mut self, rhex: &Rhex) -> Result<()> {
        let data: RhexData = cbor2::from_slice(&rhex.data)?;
        let (meta, keys) = match &data {
            RhexData::Mixed { meta, binary } => {
                let meta_value: Value = serde_json::from_slice(&meta)?;
                let keys: Vec<PublicKey> = cbor2::from_slice(&binary)?;
                (meta_value, keys)
            }
            _ => anyhow::bail!("Invalid RhexData type"),
        };

        // Get groups
        let groups = meta.get("groups");
        if groups.is_some() {
            let groups = util::value_to_string_arr(groups.unwrap().clone())?;
            // Strip them
            self.remove_membership(keys, groups);
        }
        Ok(())
    }
}
