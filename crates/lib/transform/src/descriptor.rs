use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransformDescriptor {
    // Name of the transform. Typically `transform.com.business.action`
    pub name: String,
    // Rhex path to package
    pub src: String,
    // String versioning for just simple ==/!=
    pub version: String,
    // Is this transform used for validation or appending?
    pub action: DescriptorAction,
    // What's the desired mounting point for this transform?
    // * or rhex://some.prefix.scope.<mount>/ for the first string
    // and then the Record Type to interact with.
    pub trigger: (String, String),
    // This is indexed by scope, containing the record types observed
    // or emitted by this transform.
    // e.g.: "rhex://<trigger>.data/", vec!["schema:set", "schema:retire", etc]
    // or: "*", vec!["motor:nameplate"]
    pub input: HashMap<String, Vec<String>>,
    pub output: HashMap<String, Vec<String>>,
    pub hash: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DescriptorAction {
    Validate,
    Appending,
}
