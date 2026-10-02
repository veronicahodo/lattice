use ed25519_dalek::{Verifier, VerifyingKey};

use crate::{
    Rhex,
    signature::{RhexSigAlgo::Ed25519, RhexSignature},
};

impl Rhex {
    /// # validate_sig
    /// Checks a single signature against it's expected hash
    ///
    pub fn validate_sig(&self, pos: usize) -> bool {
        match self.sigs[pos].pk.algo {
            Ed25519 => self.validate_ed25519(self.sigs[pos].clone()),
        }
    }

    pub fn data_size(&self) -> usize {
        minicbor::to_vec(&self.data).unwrap().len()
    }

    fn validate_ed25519(&self, sig: RhexSignature) -> bool {
        let hash = self.get_hash(sig.t);
        let ed_sig = ed25519_dalek::Signature::from_bytes(
            &sig.sig
                .try_into()
                .expect("Ed25519 sig is not 64 bytes long"),
        );
        let pk = match VerifyingKey::from_bytes(
            &sig.pk
                .key_bytes
                .try_into()
                .expect("ed25519 key not 32 bytes"),
        ) {
            Ok(pk) => pk,
            Err(e) => {
                println!("Invalid public key is sig verification: {:?}", e);
                return false;
            }
        };
        match pk.verify(&hash, &ed_sig) {
            Ok(_) => true,
            Err(e) => {
                println!("Invalid signature: {:?}", e);
                false
            }
        }
    }
}
