use anyhow::Result;
use iroh::PublicKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RtLatticeGenesis {
    pub at: i64,
    pub genesis_key: PublicKey,
}

impl RtLatticeGenesis {
    pub fn from_slice(slice: &Vec<u8>) -> Result<Self> {
        Ok(cbor2::from_slice(slice)?)
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(cbor2::to_canonical_vec(self)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RtLatticeManifesto {
    pub text: String,
}

impl RtLatticeManifesto {
    pub fn from_slice(slice: &Vec<u8>) -> Result<Self> {
        Ok(cbor2::from_slice(slice)?)
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(cbor2::to_canonical_vec(self)?)
    }
}
