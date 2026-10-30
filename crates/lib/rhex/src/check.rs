use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    Rhex,
    signature::{RhexSigAlgo::Ed25519, RhexSignatureType},
};

impl Rhex {
    /// # check_curr_hash
    /// Checks if the Rhex's proposed `curr` matches the calculated
    /// final hash of the object
    ///
    pub fn check_curr_hash(&self) -> Result<CheckStatus> {
        let calc_curr = self.calc_curr();
        println!("self: {:?}\ncalced: {:?}", self.calc_curr(), calc_curr);
        if self.curr.is_none() || (calc_curr != self.curr.unwrap()) {
            return Ok(CheckStatus::CurrentHashMismatch {
                presented: self.curr.unwrap(),
                calculated: calc_curr,
            });
        };
        Ok(CheckStatus::Success)
    }

    /// # check_data_size
    /// Very basically checks to see if the CBOR data size is over 1k
    ///
    pub fn check_data_size(&self) -> Result<CheckStatus> {
        let size = self.data_size();
        if size > Rhex::MAX_DATA {
            return Ok(CheckStatus::DataBloated(size));
        }
        Ok(CheckStatus::Success)
    }

    /// # check_schema
    /// This is supposed to check the `intent.schema` and validate
    /// against it. It currently does none of this lol.
    ///
    pub fn check_schema(&self) -> Result<CheckStatus> {
        // TODO: actually do this? lol
        Ok(CheckStatus::Success)
    }

    /// # check_sig
    /// Checks a singular signature by position in rhex.sigs
    ///
    pub fn check_sig(&self, pos: usize) -> Result<CheckStatus> {
        let sig_status = match self.sigs[pos].pk.algo {
            Ed25519 => self.validate_sig(pos)?,
        };
        if sig_status == false {
            return Ok(CheckStatus::SignatureInvalid(pos.try_into().unwrap()));
        }
        Ok(CheckStatus::Success)
    }

    /// # check_total_size
    /// Checks the completed total size of the Rhex and errors if it's
    /// over 4k
    ///
    pub fn check_total_size(&self) -> Result<CheckStatus> {
        let bin = cbor2::to_canonical_vec(&self)?;
        if bin.len() > Rhex::MAX_OVERALL {
            return Ok(CheckStatus::RhexBloated(bin.len()));
        }
        Ok(CheckStatus::Success)
    }
}

/// # CheckStatus
/// This is all the possible outcomes of the "check" functions
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CheckStatus {
    // **** GENERIC ****
    Success,
    Unsupported,
    Unknown,

    // **** INTENT RELATED ****
    PrevHashMismatch {
        presented: Option<[u8; 32]>,
        expected: Option<[u8; 32]>,
    },
    NotThisScope {
        presented: String,
        expected: String,
    },
    NotUsherForThisScope,
    NonceReused,
    AccessDenied,
    InvalidUsher,
    NotMe,
    RtNotAllowed,
    DataBloated(usize),
    InteractionAborted {
        transform: String,
        exitcode: usize,
    },
    SchemaFailed(String),
    SchemaNotFound(String),

    // **** CONTEXT RELATED ****
    SpacialDataIncorrect(String),
    TimeReversal {
        presented: u64,
        prev: u64,
    },

    // **** SIGNATURE RELATED ****
    SignatureInvalid(u8),
    QuorumCountUnder {
        provided: usize,
        k: usize,
    },
    ObserverCountUnder {
        provided: usize,
        o: usize,
    },
    SignatureNotInWindow(u8),
    /*WrongStage {
        attempted: RhexSignatureType,
    },*/
    // **** FINAL RELATED ****
    CurrentHashMismatch {
        presented: [u8; 32],
        calculated: [u8; 32],
    },
    CurrentHashNotSet,
    RhexBloated(usize),
}

pub trait ProcessChecks {
    /// Appends all statuses to `output` and returns `Err(errors)`
    /// if any errors were present, enabling the `?` operator.
    fn check_and_push_into(self, output: &mut Vec<CheckStatus>) -> Result<(), Vec<CheckStatus>>;
}

impl<I: IntoIterator<Item = CheckStatus>> ProcessChecks for I {
    fn check_and_push_into(self, output: &mut Vec<CheckStatus>) -> Result<(), Vec<CheckStatus>> {
        let mut errors = Vec::new();

        for status in self {
            if status != CheckStatus::Success {
                errors.push(status.clone());
            }
            output.push(status);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
