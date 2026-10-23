use anyhow::Result;

use crate::{
    Rhex,
    signature::{RhexSigAlgo::Ed25519, RhexSignature},
};

impl Rhex {
    /// # validate_sig
    /// Checks a single signature against it's expected hash
    ///
    pub fn validate_sig(&self, pos: usize) -> Result<bool> {
        match self.sigs[pos].pk.algo {
            Ed25519 => self.validate_ed25519(self.sigs[pos].clone()),
        }
    }

    pub fn data_size(&self) -> usize {
        cbor2::to_canonical_vec(&self.data).unwrap().len()
    }

    pub fn validate_ed25519(&self, sig: RhexSignature) -> Result<bool> {
        let hash = self.get_hash(sig.t);
        sig.validate(&hash)
    }
}
