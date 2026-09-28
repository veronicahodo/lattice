use anyhow::Result;
use minicbor::{Decode, Encode};

use crate::registry::TransformRegistry;

impl TransformRegistry {
    pub fn from_file(file: &String, trans_dir: &String) -> Result<Self> {
        // Get the stored registry
        let trans_file_bin = std::fs::read(file)?;
        let trans_file: Vec<TransFileTriggers> = minicbor::decode(&trans_file_bin)?;

        let mut trans_reg = TransformRegistry::new();
        for trans in trans_file {
            trans_reg.add_transform(&trans.name, &trans.scope, trans_dir)?;
        }
        Ok(trans_reg)
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct TransFileTriggers {
    #[n(0)]
    name: String,
    #[n(1)]
    scope: String,
    #[n(2)]
    rt: String,
    #[n(3)]
    inputs: Vec<(String, String)>,
    #[n(4)]
    outputs: Vec<(String, String)>,
}
