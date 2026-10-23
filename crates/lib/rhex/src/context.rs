use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhexContext {
    pub at: u64,
    pub s: Option<ContextSpacial>,
}

impl RhexContext {
    pub fn new(at: u64, s: Option<ContextSpacial>) -> Self {
        Self { at, s }
    }

    pub fn get_hash(&self) -> [u8; 32] {
        let bin = cbor2::to_canonical_vec(self);
        blake3::hash(&bin.unwrap()).into()
    }
}

impl RhexContext {
    pub fn get_sig_hash(&self, author_sig: &[u8; 64]) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"RHEX_USHER_SIG_0");
        hasher.update(author_sig);
        hasher.update(&cbor2::to_canonical_vec(&self).unwrap());
        hasher.finalize().into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSpacial {
    pub s_ref: String,
    pub s_data: Vec<u8>,
}

impl ContextSpacial {
    pub fn new(s_ref: String, s_data: Vec<u8>) -> Self {
        Self { s_ref, s_data }
    }
}
