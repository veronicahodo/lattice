use anyhow::Result;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use minicbor::{Decode, Encode};

#[derive(Debug, Clone, Copy, Encode, Decode)]
pub enum RhexSigAlgo {
    #[n(0)]
    Ed25519,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct AgileKey {
    #[n(0)]
    pub algo: RhexSigAlgo,
    #[n(1)]
    pub key_bytes: Vec<u8>,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct RhexSignature {
    #[n(0)]
    pub pk: AgileKey,
    #[n(1)]
    pub sig: Vec<u8>,
    #[n(2)]
    pub t: RhexSignatureType,
}

#[derive(Debug, Clone, Copy, PartialEq, Encode, Decode)]
pub enum RhexSignatureType {
    #[n(0)]
    Author,
    #[n(1)]
    Usher,
    #[n(2)]
    Quorum(#[n(0)] u64),
    #[n(3)]
    Observer(#[n(0)] u64),
    #[n(4)]
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
        Ok(minicbor::to_vec(self)?)
    }
}
