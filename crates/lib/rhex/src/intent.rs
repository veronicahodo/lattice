use iroh::{EndpointId, PublicKey};
use serde::{Deserialize, Serialize};
/*
    Structure: RhexIntent

    This is the core of what the author proposes. Only the author
    gets to set these values, so head lookup is important before
    signing because the "prev"ious hash can't be set after the fact.

    prev: the "curr"ent hash of the last record in the chain
    scope: The namespace this record lives in
    nonce: rando generated
    author: Public key of the author
    usher: Public key of the usher
    schema: lattice referrenced (rhex://) or internet
        referrenced (https://)
    rt: Record type, eg, "policy:set", "key:grant"
    data: Enum of the possible payload types. See data.rs for more
        details.
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhexIntent {
    pub prev: Option<[u8; 32]>,
    pub scope: String,
    pub nonce: [u8; 16],
    pub author: PublicKey,
    pub usher: EndpointId,
    pub schema: Option<String>,
    pub rt: String,
    pub data_hash: Option<[u8; 32]>,
}

impl RhexIntent {
    pub fn new() -> Self {
        Self {
            prev: None,
            scope: "".to_string(),
            nonce: Self::gen_nonce(),
            author: PublicKey::from_bytes(&[0; 32]).unwrap(),
            usher: EndpointId::from_bytes(&[0; 32]).unwrap(),
            schema: None,
            rt: "".to_string(),
            data_hash: None,
        }
    }

    pub fn get_hash(&self) -> [u8; 32] {
        let bin = cbor2::to_canonical_vec(self);
        blake3::hash(&bin.unwrap()).as_bytes().clone()
    }

    pub fn get_sig_hash(&self) -> [u8; 32] {
        let bin = cbor2::to_canonical_vec(self);
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"RHEX_AUTHOR_SIG_0");
        hasher.update(&bin.unwrap());
        hasher.finalize().into()
    }

    pub fn gen_nonce() -> [u8; 16] {
        rand::random()
    }
}
