use std::path::PathBuf;

use anyhow::{Result, bail};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use comms::chatter::{ChatterRequest, ChatterRequestType, ChatterResponseType};
use iroh::PublicKey;
use key::Key;
use scope::rhex::{Rhex, signature::RhexSignatureType};

pub async fn submit(rhex: String, address: Option<String>, request: Option<String>) -> Result<()> {
    let version: u16 = 0;
    // FIXME: Should obviously check to see if the file is actually
    // there.
    println!("Opening {}...", rhex);
    let rhex_out = rhex.clone();
    let mut rhex: Rhex = cbor2::from_slice(&std::fs::read(PathBuf::from(rhex))?)?;
    let address = string_to_address(&rhex, address)?;

    let request_type = if let Some(req) = request.clone() {
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
            "submit" => ChatterRequestType::Submit(finalize(&rhex)?),
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
        request: request_type,
    };
    let response = comms::submit::submit(vec![chatter]).await?;
    if response.len() > 1 {
        // Should error here, because a single request should = a single
        // response. Response includes `payload` that has the results
        // of any additional data.
    }
    match request.unwrap().as_str() {
        "author" => match &response[0].response {
            ChatterResponseType::AuthorSignResponse { intent_hash, sig } => {
                if intent_hash.clone() != rhex.intent.get_hash() {
                    bail!("Got the sig for the wrong R⬢? How does that even happen?!");
                }
                let author_sig = URL_SAFE_NO_PAD.encode(sig.sig.clone());
                println!(
                    "Received author sig: 💠{} ({})",
                    Key::sigil_id(&sig.pk.key_bytes_to_publickey()),
                    author_sig
                );
                rhex.sigs.push(sig.clone());
                rhex.single_disk_put(&rhex_out);
                println!("Added sig and saved!");
            }
            ChatterResponseType::Rejected(status) => {
                for s in status {
                    println!("Rejected because: {:?}", s);
                }
            }
            _ => {
                println!("{:?}", response)
            }
        },
        "usher" => match &response[0].response {
            ChatterResponseType::UsherSignResponse {
                usher_hash,
                context,
                sig,
            } => {
                let usher_sig = URL_SAFE_NO_PAD.encode(sig.sig.clone());
                println!(
                    "Received usher sig: 💠{} ({})",
                    Key::sigil_id(&sig.pk.key_bytes_to_publickey()),
                    usher_sig
                );
                rhex.context = Some(context.clone());
                rhex.sigs.push(sig.clone());
                let usher_calc = rhex.get_hash(RhexSignatureType::Usher);
                if usher_hash != &usher_calc {
                    bail!("Wrong hash for usher sig!");
                }
                rhex.single_disk_put(&rhex_out);
                println!("Added sig and saved!");
            }
            _ => println!("{:?}", response),
        },
        "quorum" => match &response[0].response {
            ChatterResponseType::QuorumSignResponse { sig_hash, sig } => {
                let mut quorum_rhex =
                    Rhex::build_with(&rhex.intent, &rhex.context, &rhex.sigs, &None);
                let quorum_time = match sig.t {
                    RhexSignatureType::Quorum(t) => t,
                    _ => bail!("Crap signature?"),
                };
                let hash = rhex.get_hash(RhexSignatureType::Quorum(quorum_time));
                if &hash != sig_hash {
                    bail!("Hashes don't match for quorum");
                };
                quorum_rhex.sigs.push(sig.clone());
                quorum_rhex.single_disk_put(&rhex_out);
                println!("Added sig @ {} and saved!", quorum_time);
            }
            _ => println!("{:?}", response),
        },
        _ => {}
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

fn finalize(rhex: &Rhex) -> Result<Rhex> {
    let mut r = rhex.clone();
    r.sort_sigs();
    let iroh = r.to_iroh()?;
    let hash = iroh.to_hash()?;
    r.curr = Some(hash.as_bytes().clone());
    Ok(r)
}
