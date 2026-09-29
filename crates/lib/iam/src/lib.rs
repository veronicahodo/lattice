use std::{collections::HashMap, fs};

use anyhow::Result;
use lattice::scope::Scope;
use minicbor::{Decode, Encode};

use crate::location::IAmLocation;

pub mod location;
pub mod pick;

/// # IAm
/// This is not like traditional IAM, but rather just keeping track
/// of which keys we own and have access to.
///
#[derive(Debug, Clone)]
pub struct IAm {
    entries: HashMap<[u8; 32], IAmMember>,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct IAmMember {
    #[n(0)]
    #[cbor(with = "minicbor::bytes")]
    pub pk: [u8; 32],
    #[n(1)]
    pub name: Option<String>,
    #[n(2)]
    pub location: IAmLocation,
    #[n(3)]
    pub last_updated: u64,
}

impl IAm {
    /// # new()
    /// Creates a new entries map and returns it.
    ///
    pub fn new() -> IAm {
        IAm {
            entries: HashMap::new(),
        }
    }

    /// # add(self, pk, name, location)
    /// Adds a new key to the entries map.
    /// This should be called with the key returned with
    /// enclave.generate()
    ///
    pub fn add(&mut self, pk: [u8; 32], name: Option<String>, location: IAmLocation) {
        self.entries.insert(
            pk,
            IAmMember {
                pk,
                name,
                location,
                last_updated: 0,
            },
        );
    }

    /// # remove(self, pk)
    /// Removes a key from the entries map.
    ///
    pub fn remove(&mut self, pk: [u8; 32]) {
        self.entries.remove(&pk);
    }

    /// # am_i(self, pubkey)
    /// Checks if the key is in the entries map.
    ///
    pub fn am_i(&self, pubkey: &[u8; 32]) -> Result<bool> {
        let entry = self.entries.get(pubkey);
        if entry.is_none() { Ok(false) } else { Ok(true) }
    }

    /// # am_i_any(self, Vec<[u8; 32]>, local_only)
    ///
    /// Checks the Vec to see if we are any of those keys. The
    /// local_only boolean is for when we want just keys that are
    /// local to this node
    ///
    pub fn am_i_any(&self, keys: Vec<[u8; 32]>, local_only: bool) -> Result<Vec<[u8; 32]>> {
        let mut out = Vec::new();
        for key in keys {
            let matching = self.entries.get(&key);
            if matching.is_none() {
                continue;
            } else {
                if local_only {
                    match &matching.unwrap().location {
                        IAmLocation::Local => {
                            out.push(key.clone());
                        }
                        _ => continue,
                    }
                } else {
                    out.push(key.clone());
                }
            }
        }
        Ok(out)
    }

    /// # is_usher(self, scope, time)
    /// Checks if the key is in the entries map and matches at least
    /// one usher in the scope.
    ///
    pub fn is_usher(&self, scope: &Scope, time: u64) -> Result<Option<[u8; 32]>> {
        let ushers = scope.ushers_at(time)?;
        for usher in ushers {
            let result = self.entries.get(&usher.0);
            if result.is_some() {
                return Ok(Some(usher.0));
            }
        }
        Ok(None)
    }

    /// # disk_from(path)
    /// Loads the entries map from a file.
    ///
    pub fn disk_from(path: &String) -> Result<IAm> {
        let cbor = fs::read(path)?;
        let iam_vec: Vec<IAmMember> = minicbor::decode(&cbor)?;
        let mut iam = HashMap::new();
        for member in iam_vec {
            iam.insert(member.pk, member);
        }
        Ok(IAm { entries: iam })
    }

    /// # disk_to(me, path)
    /// Saves the entries map to a file.
    ///
    pub fn disk_to(&self, path: &String) -> Result<()> {
        let mut iam_vec = Vec::new();
        for (_, member) in self.entries.clone() {
            iam_vec.push(member.clone());
        }
        let contents = minicbor::to_vec(iam_vec)?;
        fs::write(path, contents)?;
        Ok(())
    }

    /// # get_local(self)
    /// Gets all the keys in I Am that are location = local
    ///
    pub fn get_local(&self) -> Result<Vec<[u8; 32]>> {
        let mut output = Vec::new();
        for entry in &self.entries {
            if entry.1.location == IAmLocation::Local {
                output.push(entry.0.clone());
            }
        }
        Ok(output)
    }
}
