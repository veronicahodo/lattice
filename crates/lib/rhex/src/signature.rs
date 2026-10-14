use anyhow::Result;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RhexSigAlgo {
    Ed25519,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgileKey {
    pub algo: RhexSigAlgo,
    pub key_bytes: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhexSignature {
    pub pk: AgileKey,
    pub sig: Vec<u8>,
    pub t: RhexSignatureType,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RhexSignatureType {
    Author,
    Usher,
    Quorum(u64),
    Observer(u64),
    Other,
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
}
