use anyhow::Result;
use iroh_blobs::Hash;
use serde::{Deserialize, Serialize};

use crate::{context::RhexContext, intent::RhexIntent, signature::RhexSignature};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrohRhex {
    pub intent: RhexIntent,
    pub context: RhexContext,
    pub sigs: Vec<RhexSignature>,
}

impl IrohRhex {
    pub fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(cbor2::to_canonical_vec(&self)?)
    }

    pub fn to_hash(&self) -> Result<Hash> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.to_vec()?);
        Ok(Hash::from_bytes(hasher.finalize().into()))
    }
}
