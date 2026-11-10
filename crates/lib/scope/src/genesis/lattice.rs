use anyhow::{Result, bail};
use iroh::PublicKey;
use rhex::{
    Rhex,
    data::RhexData,
    rt_structs::policy::{RtPolicySet, Rule},
};
use serde_json::Value;

use crate::{Scope, membership::Membership};

impl Scope {
    pub fn process_lattice_genesis(&mut self, rhex: &Rhex) -> Result<()> {
        if rhex.data.len() == 0 {
            bail!("No data payload for lattice:genesis");
        }
        let data: RhexData = cbor2::from_slice(&rhex.data)?;
        let (at, key) = match data {
            RhexData::Mixed { meta, binary } => {
                let json: Value = serde_json::from_slice(&meta)?;
                let at = json.get("at");
                let at = match at {
                    None => 1793124803572,
                    Some(at) => at.as_i64().unwrap(),
                };
                (at, PublicKey::from_bytes(&binary.try_into().unwrap())?)
            }
            _ => bail!("lattice:genesis is wrong data type"),
        };
        println!("Setting global time using Unix MS: {}", at);
        let status = time::set_global_genesis(at);
        println!("Time was set to: {}", time::get_global_genesis());
        if status.is_err() {
            println!(
                "Couldn't set time. Time set to: {}",
                time::get_current_time()
            );
        } else {
            println!("Time is now: {}", time::print_gt(&time::get_current_time()));
        }
        let policy = RtPolicySet {
            desc: "Lattice Genesis Awaiting".to_string(),
            rules: vec![Rule {
                append: vec!["world_line_zero".to_string()],
                k: 1,
                o: 1,
                quorum: vec!["world_line_zero".to_string()],
                delay: 0,
                rt: vec!["policy:set".to_string()],
                window: 1_000,
            }],
            eff: 1,
            exp: 1_000_000_000_000_000,
            tags: vec![],
            issued: 1,
        };
        self.policy_map.push((1, policy));

        self.add_membership(
            "world_line_zero".to_string(),
            vec![key.clone()],
            Membership {
                issued: 0,
                eff: 0,
                exp: 1_000_000_000_000_000,
                by: key,
            },
        )?;
        Ok(())
    }
}
