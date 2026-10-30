use std::{fs, path::PathBuf};

use anyhow::Result;
use base64::{Engine, engine};
use ed25519_dalek::{Signature, Signer, Verifier};
use iroh::{PublicKey, SecretKey};
use serde::{Deserialize, Serialize};

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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Key {
    pub sk: Option<SecretKey>,
    pub pk: Option<PublicKey>,
    pub name: Option<String>,
    pub instanced: KeyInstanced,
    pub expires: Option<u64>,
}

/// Enum of possible enclave entryways
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyInstanced {
    Unknown,
    Generated(u64),
    Imported(u64),
}

impl Key {
    /// # `new(sk, name, instanced, expires`
    ///
    /// Creates a new key from `sk` provided
    ///
    pub fn new(
        sk: &SecretKey,
        name: &Option<String>,
        instanced: &KeyInstanced,
        expires: &Option<u64>,
    ) -> Self {
        let dalek_sk = ed25519_dalek::SigningKey::from_bytes(&sk.to_bytes());
        let pk = dalek_sk.verifying_key().to_bytes();
        Self {
            sk: Some(sk.clone()),
            pk: Some(PublicKey::from_bytes(&pk).unwrap()),
            name: name.clone(),
            instanced: instanced.clone(),
            expires: expires.clone(),
        }
    }

    /// # `new_pk(pk, name, instanced, expires)`
    ///
    /// Creates just a public key entry. These are generally not stored,
    /// but used for verifying signatures.
    ///
    pub fn new_pk(
        pk: PublicKey,
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
            sk: Some(SecretKey::from_bytes(&dalek_sk.to_bytes())),
            pk: Some(PublicKey::from_bytes(&pk).unwrap()),
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
        let dalek_sk = ed25519_dalek::SigningKey::from_bytes(&self.sk.clone().unwrap().to_bytes());
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
    pub fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(cbor2::to_canonical_vec(self)?)
    }

    /// # `from_vec(data)`
    ///
    /// Creates a key structure from the Vec<u8>
    ///
    pub fn from_slice(data: &Vec<u8>) -> Result<Self> {
        Ok(cbor2::from_slice(data)?)
    }

    /// # `disk_get(path)`
    ///
    /// Gets a key structure from a file on disk
    ///
    pub fn disk_get(path: &String) -> Self {
        let data = fs::read(path).unwrap();
        let key = cbor2::from_slice(&data).unwrap();
        key
    }

    /// # `disk_put(path)`
    ///
    /// Places a key file in `path`. `path` is a complete file
    /// path.
    ///
    pub fn disk_put(&self, path: PathBuf) -> Result<()> {
        let data = self.to_vec()?;
        fs::write(path, data)?;
        Ok(())
    }

    /// # `sigil_id()`
    ///
    /// This generates the SigilID from the key.
    /// Used in scope determination and easy identification
    ///
    pub fn to_sigid_id(&self) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.pk.unwrap().to_vec());
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

    pub fn sigil_id(key: &[u8; 32]) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(key);
        let hash: [u8; 32] = hasher.finalize().into();
        let crockford = base32::encode(base32::Alphabet::Crockford, &hash);
        format!(
            "{}-{}-{}-{}",
            &crockford[0..4],
            &crockford[4..8],
            &crockford[8..12],
            &crockford[12..16]
        )
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
            let sk = engine::general_purpose::URL_SAFE_NO_PAD
                .encode(self.sk.clone().unwrap().to_bytes());
            output = format!("Secret Key: BASE64({})\n", sk);
        };
        if self.pk.is_some() {
            let pk = engine::general_purpose::URL_SAFE_NO_PAD.encode(self.pk.unwrap().as_bytes());
            output = format!(
                "{}Public Key: BASE64({})\nSigilID: 💠{}\n",
                output,
                pk,
                self.to_sigid_id()
            );
        };
        output
    }
}
