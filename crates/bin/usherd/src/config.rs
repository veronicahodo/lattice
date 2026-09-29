use std::{fs, path::PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsherdConfig {
    /// Root path for the app itself.
    ///
    /// ## Example
    /// ```
    /// config.root_path = "/usr/local/usherd/".to_string();
    /// ```
    ///
    /// ## Default
    /// `./`
    ///
    pub root_path: String,
    /// Enclave path. Technically just the path to a bunch
    /// of key files. This should obviously be well guarded because
    /// the key files contain private keys.
    ///
    /// ## Example
    /// ```
    /// config.enclave = "/path/to/secure/keys/".to_string();
    /// ```
    ///
    /// ## Default
    /// `./keys/` - Bro... just _don't_ leave this as the default for
    /// ANYTHING you care about
    ///
    pub enclave: String,
    /// Path to scope storage. Once again, just a dir of
    /// chain files. Each scope has it's own .rchain file, with root
    /// being `-root-.rchain`
    ///
    /// ## Example
    /// ```
    /// config.scopes = "./scopes/".to_string();
    /// ```
    ///
    /// ## Default
    /// `./scopes/`
    ///
    pub scopes: String,
    /// Path to the file storing the I AM table. Not traditional IAM,
    /// but rather just "I have these keys that I can sign with"
    ///
    /// ## Example
    /// ```
    /// config.i_am = "./i-am.cbor".to_string();
    /// ```
    ///
    /// ## Default
    /// `./i-am.cbor`
    ///
    pub i_am: String,
    /// Path to the file storing the transform registry. This is what
    /// dictates what transforms fire for each scope/rt pair. This
    /// should be well guarded from prying eyes, as modifying would
    /// wildy change the behavior of your usher!
    ///
    /// ## Example
    /// ```
    /// config.transform_registry = "/path/to/secure/trans.registry".to_string();
    /// ```
    ///
    /// ## Default
    /// `./trans_reg.cbor`
    /// Using the default creates a security risk!
    ///
    pub transform_registry: String,
    /// Path to the stored transform packages so they can be loaded
    ///
    /// ## Example
    /// ```
    /// config.transform_store = "/path/to/transform_store".to_string();
    /// ```
    ///
    /// ## Default
    /// `./trans/`
    ///
    pub transform_store: String,
    /// Path to the usher map. This can be generated without a stored
    /// file with the `--no-usher-map` option, where it will just
    /// query for all usher lookups. This is generally taxing on the
    /// whole lattice so we cache our local history.
    ///
    /// ## Example
    /// ```
    /// config.usher_map = "./ushers.cbor".to_string();
    /// ```
    ///
    /// ## Default
    /// `./usher_map.cbor`
    ///
    pub usher_map: String,
    /// Address to bind the server to. This is pretty much just passed
    /// to the server so we must elsewhere check to make sure we can
    /// even bind to that address.
    ///
    /// ## Example
    /// ```
    /// config.bind = "192.168.0.1"
    /// ```
    ///
    /// ## Default
    /// `0.0.0.0` (Listen on all addresses)
    ///
    pub bind: String,
    /// Port number to use for listening. Pretty straight forward.
    ///
    /// ## Example
    /// ```
    /// config.port = 1984;
    /// ```
    ///
    /// ## Default
    /// `1984` 😏
    ///
    pub port: u16,
    /// Do we rebuild the whole lattice from scratch? This takes the
    /// data from the bootstrap path and places it as the starting
    /// point to rebuilding the lattice from the root scope.
    ///
    /// ## Example
    /// ```
    /// config.rebuild = true;
    /// ```
    ///
    /// ## Default
    /// `false` - Obvs... or we'd be rebuilding every time
    ///
    pub rebuild: bool,
    /// What is the path to the rebuild data? What is basically our
    /// "fresh install" of the lattice?
    ///
    /// ## Example
    /// ```
    /// config.bootstrap = "./bootstrap/".to_string();
    /// ```
    ///
    /// ## Default
    /// `./bootstrap/` - Obviously this needs to be a verified
    /// bootstrap from a root scope provider.
    ///
    pub bootstrap: String,
    /// Bog standard verbose command for a CLI app.
    pub verbose: bool,
}

impl UsherdConfig {
    pub fn from_file(path: &PathBuf) -> Result<Self> {
        let file_config: FileConfig = fs::read_to_string(path)
            .ok()
            .and_then(|contents| serde_json::from_str(&contents).ok())
            .unwrap_or_default();
        let root_path = if file_config.root.is_some() {
            tail_slash(file_config.root.unwrap())
        } else {
            "./".to_string()
        };
        let enclave = if file_config.enclave.is_some() {
            tail_slash(file_config.enclave.unwrap())
        } else {
            "./keys/".to_string()
        };
        let scopes = if file_config.scopes.is_some() {
            tail_slash(file_config.scopes.unwrap())
        } else {
            "./scopes/".to_string()
        };
        let i_am = file_config
            .i_am
            .unwrap_or_else(|| "./i_am.cbor".to_string());
        let transform_registry = file_config
            .transform_registry
            .unwrap_or_else(|| "./trans_reg.cbor".to_string());
        let transform_store = if file_config.transform_store.is_some() {
            tail_slash(file_config.transform_store.unwrap())
        } else {
            "./trans/".to_string()
        };
        let usher_map = file_config
            .usher_map
            .unwrap_or_else(|| "./usher_map.cbor".to_string());
        let bind = file_config.bind.unwrap_or_else(|| "0.0.0.0".to_string());
        let port = file_config.port.unwrap_or_else(|| 1984);
        let rebuild = file_config.rebuild.unwrap_or_else(|| false);
        let bootstrap = if file_config.bootstrap.is_some() {
            tail_slash(file_config.bootstrap.unwrap())
        } else {
            "./bootstrap/".to_string()
        };
        let verbose = file_config.verbose.unwrap_or_else(|| false);

        Ok(Self {
            root_path,
            enclave,
            scopes,
            i_am,
            transform_registry,
            transform_store,
            usher_map,
            bind,
            port,
            rebuild,
            bootstrap,
            verbose,
        })
    }

    /// # merge_config(...)
    ///
    /// Merge CLI and file config settings
    ///
    pub fn merge_config(
        self,
        root: Option<String>,
        enclave: Option<String>,
        scopes: Option<String>,
        i_am: Option<String>,
        transform_registry: Option<String>,
        transform_store: Option<String>,
        usher_map: Option<String>,
        bind: Option<String>,
        port: Option<u16>,
        rebuild: Option<bool>,
        bootstrap: Option<String>,
        verbose: bool,
    ) -> Result<Self> {
        Ok(UsherdConfig {
            root_path: root.or(Some(self.root_path)).unwrap(),
            enclave: enclave.or(Some(self.enclave)).unwrap(),
            scopes: scopes.or(Some(self.scopes)).unwrap(),
            i_am: i_am.or(Some(self.i_am)).unwrap(),
            transform_registry: transform_registry
                .or(Some(self.transform_registry))
                .unwrap(),
            transform_store: transform_store.or(Some(self.transform_store)).unwrap(),
            usher_map: usher_map.or(Some(self.usher_map)).unwrap(),
            bind: bind.or(Some(self.bind)).unwrap(),
            port: port.or(Some(self.port)).unwrap(),
            rebuild: rebuild.is_some(),
            bootstrap: bootstrap.or(Some(self.bootstrap)).unwrap(),
            verbose,
        })
    }
}

fn tail_slash(path: String) -> String {
    if path.ends_with("/") {
        path
    } else {
        format!("{}/", path)
    }
}

/// This is just here for JSON translation.
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct FileConfig {
    root: Option<String>,
    enclave: Option<String>,
    scopes: Option<String>,
    i_am: Option<String>,
    transform_registry: Option<String>,
    transform_store: Option<String>,
    usher_map: Option<String>,
    bind: Option<String>,
    port: Option<u16>,
    rebuild: Option<bool>,
    bootstrap: Option<String>,
    verbose: Option<bool>,
}
