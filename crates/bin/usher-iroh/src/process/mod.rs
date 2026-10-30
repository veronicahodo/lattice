use anyhow::Result;
use comms::chatter::{ChatterRequest, ChatterRequestType, ChatterResponse, ChatterResponseType};

use crate::{
    append,
    author::sign_as_author,
    config::UsherIrohConfig,
    observe, quorum,
    server::appstate::AppState,
    usher::{self},
};

pub mod action;
pub mod verify;
pub mod walk;

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
            } => quorum::sign_as_quorum(
                &c.address,
                &enclave_guard,
                &intent,
                &context,
                &sigs,
                &lattice_guard,
            )?,
            // *****************************************************
            // **** Observer Sign                               ****
            // *****************************************************
            ChatterRequestType::ObserverSign(bin) => {
                let key = c.address.clone();
                observe::sign_as_observer(&enclave_guard, &key, &bin)?
            }
            // *****************************************************
            // **** Submit                                      ****
            // *****************************************************
            ChatterRequestType::Submit(r) => {
                append::submit_rhex(config, &r, &mut lattice_guard, &trans_reg_guard).await?
            }
        };

        output.push(reponse);
    }

    Ok(output)
}
