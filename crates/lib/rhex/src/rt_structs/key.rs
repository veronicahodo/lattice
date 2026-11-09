use anyhow::Result;
use iroh::PublicKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RtKeyGrant {
    pub groups: Vec<String>,
    pub keys: Vec<PublicKey>,
}

impl RtKeyGrant {
    pub fn from_slice(slice: &Vec<u8>) -> Result<Self> {
        Ok(cbor2::from_slice(slice)?)
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(cbor2::to_canonical_vec(self)?)
    }
}
