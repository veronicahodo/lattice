use iroh::{EndpointId, PublicKey};
use rand::random;

use crate::{Rhex, context::RhexContext, intent::RhexIntent};

impl Rhex {
    pub fn build(
        prev: Option<[u8; 32]>,
        scope: String,
        nonce: Option<[u8; 32]>,
        author: PublicKey,
        usher: EndpointId,
        rt: String,
        schema: Option<String>,
        data_hash: Option<[u8; 32]>,
    ) -> Self {
        let nonce = match nonce {
            Some(n) => n,
            None => random(),
        };
        Self {
            magic: Rhex::MAGIC,
            intent: RhexIntent {
                prev,
                scope,
                nonce,
                author,
                usher,
                schema,
                rt,
                data_hash,
            },
            data: vec![],
            context: RhexContext { at: 0, s: None },
            sigs: Vec::new(),
            curr: None,
        }
    }
}
