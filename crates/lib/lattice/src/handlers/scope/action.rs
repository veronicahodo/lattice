use anyhow::Result;
use scope::rhex::rt_structs::scope::{RtScopeCreate, RtScopeRequest};

pub fn action_scope_create(scope_create: &RtScopeCreate, path: &String) -> Result<()> {
    let path = format!("{}{}/", path, scope_create.scope);
    if !std::fs::exists(&path)? {
        std::fs::create_dir(&path)?
    }
    Ok(())
}

pub fn action_scope_request(_scope_request: &RtScopeRequest) -> Result<()> {
    // Stub.

    // This doesn't need an action at this time... but while I'm in
    // this I might as well do it.
    Ok(())
}
