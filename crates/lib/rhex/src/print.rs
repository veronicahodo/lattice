use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};

use crate::{Rhex, data::RhexData};

impl Rhex {
    pub fn pretty_print(&self) -> String {
        let magic = match self.magic.as_slice() {
            b"RHEX\x00\x01" => "V1",
            b"RHEX\x00\x02" => "V2",
            _ => "Unknown",
        };
        let prev_hash = match self.intent.prev {
            Some(prev) => URL_SAFE_NO_PAD.encode(prev),
            None => "None".to_string(),
        };
        let scope = match self.intent.scope.as_str() {
            "" => "(root)".to_string(),
            _ => self.intent.scope.clone(),
        };
        let author = URL_SAFE_NO_PAD.encode(self.intent.author);
        let usher = URL_SAFE_NO_PAD.encode(self.intent.usher);
        let schema = match &self.intent.schema {
            Some(schema) => schema,
            None => &"None".to_string(),
        };

        let mut sigs = Vec::new();
        for sig in self.sigs.iter() {
            sigs.push(sig.print());
        }
        let sigs = format!("\n\t\t{}", sigs.join("\n\t\t"));
        let curr = match self.curr {
            Some(curr) => URL_SAFE_NO_PAD.encode(curr),
            None => "None".to_string(),
        };
        let data_hash = match self.intent.data_hash {
            Some(hash) => URL_SAFE_NO_PAD.encode(hash),
            None => "None".to_string(),
        };
        let data: RhexData = cbor2::from_slice(&self.data).unwrap();
        let context = if self.context.is_none() {
            "None".to_string()
        } else {
            let spacial = match &self.clone().context.unwrap().s {
                Some(s) => format!(
                    "\"{}\": {}",
                    s.s_ref,
                    URL_SAFE_NO_PAD.encode(s.s_data.clone())
                ),
                None => "None".to_string(),
            };
            format!(
                "{{
            \t\tat: {}
            \t\ts: {}
            \t}}",
                &self.context.clone().unwrap().at,
                spacial
            )
        };

        let output = format!(
            "Rhex: {{
    \tmagic: {},
    \tintent: {{
    \t\tprev: {},
    \t\tscope: {},
    \t\tauthor: {},
    \t\tusher: {},
    \t\trt: {},
    \t\tschema: {},
    \t\tdata_hash: {},
    \t}},
    \tdata: {},
    \tcontext: {}
    \tsigs: {},
    \tcurr: {},
}}",
            magic,
            prev_hash,
            scope,
            author,
            usher,
            self.intent.rt,
            schema,
            data_hash,
            data.print(),
            context,
            sigs,
            curr
        );
        output
    }
}
