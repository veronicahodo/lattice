use anyhow::{Result, bail};
use iroh::{EndpointId, PublicKey};
use rand::random;

use crate::{Rhex, context::RhexContext, intent::RhexIntent, signature::RhexSignature};

impl Rhex {
    pub fn build(
        prev: Option<[u8; 32]>,
        scope: String,
        nonce: Option<[u8; 16]>,
        author: PublicKey,
        usher: EndpointId,
        rt: String,
        schema: Option<String>,
        data_hash: Option<[u8; 32]>,
    ) -> Self {
        let nonce = match nonce {
            Some(n) => n,
            None => random(),
        };
        Self {
            magic: Rhex::MAGIC,
            intent: RhexIntent {
                prev,
                scope,
                nonce,
                author,
                usher,
                schema,
                rt,
                data_hash,
            },
            data: vec![],
            context: None,
            sigs: Vec::new(),
            curr: None,
        }
    }

    pub fn build_with(
        intent: &RhexIntent,
        context: &Option<RhexContext>,
        sigs: &Vec<RhexSignature>,
        data: &Option<Vec<u8>>,
    ) -> Rhex {
        Rhex {
            magic: Rhex::MAGIC,
            intent: intent.clone(),
            context: context.clone(),
            sigs: sigs.clone(),
            data: data.clone().unwrap_or_else(|| vec![]),
            curr: None,
        }
    }

    pub fn finalize(&mut self) -> Result<()> {
        if self.context.is_none() {
            bail!("Not final!");
        }
        self.curr = Some(self.calc_curr());
        Ok(())
    }
}
