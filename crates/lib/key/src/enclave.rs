use std::fs;

use anyhow::{Ok, Result, bail};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};

use crate::{Key, KeyInstanced};

pub struct Enclave {
    path: String,
    pub keys: Vec<[u8; 32]>,
}

impl Enclave {
    /// # new(path)
    /// Creates a new enclave using the path either set or assumed
    /// to be `./keys`.
    ///
    pub fn new(path: Option<&String>) -> Self {
        let default_path = "./keys/".to_string();
        let path = path.clone().unwrap_or(&default_path);

        Self {
            path: path.clone(),
            keys: Vec::new(),
        }
    }

    /// # disk_get(pk)
    /// Pulls a single key from disk.
    ///
    pub fn disk_get(&self, pk: [u8; 32]) -> Result<Key> {
        let data = std::fs::read(format!("{}/{}.key", self.path, hex::encode(&pk))).unwrap();
        let key = Key::from_vec(&data)?;
        Ok(key)
    }

    /// # disk_put(pk)
    /// Puts a single key to disk.
    /// FIXME: This actually does nothing but storing the PK in the
    /// file. I don't know what the hell I was thinking. SKs live on
    /// disk and are loaded as needed, so to "put" to disk would just
    /// be copying a file.
    ///
    pub fn disk_put(&self, pk: [u8; 32]) {
        let key = self.keys.iter().find(|c| **c == pk);
        if key.is_none() {
            return;
        }
        let key = key.unwrap();
        let mut data = Vec::new();
        minicbor::encode(key, &mut data).unwrap();
        let _ = std::fs::write(format!("{}/{}.key", self.path, hex::encode(&pk)), &data);
    }

    /// # populate()
    /// builds the enclave from stored keys on disk
    ///
    pub fn populate(&mut self) -> Result<()> {
        if !fs::exists(self.path.clone())? {
            fs::create_dir(self.path.clone()).unwrap();
        }
        let dir = fs::read_dir(self.path.clone())?;
        for entry in dir {
            let path = entry.unwrap().path();
            if !path.is_file() {
                continue;
            }
            if path.extension().unwrap() != "key" {
                continue;
            }
            let hexstr = path.file_name().unwrap().to_str().unwrap();
            let hex = hexstr.replace(".key", "");

            let pk = hex::decode(hex).unwrap().try_into().unwrap();
            let key = self.disk_get(pk)?;
            self.keys.push(key.pk.unwrap());
        }
        Ok(())
    }

    /// # sign(pk, msg)
    /// Signs `msg` with `pk` and returns the signature.
    ///
    pub fn sign(&self, pk: &[u8; 32], msg: &[u8]) -> Result<[u8; 64]> {
        let key = self.keys.iter().find(|c| *c == pk);
        if key.is_none() {
            return Err(anyhow::anyhow!("Key not found"));
        }

        let key = self.disk_get(*pk)?;
        let sig = key.sign(msg);
        Ok(sig)
    }

    /// # generate(name)
    /// Creates a new key with `name` and returns it.
    ///
    pub fn generate(
        &mut self,
        name: Option<String>,
        time: Option<u64>,
        expires: Option<u64>,
    ) -> Result<Key> {
        let mut key = Key::generate(name, time, expires);
        let mut buf = Vec::new();
        minicbor::encode(&key, &mut buf)?;
        let data = buf;
        std::fs::write(
            format!("{}/{}.key", self.path, hex::encode(&key.pk.unwrap())),
            &data,
        )?;
        key.sk = None;
        self.keys.push(key.pk.unwrap());
        Ok(key)
    }

    /// # show_key(pk, show_sk)
    /// Shows the key data stored in the file system.
    ///
    pub fn show_key(&self, pk: [u8; 32], show_sk: bool) -> Result<String> {
        let key = self.disk_get(pk)?;
        Ok(key.pretty_format(show_sk))
    }

    /// # import(sk, name)
    /// imports a key into the enclave.
    ///
    pub fn import(
        &mut self,
        sk: [u8; 32],
        name: Option<String>,
        time: Option<u64>,
        expires: Option<u64>,
    ) -> Result<Key> {
        let instanced = if time.is_some() {
            KeyInstanced::Imported(time.unwrap())
        } else {
            KeyInstanced::Unknown
        };
        let key = Key::new(sk, name, instanced, expires);
        self.keys.push(key.pk.unwrap());
        let mut buf = Vec::new();
        minicbor::encode(&key, &mut buf)?;
        fs::write(
            format!(
                "{}/{}.key",
                self.path.clone(),
                hex::encode(&key.pk.unwrap())
            ),
            &buf,
        )?;
        Ok(key)
    }

    /// # check_map(self, i_am)
    /// Takes a vec of pks that are local in the IAm map and confirms
    /// they are all there. This really shouldn't be exposed anywhere
    /// but at startup since this could be used to enumerate keys that
    /// we have.
    ///
    pub fn check_map(&self, i_am: Vec<[u8; 32]>) -> Result<()> {
        for key in i_am {
            if !self.keys.contains(&key) {
                let key_base64 = URL_SAFE_NO_PAD.encode(key);
                bail!(format!("Key {} not in local enclave", key_base64));
            }
        }
        Ok(())
    }
}
