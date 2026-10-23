use anyhow::Result;
use iroh::EndpointId;
use lattice::{
    Rhex,
    rhex::{
        check::CheckStatus, context::RhexContext, intent::RhexIntent, signature::RhexSignature,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatterRequest {
    pub version: u16,
    pub address: EndpointId,
    pub request: ChatterRequestType,
}

impl ChatterRequest {
    pub fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(cbor2::to_canonical_vec(self)?)
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        Ok(cbor2::from_slice(slice)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatterResponse {
    pub version: u16,
    pub response: ChatterResponseType,
    pub payload: Vec<Rhex>,
}

impl ChatterResponse {
    pub fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(cbor2::to_canonical_vec(self)?)
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        Ok(cbor2::from_slice(slice)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatterRequestType {
    AuthorSign {
        intent: RhexIntent,
        token: Vec<u8>,
    },
    UsherSign {
        intent: RhexIntent,
        sigs: Vec<RhexSignature>,
    },
    QuorumSign {
        intent: RhexIntent,
        context: RhexContext,
        sigs: Vec<RhexSignature>,
    },
    ObserverSign([u8; 32]),
    Submit(Rhex),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatterResponseType {
    AuthorSignResponse {
        intent_hash: [u8; 32],
        sig: RhexSignature,
    },
    UsherSignResponse {
        usher_hash: [u8; 32],
        context: RhexContext,
        sig: RhexSignature,
    },
    QuorumSignResponse {
        sig_hash: [u8; 32],
        sig: RhexSignature,
    },
    ObserverSignResponse(RhexSignature),
    Appended {
        scope: String,
        final_hash: [u8; 32],
    },
    Accepted([u8; 32]),
    Rejected(Vec<CheckStatus>),
}
