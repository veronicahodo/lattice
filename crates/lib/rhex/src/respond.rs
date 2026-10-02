use anyhow::Result;

use crate::{Rhex, data::RhexData};

impl Rhex {
    pub fn reply_to(
        rhex: &Rhex,
        rt: &String,
        schema: &Option<String>,
        data: &RhexData,
    ) -> Result<Self> {
        let mut out = Rhex::new();
        out.intent.scope = rhex.intent.scope.clone();
        out.intent.author = rhex.intent.usher;
        out.intent.usher = rhex.intent.author;
        out.intent.rt = rt.clone();
        out.intent.schema = schema.clone();
        (out.intent.data_hash, out.data) = match data {
            RhexData::None => (None, vec![]),
            _ => (Some(data.get_hash()), data.to_vec()?),
        };
        Ok(out)
    }
}
