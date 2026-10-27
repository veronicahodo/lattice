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
            // *****************************************************
            // **** Author Sign                                 ****
            // *****************************************************
            // Author Sign is ABSOLUTELY FUCKING BROKEN! See FIXME: entries
            // in /usher-iroh/src/author/mod.rs
            ChatterRequestType::AuthorSign { intent, token } => {
                sign_as_author(&intent, &token, &enclave_guard)?
            }
            // *****************************************************
            // **** Usher Sign                                  ****
            // *****************************************************
            ChatterRequestType::UsherSign { intent, sigs } => usher::sign_as_usher(
                &intent,
                &sigs,
                &enclave_guard,
                &iam_guard,
                &trans_reg_guard,
                &lattice_guard,
            )?,
            // *****************************************************
            // **** Quorum Sign                                 ****
            // *****************************************************
            ChatterRequestType::QuorumSign {
                intent,
                context,
                sigs,
            } => quorum::sign_as_quorum(&intent, &context, &sigs, &lattice_guard)?,
            // *****************************************************
            // **** Observer Sign                               ****
            // *****************************************************
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
            // *****************************************************
            // **** Submit                                      ****
            // *****************************************************
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
    }

    Ok(output)
}
