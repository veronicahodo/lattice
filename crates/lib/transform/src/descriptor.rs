use minicbor::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub struct TransformDescriptor {
    // Name of the transform. Typically `transform.com.business.action`
    #[n(0)]
    pub name: String,
    // Rhex path to package
    #[n(1)]
    pub src: String,
    // String versioning for just simple ==/!=
    #[n(2)]
    pub version: String,
    // Is this transform used for validation or appending?
    #[n(3)]
    pub action: DescriptorAction,
    // What's the desired mounting point for this transform?
    // * or rhex://some.prefix.scope.<mount>/ for the first string
    // and then the Record Type to interact with.
    #[n(4)]
    pub trigger: (String, String),
    // This is indexed by scope, containing the record types observed
    // or emitted by this transform.
    // e.g.: "rhex://<trigger>.data/", vec!["schema:set", "schema:retire", etc]
    // or: "*", vec!["motor:nameplate"]
    #[n(5)]
    pub input: Vec<(String, Vec<String>)>,
    #[n(6)]
    pub output: Vec<(String, Vec<String>)>,
    #[n(7)]
    pub hash: [u8; 32],
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, Hash)]
pub enum DescriptorAction {
    #[n(0)]
    Validate,
    #[n(1)]
    Appending,
}
