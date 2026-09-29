use std::collections::HashMap;

use anyhow::{Result, bail};
use minicbor::{Decode, Encode};

use crate::{descriptor::DescriptorAction, registry::TransformRegistry};

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

    pub fn to_file(&self, file: &String) -> Result<()> {
        let mut transform_lookup: HashMap<[u8; 32], String> = HashMap::new();
        let mut output = Vec::new();
        for actions in &self.triggers {
            let action = actions.0.clone();
            for scope in actions.1 {
                for rt in scope.1 {
                    for key in rt.1 {
                        let transform = transform_lookup.get(key);
                        let transform_name = if transform.is_none() {
                            let desc = self.registry.get(key);
                            if desc.is_none() {
                                bail!(format!(
                                    "Tried to save a transform registry that doesn't contain all referenced transforms. Ref: {:?}",
                                    key
                                ));
                            };
                            let desc = desc.unwrap().descriptor.clone();
                            transform_lookup.insert(key.clone(), desc.name.clone());
                            desc.name
                        } else {
                            transform.unwrap().clone()
                        };
                        output.push(TransFileTriggers {
                            name: transform_name,
                            scope: scope.0.clone(),
                            rt: rt.0.clone(),
                            inputs: vec![],
                            outputs: vec![],
                            action: action.clone(),
                        })
                    }
                }
            }
        }
        std::fs::write(file, minicbor::to_vec(output)?)?;
        Ok(())
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
    #[n(5)]
    action: DescriptorAction,
}
