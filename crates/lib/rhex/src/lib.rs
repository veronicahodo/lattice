use minicbor::{Decode, Encode};
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
pub mod print;
pub mod respond;
pub mod signature;
pub mod validate;

#[derive(Debug, Clone, Encode, Decode)]
pub struct Rhex {
    #[n(0)]
    pub magic: [u8; 6],
    #[n(1)]
    pub intent: RhexIntent,
    #[n(2)]
    pub data: Vec<u8>,
    #[n(3)]
    pub context: RhexContext,
    #[n(4)]
    pub sigs: Vec<RhexSignature>,
    #[n(5)]
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
            context: RhexContext { at: 0, s: None },
            sigs: Vec::new(),
            curr: None,
        }
    }

    pub fn get_hash(&self, sig_type: RhexSignatureType) -> [u8; 32] {
        match sig_type {
            RhexSignatureType::Author => {
                let mut hasher = blake3::Hasher::new();
                hasher.update(b"RHEX_AUTHOR_SIG_0");
                hasher.update(&self.intent.get_hash().clone());
                hasher.finalize().into()
            }
            RhexSignatureType::Usher => {
                let mut hasher = blake3::Hasher::new();
                hasher.update(b"RHEX_USHER_SIG_0");
                hasher.update(&self.sigs[0].sig);
                hasher.update(&self.context.get_hash());
                hasher.finalize().into()
            }
            RhexSignatureType::Quorum(t) | RhexSignatureType::Observer(t) => {
                let mut hasher = blake3::Hasher::new();
                hasher.update(b"RHEX_OBSERVED_SIG_0");
                hasher.update(&self.sigs[0].sig);
                hasher.update(&self.sigs[1].sig);
                // This is what google said to do, I feel like it should
                // be little endian but what do I know?
                hasher.update(&t.to_be_bytes());
                hasher.finalize().into()
            }
            RhexSignatureType::Other => unimplemented!(),
        }
    }

    /// Sorts signatures 2..n
    /// First two are fixed (Author, Usher), but the rest are stored
    /// byte sorted by the signature itself.
    pub fn sort_sigs(&mut self) {
        let author = self.sigs.remove(0);
        let usher = self.sigs.remove(0);
        self.sigs.sort_by(|a, b| a.sig.cmp(&b.sig));
        self.sigs.insert(0, author);
        self.sigs.insert(1, usher);
    }

    pub fn to_vec(&self) -> Vec<u8> {
        minicbor::to_vec(&self).unwrap()
    }

    pub fn from_vec(data: &[u8]) -> Self {
        let restored_rhex: Rhex = minicbor::decode(data).unwrap();
        restored_rhex
    }

    /// Calculates the current hash of the record
    /// H(label | intent | context | sigs)
    pub fn calc_curr(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"RHEX_CURR_HASH_0");
        hasher.update(&self.intent.get_hash());
        hasher.update(&self.context.get_hash());
        hasher.update(&minicbor::to_vec(&self.sigs).unwrap());
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

    pub fn validate(&self) -> bool {
        let mut valid = true;
        for i in 0..self.sigs.len() {
            let sig_ok = self.validate_sig(i);
            valid = valid && sig_ok;
        }
        valid = valid && self.validate_curr();
        valid
    }
}
