use anyhow::Result;
use iroh::PublicKey;
use rhex::{context::RhexContext, signature::RhexSignature};
use serde::{Deserialize, Serialize};

use crate::location::UsherLocation;
pub use crate::map::UsherMap;

pub mod location;
pub mod map;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usher {
    pub name: Option<String>,
    pub pk: PublicKey,
    pub location: UsherLocation,
    pub last_updated: u64,
}

impl Usher {
    pub fn new() -> Self {
        Self {
            name: None,
            pk: PublicKey::from_bytes(&[0; 32]).unwrap(),
            location: UsherLocation::Unknown,
            last_updated: 0,
        }
    }

    pub fn create(name: Option<String>, pk: PublicKey, location: UsherLocation, now: u64) -> Self {
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
    pub appended: bool,
}

impl UsherSigResponse {
    pub fn new(context: RhexContext, sig: RhexSignature, appended: bool) -> Self {
        Self {
            context,
            sig,
            appended,
        }
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(cbor2::to_canonical_vec(self)?)
    }

    pub fn from_slice(data: &[u8]) -> Result<Self> {
        Ok(cbor2::from_slice(data)?)
    }
}
