use std::{fs, path::PathBuf};

use anyhow::Result;
use base64::{Engine, engine};
use ed25519_dalek::{Signature, Signer, Verifier};
use minicbor::{Decode, Encode};

pub mod enclave;

/*
Old key struct

#[derive(Debug, Clone, Encode, Decode)]
pub struct Key {
    #[n(0)]
    pub sk: Option<[u8; 32]>,
    #[n(1)]
    pub pk: Option<[u8; 32]>,
    #[n(2)]
    pub name: Option<String>,
}*/

/// This is the main key stucture, v2.0
/// sk = secret key
/// pk = public key (rarely used and not trusted)
/// name = String identifier
/// instanced = enum of possible ways a key gets into the enclave
/// expires = Obviously, when the key expires
///
#[derive(Debug, Clone, Encode, Decode)]
pub struct Key {
    #[n(0)]
    pub sk: Option<[u8; 32]>,
    #[n(1)]
    pub pk: Option<[u8; 32]>,
    #[n(2)]
    pub name: Option<String>,
    #[n(3)]
    pub instanced: KeyInstanced,
    #[n(4)]
    pub expires: Option<u64>,
}

/// Enum of possible enclave entryways
#[derive(Debug, Clone, Encode, Decode)]
pub enum KeyInstanced {
    #[n(0)]
    Unknown,
    #[n(1)]
    Generated(#[n(0)] u64),
    #[n(2)]
    Imported(#[n(0)] u64),
}

impl Key {
    /// # `new(sk, name, instanced, expires`
    ///
    /// Creates a new key from `sk` provided
    ///
    pub fn new(
        sk: [u8; 32],
        name: Option<String>,
        instanced: KeyInstanced,
        expires: Option<u64>,
    ) -> Self {
        let dalek_sk = ed25519_dalek::SigningKey::from_bytes(&sk);
        let pk = dalek_sk.verifying_key().to_bytes();
        Self {
            sk: Some(sk),
            pk: Some(pk),
            name,
            instanced,
            expires,
        }
    }

    /// # `new_pk(pk, name, instanced, expires)`
    ///
    /// Creates just a public key entry. These are generally not stored,
    /// but used for verifying signatures.
    ///
    pub fn new_pk(
        pk: [u8; 32],
        name: Option<String>,
        instanced: KeyInstanced,
        expires: Option<u64>,
    ) -> Self {
        Self {
            sk: None,
            pk: Some(pk),
            name,
            instanced,
            expires,
        }
    }

    /// # `generate(name, time, expires)`
    ///
    /// Generates a new key pair. This is how 99% of secret keys
    /// arrive after initial setup.
    pub fn generate(name: Option<String>, time: Option<u64>, expires: Option<u64>) -> Self {
        let seed: [u8; 32] = rand::random();
        let dalek_sk = ed25519_dalek::SigningKey::from_bytes(&seed);
        let pk = dalek_sk.verifying_key().to_bytes();
        let instanced = if time.is_some() {
            KeyInstanced::Generated(time.unwrap())
        } else {
            KeyInstanced::Unknown
        };
        Self {
            sk: Some(dalek_sk.to_bytes()),
            pk: Some(pk),
            name,
            instanced,
            expires,
        }
    }

    /// # `sign(msg)`
    ///
    /// I mean... it signs over msg using the current `sk`...?
    ///
    pub fn sign(&self, msg: &[u8]) -> [u8; 64] {
        let dalek_sk = ed25519_dalek::SigningKey::from_bytes(&self.sk.unwrap());
        dalek_sk.sign(msg).to_bytes()
    }

    /// # `verify(msg, sig)`
    ///
    /// Verifies a msg against the signature using this `pk`
    ///
    pub fn verify(&self, msg: &[u8], sig: &[u8]) -> bool {
        let dalek_pk = ed25519_dalek::VerifyingKey::from_bytes(&self.pk.unwrap()).unwrap();
        let valid = dalek_pk.verify(msg, &Signature::from_bytes(sig.try_into().unwrap()));
        match valid {
            Ok(_) => true,
            Err(e) => {
                println!("Invalid signature: {:?}", e);
                false
            }
        }
    }

    /// # `to_vec()`
    ///
    /// Returns a CBOR encoded Vec<u8> of the key data
    ///
    pub fn to_vec(&self) -> Vec<u8> {
        let mut output = Vec::new();
        minicbor::encode(self, &mut output).unwrap();
        output
    }

    /// # `from_vec(data)`
    ///
    /// Creates a key structure from the Vec<u8>
    ///
    pub fn from_vec(data: &Vec<u8>) -> Result<Self> {
        Ok(minicbor::decode(data).unwrap())
    }

    /// # `disk_get(path)`
    ///
    /// Gets a key structure from a file on disk
    ///
    pub fn disk_get(path: &String) -> Self {
        let data = fs::read(path).unwrap();
        let key = minicbor::decode(&data).unwrap();
        key
    }

    /// # `disk_put(path)`
    ///
    /// Places a key file in `path`. `path` is a complete file
    /// path.
    ///
    pub fn disk_put(&self, path: PathBuf) -> Result<()> {
        let data = self.to_vec();
        fs::write(path, data)?;
        Ok(())
    }

    /// # `sigil_id()`
    ///
    /// This generates the SigilID from the key.
    /// Used in scope determination and easy identification
    ///
    pub fn sigid_id(&self) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.pk.unwrap());
        let hash = hasher.finalize();
        let crockford = base32::encode(base32::Alphabet::Crockford, &hash.as_bytes()[0..10]);
        let output = format!(
            "{}-{}-{}-{}",
            &crockford[0..4],
            &crockford[4..8],
            &crockford[8..12],
            &crockford[12..16]
        );
        output
    }

    /// # `pretty_format(show_sk)`
    ///
    /// Returns a formatted string showing the contents of the
    /// key structure. `show_sk` = true to show the secret key
    /// in the output [Generally not recommended]
    ///
    pub fn pretty_format(&self, show_sk: bool) -> String {
        let mut output = String::new();

        if self.sk.is_some() && show_sk {
            let sk = engine::general_purpose::URL_SAFE_NO_PAD.encode(self.sk.unwrap());
            output = format!("Secret Key: BASE64({})\n", sk);
        };
        if self.pk.is_some() {
            let pk = engine::general_purpose::URL_SAFE_NO_PAD.encode(self.pk.unwrap());
            output = format!(
                "{}Public Key: BASE64({})\nSigilID: 💠{}\n",
                output,
                pk,
                self.sigid_id()
            );
        };
        output
    }
}
