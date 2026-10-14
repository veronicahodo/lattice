use anyhow::Result;
use rhex::{context::RhexContext, signature::RhexSignature};
use serde::{Deserialize, Serialize};

use crate::location::UsherLocation;
pub use crate::map::UsherMap;

pub mod location;
pub mod map;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usher {
    pub name: Option<String>,
    pub pk: [u8; 32],
    pub location: UsherLocation,
    pub last_updated: u64,
}

impl Usher {
    pub fn new() -> Self {
        Self {
            name: None,
            pk: [0; 32],
            location: UsherLocation::Unknown,
            last_updated: 0,
        }
    }

    pub fn create(name: Option<String>, pk: [u8; 32], location: UsherLocation, now: u64) -> Self {
        Self {
            name,
            pk,
            location,
            last_updated: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsherSigResponse {
    pub context: RhexContext,
    pub sig: RhexSignature,
}

impl UsherSigResponse {
    pub fn new(context: RhexContext, sig: RhexSignature) -> Self {
        Self { context, sig }
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(cbor2::to_canonical_vec(self)?)
    }

    pub fn from_slice(data: &[u8]) -> Result<Self> {
        Ok(cbor2::from_slice(data)?)
    }
}
