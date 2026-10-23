use anyhow::Result;
use iroh::endpoint::{RecvStream, SendStream};
use lattice::Rhex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::{
    comms::chatter::ChatterRequest, config::UsherIrohConfig, process, server::appstate::AppState,
};

pub async fn process_stream(
    send: &mut SendStream,
    recv: &mut RecvStream,
    appstate: &mut AppState,
    config: &UsherIrohConfig,
) -> Result<()> {
    // Length-prefixed framing
    let len = recv.read_u32().await? as usize;
    let mut buffer = vec![0u8; len];
    recv.read_exact(&mut buffer).await?;

    let requests: Vec<ChatterRequest> = cbor2::from_slice(&buffer)?;
    println!("Received {} R⬢ requests", requests.len());

    let out_intents = process::process_rhex(requests, config, appstate).await?;

    // stub
    let responses: Vec<Rhex> = Vec::new();

    let response_bytes = cbor2::to_canonical_vec(&responses)?;

    // Write length-prefixed response back
    send.write_u32(response_bytes.len() as u32).await?;
    send.write_all(&response_bytes).await?;
    send.finish()?;

    Ok(())
}
