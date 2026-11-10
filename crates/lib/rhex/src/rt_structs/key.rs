use anyhow::Result;
use iroh::PublicKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RtKeyGrant {
    pub groups: Vec<String>,
    pub keys: Vec<PublicKey>,
    pub eff: u64,
    pub exp: u64,
}

impl RtKeyGrant {
    pub fn from_slice(slice: &Vec<u8>) -> Result<Self> {
        Ok(cbor2::from_slice(slice)?)
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(cbor2::to_canonical_vec(self)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RtKeyRevoke {
    pub groups: Vec<String>,
    pub keys: Vec<PublicKey>,
}

impl RtKeyRevoke {
    pub fn from_slice(slice: &Vec<u8>) -> Result<Self> {
        Ok(cbor2::from_slice(slice)?)
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(cbor2::to_canonical_vec(self)?)
    }
}
