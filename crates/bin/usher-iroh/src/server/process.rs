use anyhow::Result;
use comms::chatter::ChatterRequest;
use iroh::endpoint::{RecvStream, SendStream};

use crate::{config::UsherIrohConfig, process, server::appstate::AppState};

pub async fn process_stream(
    send: &mut SendStream,
    recv: &mut RecvStream,
    appstate: &mut AppState,
    config: &UsherIrohConfig,
) -> Result<()> {
    let max_size = 50 * 1024 * 1024;

    // Read all incoming bytes until the client calls finish()
    let buffer = recv.read_to_end(max_size).await?;

    let requests: Vec<ChatterRequest> = cbor2::from_slice(&buffer)?;
    println!("Received {} R⬢ requests", requests.len());

    let response = match process::process_rhex(requests, config, appstate).await {
        Ok(intents) => intents,
        Err(e) => {
            eprintln!("Processing error: {e}");
            return Err(e.into());
        }
    };
    let response_bytes = cbor2::to_canonical_vec(&response)?;

    // Write response and finish (no length prefix needed)
    send.write_all(&response_bytes).await?;
    send.finish()?;

    Ok(())
}
