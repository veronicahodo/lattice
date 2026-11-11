use anyhow::Result;
use rhex::Rhex;

pub fn action_request_rhex(_rhex: &Rhex) -> Result<()> {
    // Another stub. This could be where we execute the request since
    // this technically would occur at "append", but since we don't
    // store these typically it just fires and drops the request
    Ok(())
}
