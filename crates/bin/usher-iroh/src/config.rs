use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct UsherIrohConfig {
    pub config: String,
    pub bind: String,
    pub port: u16,
    pub root: String,
    pub enclave: String,
    pub i_am: String,
    pub scopes: String,
    pub usher_map: String,
    pub trans_registry: String,
    pub trans_store: String,
    pub bootstrap: String,
    pub rebuild: bool,
    pub observer: bool,
    pub verbose: bool,
}

/// # UsherConfigJson
///
/// This the intermediate step in loading of the config file.
/// The file of course is in JSON and we are turning it into a
/// UsherIrohConfig by way of this struct.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsherConfigJson {
    bind: Option<String>,
    port: Option<u16>,
    root: Option<String>,
    enclave: Option<String>,
    i_am: Option<String>,
    scopes: Option<String>,
    usher_map: Option<String>,
    trans_registry: Option<String>,
    trans_store: Option<String>,
    bootstrap: Option<String>,
    rebuild: Option<bool>,
    observer: Option<bool>,
    verbose: Option<bool>,
}

impl UsherIrohConfig {
    pub const DEFAULT_CONFIG: &'static str = "./config.json";
    pub const DEFAULT_BIND: &'static str = "0.0.0.0";
    pub const DEFAULT_PORT: u16 = 1984;
    pub const DEFAULT_ROOT: &'static str = "./";
    pub const DEFUALT_ENCLAVE: &'static str = "./keys/";
    pub const DEFAULT_IAM: &'static str = "./iam.cbor";
    pub const DEFAULT_SCOPES: &'static str = "./scopes/";
    pub const DEFAULT_USHER_MAP: &'static str = "./ushers.cbor";
    pub const DEFAULT_TRANS_REGISTRY: &'static str = "./trans.cbor";
    pub const DEFAULT_TRANS_STORE: &'static str = "./trans/";
    pub const DEFAULT_BOOTSTRAP: &'static str = "./bootstrap/";

    /// # new()
    ///
    /// Makes a new config struct with the defaults
    ///
    pub fn new() -> UsherIrohConfig {
        UsherIrohConfig {
            config: UsherIrohConfig::DEFAULT_CONFIG.to_string(),
            bind: UsherIrohConfig::DEFAULT_BIND.to_string(),
            port: UsherIrohConfig::DEFAULT_PORT,
            root: UsherIrohConfig::DEFAULT_ROOT.to_string(),
            enclave: UsherIrohConfig::DEFUALT_ENCLAVE.to_string(),
            i_am: UsherIrohConfig::DEFAULT_IAM.to_string(),
            scopes: UsherIrohConfig::DEFAULT_SCOPES.to_string(),
            usher_map: UsherIrohConfig::DEFAULT_USHER_MAP.to_string(),
            trans_registry: UsherIrohConfig::DEFAULT_TRANS_REGISTRY.to_string(),
            trans_store: UsherIrohConfig::DEFAULT_TRANS_STORE.to_string(),
            bootstrap: UsherIrohConfig::DEFAULT_BOOTSTRAP.to_string(),
            rebuild: false,
            observer: false,
            verbose: false,
        }
    }

    /// # from_disk(path)
    ///
    /// Loads a JSON file with, in theory, the config values
    ///
    pub fn from_disk(path: String) -> Result<UsherIrohConfig> {
        let data: UsherConfigJson = serde_json::from_slice(&std::fs::read(&path)?)?;
        Ok(UsherIrohConfig {
            config: path,
            bind: data
                .bind
                .unwrap_or_else(|| UsherIrohConfig::DEFAULT_BIND.to_string()),
            port: data.port.unwrap_or_else(|| UsherIrohConfig::DEFAULT_PORT),
            root: data
                .root
                .unwrap_or_else(|| UsherIrohConfig::DEFAULT_ROOT.to_string()),
            enclave: data
                .enclave
                .unwrap_or_else(|| UsherIrohConfig::DEFUALT_ENCLAVE.to_string()),
            i_am: data
                .i_am
                .unwrap_or_else(|| UsherIrohConfig::DEFAULT_IAM.to_string()),
            scopes: data
                .scopes
                .unwrap_or_else(|| UsherIrohConfig::DEFAULT_SCOPES.to_string()),
            usher_map: data
                .usher_map
                .unwrap_or_else(|| UsherIrohConfig::DEFAULT_USHER_MAP.to_string()),
            trans_registry: data
                .trans_registry
                .unwrap_or_else(|| UsherIrohConfig::DEFAULT_TRANS_REGISTRY.to_string()),
            trans_store: data
                .trans_store
                .unwrap_or_else(|| UsherIrohConfig::DEFAULT_TRANS_STORE.to_string()),
            bootstrap: data
                .bootstrap
                .unwrap_or_else(|| UsherIrohConfig::DEFAULT_BOOTSTRAP.to_string()),
            rebuild: data.rebuild.unwrap_or_else(|| false),
            observer: data.observer.unwrap_or_else(|| false),
            verbose: data.verbose.unwrap_or_else(|| false),
        })
    }

    /// # merge(...)
    ///
    /// This takes the command line parameters passed from the Parser
    /// and merges them with a pre-existing set (usually either
    /// `UsherIrohConfig::new()` or `UsherIrohConfig::from_disk(path)`)
    ///
    pub fn merge(
        &mut self,
        config: String,
        bind: Option<String>,
        port: Option<u16>,
        root: Option<String>,
        enclave: Option<String>,
        i_am: Option<String>,
        scopes: Option<String>,
        usher_map: Option<String>,
        trans_registry: Option<String>,
        trans_store: Option<String>,
        bootstrap: Option<String>,
        rebuild: bool,
        observer: bool,
        verbose: bool,
    ) -> Result<()> {
        self.config = config;
        if let Some(b) = bind {
            self.bind = b;
        }
        if let Some(p) = port {
            self.port = p;
        }
        if let Some(r) = root {
            self.root = r;
        }
        if let Some(e) = enclave {
            self.enclave = e;
        }
        if let Some(i) = i_am {
            self.i_am = i;
        }
        if let Some(s) = scopes {
            self.scopes = s;
        }
        if let Some(u) = usher_map {
            self.usher_map = u;
        }
        if let Some(t) = trans_registry {
            self.trans_registry = t;
        }
        if let Some(t) = trans_store {
            self.trans_store = t;
        }
        if let Some(b) = bootstrap {
            self.bootstrap = b;
        }
        if rebuild {
            self.rebuild = true
        };
        if observer {
            self.observer = true
        };
        if verbose {
            self.verbose = true
        };
        Ok(())
    }
}
