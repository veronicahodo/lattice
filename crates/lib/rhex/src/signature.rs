use anyhow::Result;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use ed25519_dalek::Verifier;
use serde::{Deserialize, Serialize};

use crate::Rhex;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RhexSigAlgo {
    Ed25519,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AgileKey {
    pub algo: RhexSigAlgo,
    pub key_bytes: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, PartialOrd, Ord, Eq)]
pub struct RhexSignature {
    pub pk: AgileKey,
    pub sig: Vec<u8>,
    pub t: RhexSignatureType,
}

impl RhexSignature {
    pub fn print(&self) -> String {
        let pk = URL_SAFE_NO_PAD.encode(self.pk.key_bytes.clone());
        let sig = URL_SAFE_NO_PAD.encode(self.sig.clone());
        let t = match self.t {
            RhexSignatureType::Author => &"Author".to_string(),
            RhexSignatureType::Usher => &"Usher".to_string(),
            RhexSignatureType::Quorum(t) => &format!("Quorum @ {}", t),
            RhexSignatureType::Observer(t) => &format!("Observer @ {}", t),
            RhexSignatureType::Other => "Other",
        };
        format!("{}: [{}]\n\t\t\t{}", t, pk, sig)
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(cbor2::to_canonical_vec(self)?)
    }

    pub fn validate(&self, hash: &[u8; 32]) -> Result<bool> {
        match self.pk.algo {
            RhexSigAlgo::Ed25519 => {
                let key = ed25519_dalek::VerifyingKey::from_bytes(
                    &self.pk.key_bytes.clone().try_into().unwrap(),
                )?;
                if let Ok(_) = key.verify(
                    hash,
                    &ed25519_dalek::Signature::from_bytes(&self.sig.clone().try_into().unwrap()),
                ) {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub enum RhexSignatureType {
    Author,
    Usher,
    Quorum(u64),
    Observer(u64),
    Other,
}

impl RhexSignatureType {
    pub fn sort_rank(&self) -> u8 {
        match self {
            RhexSignatureType::Author => 0,
            RhexSignatureType::Usher => 1,
            RhexSignatureType::Quorum(_) => 2,
            RhexSignatureType::Observer(_) => 3,
            _ => 4,
        }
    }
}

impl Rhex {
    pub fn get_sigs(&mut self, sig_type: &RhexSignatureType) -> Result<Vec<RhexSignature>> {
        self.sort_sigs();
        let mut output = Vec::new();
        for sig in &self.sigs {
            if &sig.t == sig_type {
                output.push(sig.clone())
            }
        }
        Ok(output)
    }
}
