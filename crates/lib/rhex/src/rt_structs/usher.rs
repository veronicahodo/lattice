use anyhow::{Result, bail};
use iroh::PublicKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RtUsherAssign {
    pub roles: Vec<UsherRole>,
    pub keys: Vec<PublicKey>,
    pub eff: u64,
    pub exp: u64,
}

impl RtUsherAssign {
    pub fn from_slice(slice: &Vec<u8>) -> Result<Self> {
        Ok(cbor2::from_slice(slice)?)
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(cbor2::to_canonical_vec(self)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RtUsherDismiss {
    pub roles: Vec<UsherRole>,
    pub keys: Vec<PublicKey>,
}

impl RtUsherDismiss {
    pub fn from_slice(slice: &Vec<u8>) -> Result<Self> {
        Ok(cbor2::from_slice(slice)?)
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(cbor2::to_canonical_vec(self)?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum UsherRole {
    Actor,
    Mirror,
    Cache,
    Quorum,
    Observer,
    Other,
}

impl UsherRole {
    pub fn to_str(&self) -> Result<String> {
        let string = match self {
            UsherRole::Actor => "actor".to_string(),
            UsherRole::Cache => "cache".to_string(),
            UsherRole::Mirror => "mirror".to_string(),
            UsherRole::Quorum => "quorum".to_string(),
            UsherRole::Observer => "observer".to_string(),
            UsherRole::Other => "other".to_string(),
        };
        Ok(string)
    }

    pub fn from_string(string: &String) -> Result<Self> {
        let role = match string.as_str() {
            "actor" => UsherRole::Actor,
            "cache" => UsherRole::Cache,
            "mirror" => UsherRole::Mirror,
            "quorum" => UsherRole::Quorum,
            "observer" => UsherRole::Observer,
            "other" => UsherRole::Other,
            _ => bail!("Invalid type in UsherRole::from_string"),
        };
        Ok(role)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsherAssignment {
    pub issued: u64,
    pub priority: u8,
    pub roles: Vec<UsherRole>,
    pub eff: u64,
    pub exp: u64,
    pub by: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsherStatus {
    pub issued: u64,
    pub eff: u64,
    pub exp: u64,
    pub by: PublicKey,
}
