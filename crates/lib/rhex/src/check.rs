use anyhow::Result;

use crate::{Rhex, signature::RhexSigAlgo::Ed25519};

impl Rhex {
    /// # check_curr_hash
    /// Checks if the Rhex's proposed `curr` matches the calculated
    /// final hash of the object
    ///
    pub fn check_curr_hash(&self) -> Result<CheckStatus> {
        let calc_curr = self.calc_curr();
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
            Ed25519 => self.validate_sig(pos),
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
        let mut buf = Vec::new();
        minicbor::encode(self, &mut buf)?;
        if buf.len() > Rhex::MAX_OVERALL {
            return Ok(CheckStatus::RhexBloated(buf.len()));
        }
        Ok(CheckStatus::Success)
    }
}

/// # CheckStatus
/// This is all the possible outcomes of the "check" functions
///
#[derive(Debug, Clone, PartialEq)]
pub enum CheckStatus {
    Success,
    PrevHashMismatch {
        presented: Option<[u8; 32]>,
        expected: Option<[u8; 32]>,
    },
    NotThisScope {
        presented: String,
        expected: String,
    },
    NonceReused,
    AccessDenied,
    InvalidUsher,
    RtNotAllowed,
    DataBloated(usize),
    InteractionAborted {
        transform: String,
        exitcode: usize,
    },
    SchemaFailed(String),
    SchemaNotFound(String),
    TimeReversal {
        presented: u64,
        prev: u64,
    },
    SpacialDataIncorrect(String),
    SignatureInvalid(u8),
    CurrentHashMismatch {
        presented: [u8; 32],
        calculated: [u8; 32],
    },
    CurrentHashNotSet,
    RhexBloated(usize),
    NotUsherForThisScope,
    QuorumCountUnder {
        provided: usize,
        k: usize,
    },
    ObserverCountUnder {
        provided: usize,
        o: usize,
    },
    SignatureNotInWindow(u8),
    Unknown,
}
