use anyhow::Result;
use serde::{Deserialize, Serialize};
/*
    Structure: Rhex

    This is the heart of the whole lattice. This is what each record
    is composed of.

    magic: unique "version" number of the record. We've made a few
        revisions over time, and this is the current implementation.
        As no ushers for earlier versions were completed this is
        techically the first implemented version @ ver 2
    intent: This is what the author signs over and submits to the
        indicated usher. See intent.rs for more details.
    context: Applied by the receiving usher and used as the telemetry
        metric used in quorum. See context.rs for more details.
    sigs: An ordered array of RhexSignature. Author = 0, usher = 1,
        quorum and observers = 2..n
    curr: The final current hash after the author has assembled the
        quorum responses and seals it for final approval to the usher.
*/
use crate::{
    context::RhexContext,
    intent::RhexIntent,
    signature::{RhexSignature, RhexSignatureType},
};

pub mod build;
pub mod chain;
pub mod check;
pub mod context;
pub mod data;
pub mod intent;
pub mod iroh;
pub mod print;
pub mod respond;
pub mod signature;
pub mod validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rhex {
    pub magic: [u8; 6],
    pub intent: RhexIntent,
    pub data: Vec<u8>,
    pub context: Option<RhexContext>,
    pub sigs: Vec<RhexSignature>,
    pub curr: Option<[u8; 32]>,
}

impl Rhex {
    pub const MAGIC: [u8; 6] = *b"RHEX\x00\x03";
    pub const MAX_DATA: usize = 1024;
    pub const MAX_OVERALL: usize = 4096;

    pub fn new() -> Self {
        Self {
            magic: Rhex::MAGIC,
            intent: RhexIntent::new(),
            data: vec![],
            context: None,
            sigs: Vec::new(),
            curr: None,
        }
    }

    pub fn get_hash(&self, sig_type: RhexSignatureType) -> [u8; 32] {
        match sig_type {
            RhexSignatureType::Author => self.intent.get_sig_hash(),
            RhexSignatureType::Usher => {
                let mut hasher = blake3::Hasher::new();
                let author_hash = self.get_sig_group_hash(&RhexSignatureType::Author);

                hasher.update(b"RHEX_USHER_SIG_0");
                hasher.update(&author_hash);
                hasher.update(&self.context.clone().unwrap().get_hash());
                hasher.finalize().into()
            }
            RhexSignatureType::Quorum(t) | RhexSignatureType::Observer(t) => {
                let mut hasher = blake3::Hasher::new();
                let author_hash = self.get_sig_group_hash(&RhexSignatureType::Author);
                let usher_hash = self.get_sig_group_hash(&RhexSignatureType::Usher);
                hasher.update(b"RHEX_OBSERVED_SIG_0");
                hasher.update(&author_hash);
                hasher.update(&usher_hash);
                // This is what google said to do, I feel like it should
                // be little endian but what do I know?
                hasher.update(&t.to_be_bytes());
                hasher.finalize().into()
            }
            RhexSignatureType::Other => unimplemented!(),
        }
    }

    /// Sorts signatures by ranking (signing order, really), first by
    /// key type and then by key bytes itself.
    pub fn sort_sigs(&mut self) {
        self.sigs.sort_by(|a, b| {
            a.t.sort_rank()
                .cmp(&b.t.sort_rank())
                .then_with(|| a.pk.key_bytes.cmp(&b.pk.key_bytes))
        });
    }

    pub fn to_sorted(input: &Vec<RhexSignature>) -> Vec<RhexSignature> {
        let mut sigs = input.clone();
        sigs.sort_by(|a, b| {
            a.t.sort_rank()
                .cmp(&b.t.sort_rank())
                .then_with(|| a.pk.key_bytes.cmp(&b.pk.key_bytes))
        });
        sigs
    }

    pub fn get_sig_group_hash(&self, sig_type: &RhexSignatureType) -> [u8; 32] {
        let group = self.clone().get_sigs(sig_type).unwrap_or(vec![]);
        let mut hasher = blake3::Hasher::new();
        for g in group {
            hasher.update(&g.sig);
        }
        hasher.finalize().into()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let bin = cbor2::to_canonical_vec(&self);
        bin.unwrap()
    }

    pub fn from_vec(data: &[u8]) -> Self {
        let restored_rhex: Rhex = cbor2::from_slice(&data).unwrap();
        restored_rhex
    }

    /// Calculates the current hash of the record
    /// H(label | intent | context | sigs)
    pub fn calc_curr(&self) -> [u8; 32] {
        let sigs = Self::to_sorted(&self.sigs);
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"RHEX_CURR_HASH_0");
        hasher.update(&self.intent.get_hash());
        hasher.update(&self.context.clone().unwrap().get_hash());
        let sigs_bin = cbor2::to_canonical_vec(&sigs);
        hasher.update(&sigs_bin.unwrap());
        hasher.finalize().into()
    }

    pub fn single_disk_get(path: &str) -> Self {
        let data = std::fs::read(path).unwrap();
        Self::from_vec(&data)
    }

    pub fn single_disk_put(&self, path: &str) {
        std::fs::write(path, &self.to_vec()).unwrap();
    }

    pub fn validate_curr(&self) -> bool {
        if self.curr.is_none() {
            return false;
        }
        let hash = self.calc_curr();
        hash == self.curr.unwrap()
    }

    pub fn validate(&self) -> Result<bool> {
        let mut valid = true;
        for sig in &self.sigs {
            let sig_ok = sig.validate(&self.get_hash(sig.t))?;
            valid = valid && sig_ok;
        }
        valid = valid && self.validate_curr();
        Ok(valid)
    }
}
