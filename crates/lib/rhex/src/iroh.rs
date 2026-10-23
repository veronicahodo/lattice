use serde::{Deserialize, Serialize};

use crate::{context::RhexContext, intent::RhexIntent, signature::RhexSignature};

/// # RhexEnvelope
///
/// Designed to be the object stored in Iroh Blobs. This is what we
/// use to get our address.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhexEnvelope {
    intent: RhexIntent,
    context: RhexContext,
    sigs: Vec<RhexSignature>,
}

impl RhexEnvelope {
    pub fn to_vec(&self) -> Vec<u8> {
        cbor2::to_canonical_vec(&self).unwrap()
    }
}
