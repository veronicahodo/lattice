use std::{fs, path::PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsherdConfig {
    /// Root path for the app itself. Default: ./
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
    /// `./trans.registry`
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
    /// `./trans_store/`
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
    /// `./ushers.cbor`
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
        let bind = if file_config.bind.is_some() {
            file_config.bind.unwrap()
        } else {
            "0.0.0.0".to_string()
        };
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
        let bootstrap = if file_config.bootstrap.is_some() {
            tail_slash(file_config.bootstrap.unwrap())
        } else {
            "./bootstrap/".to_string()
        };
        let verbose = if file_config.verbose.is_some() {
            file_config.verbose.unwrap()
        } else {
            false
        };

        Ok(Self {
            root_path,
            enclave,
            scopes,
            i_am: file_config
                .i_am
                .unwrap_or_else(|| "./i-am.cbor".to_string()),
            transform_registry: file_config
                .transform_registry
                .unwrap_or_else(|| "./trans.registry".to_string()),
            transform_store: file_config
                .transform_store
                .unwrap_or_else(|| "./trans_store/".to_string()),
            usher_map: file_config
                .usher_map
                .unwrap_or_else(|| "./ushers.cbor".to_string()),
            bind,
            port: file_config.port.unwrap_or_else(|| 1984),
            rebuild: file_config.rebuild.unwrap_or_else(|| false),
            bootstrap,
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
