use anyhow::Result;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RhexData {
    None,
    Json(Vec<u8>),
    Binary(Vec<u8>),
    Mixed { meta: Vec<u8>, binary: Vec<u8> },
    Removed(String),
}

impl RhexData {
    pub fn print(&self) -> String {
        match self {
            RhexData::None => "None".to_string(),
            RhexData::Json(json) => serde_json::to_string(json).unwrap(),
            RhexData::Binary(data) => URL_SAFE_NO_PAD.encode(data),
            RhexData::Mixed { meta, binary } => {
                let meta_str = serde_json::to_string(meta).unwrap();
                let binary_str = URL_SAFE_NO_PAD.encode(binary);
                format!("Meta: {}\n\t\tBinary: {}", meta_str, binary_str)
            }
            RhexData::Removed(s) => {
                format!("Removed (Reason: {})", s)
            }
        }
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(cbor2::to_canonical_vec(self)?)
    }

    pub fn from_vec(data: &Vec<u8>) -> Result<Self> {
        Ok(cbor2::from_slice(data)?)
    }

    pub fn get_hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.to_vec().unwrap());
        hasher.finalize().into()
    }
}
