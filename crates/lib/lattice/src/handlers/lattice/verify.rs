use anyhow::{Result, bail};
use scope::rhex::{
    data::RhexData,
    rt_structs::lattice::{RtLatticeGenesis, RtLatticeManifesto},
};

/// # verify_lattice_genesis(rhex_data)
///
/// This is the basic verify function for `lattice:genesis`
///
pub fn verify_lattice_genesis(rhex_data: &RhexData) -> Result<RtLatticeGenesis> {
    Ok(match rhex_data {
        RhexData::Binary(bin) => RtLatticeGenesis::from_slice(bin)?,
        _ => bail!("`lattice:genesis` is wrong data type (should be binary)"),
    })
}

/// # verify_lattice_manifesto(rhex_data)
///
/// This function will accept `::Binary()` and `::Json` for flexibility.
/// Requires a "text" field for JSON entries.
///
pub fn verify_lattice_manifesto(rhex_data: &RhexData) -> Result<RtLatticeManifesto> {
    Ok(match rhex_data {
        RhexData::Binary(bin) => RtLatticeManifesto::from_slice(bin)?,
        RhexData::Json(json) => serde_json::from_slice::<RtLatticeManifesto>(json)?,
        _ => bail!("`lattice:manifesto` is wrong data type (should be binary or json)"),
    })
}
