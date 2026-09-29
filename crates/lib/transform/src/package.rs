use std::fs;

use anyhow::Result;
use minicbor::{Decode, Encode};

use crate::descriptor::TransformDescriptor;

#[derive(Debug, Clone, Encode, Decode)]
pub struct TransformPackage {
    #[n(0)]
    pub magic: [u8; 8],
    #[n(1)]
    pub descriptor: TransformDescriptor,
    #[n(2)]
    pub binary: Vec<u8>,
}

impl TransformPackage {
    pub fn new(descriptor: TransformDescriptor, binary: Vec<u8>) -> Self {
        Self {
            magic: *b"TRANS\x00\x00\x00",
            descriptor,
            binary,
        }
    }

    pub fn disk_get(path: String) -> Result<Self> {
        let file_bin = fs::read(path)?;
        let pkg: TransformPackage = minicbor::decode(&file_bin)?;

        Ok(pkg)
    }
}
