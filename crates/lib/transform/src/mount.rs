use std::collections::HashMap;

use anyhow::Result;

use crate::{descriptor::TransformDescriptor, registry::TransformRegistry};

impl TransformRegistry {
    /// # mount
    /// Mounts an instance of the transform into the interactions map
    ///
    pub fn mount(&mut self, mount: &String, descriptor: TransformDescriptor) -> Result<()> {
        // format our main mounting string
        let mount_point = match descriptor.trigger.0.as_str() {
            "*" => mount,
            _ => &descriptor.trigger.0.replace("<mount>", &mount),
        };
        // Mount the appropriate action set
        let triggers = self
            .triggers
            .entry(descriptor.action.clone())
            .or_insert(HashMap::new());

        // Insert our trigger
        let scope_entry = triggers
            .entry(mount_point.clone())
            .or_insert(HashMap::new());
        let rt_entry = scope_entry
            .entry(descriptor.trigger.1.clone())
            .or_insert(Vec::new());
        rt_entry.push(descriptor.hash);

        Ok(())
    }

    pub fn umount(
        &mut self,
        mount: String,
        rt: String,
        descriptor: TransformDescriptor,
    ) -> Result<()> {
        // Rip the trigger mount out for an instance of the transform
        let triggers = self
            .triggers
            .entry(descriptor.action.clone())
            .or_insert(HashMap::new());
        let scope = triggers.entry(mount).or_insert(HashMap::new());
        let rt = scope.entry(rt).or_insert(Vec::new());
        rt.retain(|&x| x != descriptor.hash);
        Ok(())
    }

    pub fn check_output_paths(
        &self,
        _mount: String,
        _descriptor: TransformDescriptor,
    ) -> Result<bool> {
        // TODO: Make this so it checks that each output of the descriptor
        // is at least
        Ok(true)
    }
}
