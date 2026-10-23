use anyhow::Result;
use lattice::rhex::signature::{AgileKey, RhexSigAlgo, RhexSignature, RhexSignatureType};
use time::MicroMarks;

use crate::{
    author::sign_as_author,
    comms::chatter::{ChatterRequest, ChatterRequestType, ChatterResponse, ChatterResponseType},
    config::UsherIrohConfig,
    quorum,
    server::appstate::AppState,
    usher::{self},
};

pub async fn process_rhex(
    chatter: Vec<ChatterRequest>,
    config: &UsherIrohConfig,
    appstate: &mut AppState,
) -> Result<Vec<ChatterResponse>> {
    let mut output: Vec<ChatterResponse> = Vec::new();

    // Set locks
    let mut lattice_guard = appstate.lattice.write().await;
    let mut trans_reg_guard = appstate.trans_registry.write().await;
    let mut iam_guard = appstate.iam.write().await;
    let mut enclave_guard = appstate.enclave.write().await;
    let mut usher_map_guard = appstate.usher_map.write().await;

    for c in chatter {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let reponse = match c.request {
            // Author Sign is ABSOLUTELY FUCKING BROKEN! See FIXME: entries
            // in /usher-iroh/src/author/mod.rs
            ChatterRequestType::AuthorSign { intent, token } => {
                sign_as_author(&intent, &token, &enclave_guard)?
            }
            ChatterRequestType::UsherSign { intent, sigs } => usher::sign_as_usher(
                &intent,
                &sigs,
                &enclave_guard,
                &iam_guard,
                &trans_reg_guard,
                &lattice_guard,
            )?,
            ChatterRequestType::QuorumSign {
                intent,
                context,
                sigs,
            } => quorum::sign_as_quorum(&intent, &context, &sigs)?,
            ChatterRequestType::ObserverSign(bin) => {
                let response = ChatterResponse {
                    version: 0,
                    response: ChatterResponseType::ObserverSignResponse(RhexSignature {
                        pk: AgileKey {
                            algo: RhexSigAlgo::Ed25519,
                            key_bytes: vec![],
                        },
                        sig: vec![],
                        t: RhexSignatureType::Observer(time.as_micromarks()),
                    }),
                    payload: vec![],
                };
                response
            }
            ChatterRequestType::Submit(r) => {
                let response = ChatterResponse {
                    version: 0,
                    response: ChatterResponseType::Accepted([0u8; 32]),
                    payload: vec![],
                };
                response
            }
        };

        output.push(reponse);

        /*let outbound: Vec<Rhex> = match stage {
            0 => {
                // Alright... well, we have zero sigs, which sounds
                // like we might be a remote node looking to get the
                // "home base" to sign off on it.

                // ... but for now we just reject
                let out_rhex = bps_response(
                    &r,
                    &UsherAction::Rejected,
                    &vec![CheckStatus::SignatureInvalid(0)],
                    &enclave_guard,
                    &None,
                    0,
                )?;
                vec![out_rhex]
            }
            1 => {
                // SHOULD be an author signed R⬢
                // This could be a multitude of things, but they are usually
                // one off requests OR the beginning of permanent submission
                let r_out = sign_as_usher(
                    &r,
                    &enclave_guard,
                    &iam_guard,
                    &trans_reg_guard,
                    &lattice_guard,
                )?;
                r_out.unwrap_or_else(|| vec![])
            }
            2 => {
                // Looking for quorum
                let r_out = sign_as_quorum(
                    &r,
                    &enclave_guard,
                    &trans_reg_guard,
                    &iam_guard,
                    &lattice_guard,
                )?;
                r_out.unwrap_or_else(|| vec![])
            }
            3.. => {
                // Appending
                vec![]
            }
        };
        output.extend(outbound);*/
    }

    Ok(output)
}
