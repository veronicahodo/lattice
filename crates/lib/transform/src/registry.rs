use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::Result;
use libloading::{Library, Symbol};

use crate::{
    context::TransformContext, descriptor::DescriptorAction, entry::TranformEntry,
    loaded::LoadedTransform, package::TransformPackage,
};

pub type Registry = HashMap<[u8; 32], LoadedTransform>;
pub type MountingPoint = HashMap<String, Vec<[u8; 32]>>;
pub type Mountings = HashMap<String, MountingPoint>;
pub type Triggers = HashMap<DescriptorAction, Mountings>;

pub struct TransformRegistry {
    pub registry: Registry,
    pub triggers: Triggers,
}

impl TransformRegistry {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
            triggers: HashMap::new(),
        }
    }

    pub fn add_transform(&mut self, name: &String, mount: &String, path: &String) -> Result<()> {
        // Load from disk first
        let full_path = format!("{}{}.trans", path, name);
        let pkg = TransformPackage::disk_get(full_path)?;

        // Verify hash
        let hash = blake3::hash(&pkg.binary);
        if hash.as_bytes() != &pkg.descriptor.hash {
            anyhow::bail!("Failed hash verification loading transform!");
        };

        // clone to cache in case the package disappears
        let path =
            PathBuf::from(format!("./trans_cache/{}.dylib", hex::encode(hash.as_bytes())).as_str());
        if !path.exists() {
            fs::create_dir_all("./trans_cache/")?;
            fs::write(&path, &pkg.binary)?;
        }

        // dlopen
        let lib = unsafe { Library::new(&path) }?;

        // Load the symbol
        let symbol: Symbol<*const TranformEntry> = unsafe { lib.get(b"RHEX_TRANSFORM")? };

        // Safety: symbol points into the loaded library which we are running
        let entry_ref: &'static TranformEntry = unsafe { &**symbol };

        // Insert into the registry
        self.registry.insert(
            hash.as_bytes().clone(),
            LoadedTransform {
                descriptor: pkg.descriptor.clone(),
                entry: entry_ref,
                library: lib,
            },
        );

        // Mount the trigger
        self.mount(mount, pkg.descriptor.clone())?;

        Ok(())
    }

    pub fn remove_transform(&mut self, hash: [u8; 32]) {
        self.registry.remove(&hash);
        // TODO: make it remove the hooks too
    }

    /// # run
    /// Executes a single transform with the input already CBORed
    ///
    pub fn run(&self, hash: &[u8; 32], input: &[u8]) -> Result<Vec<u8>> {
        let transform = self.registry.get(hash);
        if transform.is_none() {
            anyhow::bail!("Transform not found");
        }
        let transform = transform.unwrap();
        let entry = transform.entry;
        let mut input = TransformContext {
            input: input,
            output: &mut None,
            diag: &mut None,
        };
        println!("Firing transform: {}", transform.descriptor.name);
        let result = (entry.entry)(&mut input);
        println!("Result: {:?}", result);

        let output = input.output.clone();
        let _diag = input.diag.clone();
        // TODO: Figure out how/what we are doing with diag.

        Ok(output.unwrap())
    }
}
