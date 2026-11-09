use anyhow::{Result, bail};
use iroh::PublicKey;
use rhex::{
    Rhex,
    data::RhexData,
    intent::RhexIntent,
    rt_structs::scope::{RtScopeCreate, RtScopeRequest},
};

use crate::Scope;

impl Scope {
    pub fn process_scope_request(&self, rhex: &Rhex, me: &PublicKey) -> Result<Rhex> {
        let data: RhexData = cbor2::from_slice(&rhex.data)?;
        let request = match data {
            RhexData::Binary(bin) => RtScopeRequest::from_slice(&bin)?,
            _ => bail!("Scope request is wrong data type"),
        };

        let scope_create_data = RtScopeCreate {
            scope: request.scope.clone(),
            creator: rhex.intent.author.clone(),
            ushers: request.ushers.clone(),
        };
        let data = RhexData::Binary(scope_create_data.to_vec()?);

        let mut intent = RhexIntent::new();
        intent.scope = request.scope.clone();
        intent.prev = rhex.curr;
        intent.author = me.clone();
        intent.usher = me.clone();
        intent.schema = Some("rhex://schema.scope.create/@0".to_string());
        intent.rt = "scope:create".to_string();
        intent.data_hash = Some(data.get_data_hash());

        let out_rhex = Rhex::build_with(&intent, &None, &vec![], &Some(data.to_vec()?));

        Ok(out_rhex)
    }
}
