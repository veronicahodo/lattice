use anyhow::Result;
use iroh::{EndpointId, PublicKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RtScopeGenesis {
    pub desc: String,
}

impl RtScopeGenesis {
    pub fn from_slice(slice: &Vec<u8>) -> Result<Self> {
        Ok(serde_json::from_slice(slice)?)
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(self)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RtScopeCreate {
    pub scope: String,
    pub creator: PublicKey,
    pub ushers: Vec<EndpointId>,
}

impl RtScopeCreate {
    pub fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(cbor2::to_canonical_vec(self)?)
    }

    pub fn from_slice(slice: &Vec<u8>) -> Result<Self> {
        Ok(cbor2::from_slice(slice)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RtScopeRequest {
    pub scope: String,
    pub token: Vec<u8>,
    pub ushers: Vec<EndpointId>,
}

impl RtScopeRequest {
    pub fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(cbor2::to_canonical_vec(self)?)
    }

    pub fn from_slice(slice: &Vec<u8>) -> Result<Self> {
        Ok(cbor2::from_slice(slice)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RtScopeEnd {
    pub by: PublicKey,
    pub reason: Option<String>,
    pub yes_im_sure: bool,
}

impl RtScopeEnd {
    pub fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(cbor2::to_canonical_vec(self)?)
    }

    pub fn from_slice(slice: &Vec<u8>) -> Result<Self> {
        Ok(cbor2::from_slice(slice)?)
    }
}

pub type RtScopeUpdate = Vec<EndpointId>;
