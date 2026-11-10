use anyhow::Result;
use iroh::{EndpointId, PublicKey};
use serde::{Deserialize, Serialize};

/// # RtScopeGenesis
///
/// Basic starter for a sub scope. I kinda wonder why I still insist
/// on this, as it really doesn't do much. I just feel like scopes
/// should have a starting record 🤷🏻‍♀️
///
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

/// # RtScopeCreate
///
/// Is issued by an usher, not the author. Authors must submit a
/// `scope:request` to trigger the usher to either create or deny
/// the new scope.
///
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

/// # RtScopeRequest
///
/// This is what the author submits to create a new sub scope. `token`
/// is a unique (and un-reusable) value that the author submits to prove
/// they have done the necessary requirements to create the scope.
///
/// What are those requirements? No clue right now lol.
///
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

/// # RtScopeEnd
///
/// This tombstones a scope. No more appends are allowed. This is
/// obviously lethal if someone just appends one to the scope without
/// authorization, so any scope that allows `scope:end` must explicitly
/// enabled through policy (`scope:*` DOES NOT enable it)
///
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

/// # RtScopeUpdate
///
///
#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct RtScopeUpdate {
    pub subscope: String,
    pub ushers: Vec<EndpointId>,
}

impl RtScopeUpdate {
    pub fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(cbor2::to_canonical_vec(self)?)
    }

    pub fn from_slice(slice: &Vec<u8>) -> Result<Self> {
        Ok(cbor2::from_slice(slice)?)
    }
}
