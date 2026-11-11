use std::collections::HashMap;

use anyhow::{Result, bail};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RtRequestRhex {
    pub identifiers: Vec<String>,
}

impl RtRequestRhex {
    pub fn from_slice(slice: &Vec<u8>) -> Result<Self> {
        Ok(cbor2::from_slice(slice)?)
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(cbor2::to_canonical_vec(self)?)
    }

    pub fn identifiers_to_addresses(
        &self,
        aliases: &HashMap<String, [u8; 32]>,
    ) -> Result<Vec<[u8; 32]>> {
        let mut output = Vec::new();
        for i in &self.identifiers {
            if i.starts_with("@") {
                let address = aliases.get(&i[1..].to_string());
                if address.is_none() {
                    bail!("Alias not resolved: @{}", i[1..].to_string());
                }
                output.push(address.unwrap().clone())
            }
            if i.starts_with("hex:") {
                let address: [u8; 32] = hex::decode(i[4..].to_string())?.try_into().unwrap();
                output.push(address.clone())
            }
            if i.starts_with("base64:") {
                let address: [u8; 32] = URL_SAFE_NO_PAD
                    .decode(i[7..].to_string())?
                    .try_into()
                    .unwrap();
                output.push(address.clone())
            }
        }

        Ok(output)
    }
}
