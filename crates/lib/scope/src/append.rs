use anyhow::Result;
use rhex::Rhex;

use crate::{Scope, append::AppendStatus::DiskError};

impl Scope {
    /// # append(mut self, rhex, time, path)
    ///
    /// This is what physically changes the chain on disk as well
    /// as updating the scope's head in the lattice object.
    ///
    pub fn append(&mut self, rhex: &Rhex, time: u64, path: &String) -> Result<AppendStatus> {
        // Load the chain from disk
        let chain = Rhex::chain_from_disk(path);
        if chain.is_err() {
            return Ok(DiskError("Failed reading".to_string()));
        }
        let mut chain = chain.unwrap();

        // Check continuity
        let head = if chain.len() == 0 {
            None
        } else {
            chain[chain.len()].curr
        };
        if head != rhex.intent.prev {
            return Ok(AppendStatus::ChainError {
                proposed_prev: rhex.intent.prev,
                chain_on_disk: head,
            });
        }

        // Append to chain
        chain.push(rhex.clone());

        // Store back to disk
        let status = Rhex::chain_to_disk(&path, &chain);
        if status.is_err() {
            return Ok(DiskError("Failed writing".to_string()));
        }

        // Update scope object
        self.head = rhex.curr.clone();
        self.updated = time;

        // output
        Ok(AppendStatus::Success)
    }
}

pub enum AppendStatus {
    Success,
    DiskError(String),
    ChainError {
        proposed_prev: Option<[u8; 32]>,
        chain_on_disk: Option<[u8; 32]>,
    },
}
