use anyhow::{Result, bail};
use blake3::Hasher;
use key::enclave::Enclave;
use lattice::rhex::{data::RhexData, signature::RhexSignature};
use minicbor::{Decode, Encode};

/// # time_go(enclave, key, hash, time)
///
/// This literally takes the submitted 128 bytes, stacks the time at
/// the end and signs over it with whatever key we provide.
///
pub fn time_go(
    enclave: &Enclave,
    key: &[u8; 32],
    hash: &[u8; 128],
    time: &u64,
) -> Result<[u8; 64]> {
    let mut hasher = Hasher::new();
    hasher.update(hash);
    hasher.update(&time.to_be_bytes());
    let complete_hash = hasher.finalize();
    enclave.sign(key, complete_hash.as_bytes())
}

pub fn time_go_from_data(data: RhexData) -> Result<[u8; 128]> {
    match data {
        RhexData::Binary(b) => {
            if b.len() != 128 {
                bail!("Time Go data incorrect size");
            }
            Ok(b.try_into().unwrap())
        }
        _ => bail!("Incorrect Time Go data type"),
    }
}

#[derive(Debug, Encode, Decode)]
pub struct TimeGoResponse {
    #[n(0)]
    pub time: u64,
    #[n(1)]
    pub sig: RhexSignature,
}

impl TimeGoResponse {
    pub fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(minicbor::to_vec(self)?)
    }
}
