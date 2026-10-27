use std::path::PathBuf;

use anyhow::{Result, bail};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use comms::chatter::{ChatterRequest, ChatterRequestType};
use iroh::PublicKey;
use scope::rhex::{Rhex, signature::RhexSignatureType};

pub async fn submit(rhex: String, address: Option<String>, request: Option<String>) -> Result<()> {
    let version: u16 = 0;
    // FIXME: Should obviously check to see if the file is actually
    // there.
    println!("Opening {}...", rhex);
    let rhex: Rhex = cbor2::from_slice(&std::fs::read(PathBuf::from(rhex))?)?;
    println!("We good?");
    let address = string_to_address(&rhex, address)?;

    let request = if let Some(req) = request {
        match req.as_str() {
            "author" => ChatterRequestType::AuthorSign {
                intent: rhex.intent.clone(),
                token: vec![],
            },
            "usher" => ChatterRequestType::UsherSign {
                intent: rhex.intent.clone(),
                sigs: rhex.sigs.clone(),
            },
            "quorum" => ChatterRequestType::QuorumSign {
                intent: rhex.intent.clone(),
                context: rhex.context.clone().unwrap(),
                sigs: rhex.sigs.clone(),
            },
            "observer" => {
                let time = time::get_current_time();
                let hash = rhex.get_hash(RhexSignatureType::Observer(time));
                ChatterRequestType::ObserverSign(hash)
            }
            "submit" => ChatterRequestType::Submit(rhex.clone()),
            _ => {
                // Some sort of fucking error... I dunno. I'm tired
                bail!("???")
            }
        }
    } else {
        // TODO: Guess on state based on the rhex.
        bail!("???x2")
    };
    let chatter = ChatterRequest {
        version,
        address,
        request,
    };
    let response = comms::submit::submit(vec![chatter]).await?;
    if response.len() > 1 {
        // Should error here, because a single request should = a single
        // response. Response includes `payload` that has the results
        // of any additional data.
    }
    Ok(())
}

fn string_to_address(rhex: &Rhex, string: Option<String>) -> Result<PublicKey> {
    Ok(if string.is_some() {
        PublicKey::from_bytes(&URL_SAFE_NO_PAD.decode(string.unwrap())?.try_into().unwrap())?
    } else {
        rhex.intent.usher.clone()
    })
}
