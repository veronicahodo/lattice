use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub append: Vec<String>,
    pub k: u16,
    pub o: u16,
    pub quorum: Vec<String>,
    pub delay: u64,
    pub rt: Vec<String>,
    pub window: u64,
}

impl Rule {
    pub fn valid(&self, rt: &String, groups: &Vec<String>) -> Result<bool> {
        let mut valid = false;
        for r in self.rt.clone() {
            if r.ends_with(":*") {
                let parent: Vec<&str> = r.split(":").collect();
                let rt_parent: Vec<&str> = rt.split(":").collect();
                if parent[0] == rt_parent[0] {
                    for g in groups {
                        if self.append.contains(g) {
                            valid = true;
                        }
                    }
                }
            } else {
                if &r == rt {
                    for g in groups {
                        if self.append.contains(g) {
                            valid = true;
                        }
                    }
                }
            }
        }
        Ok(valid)
    }
}
