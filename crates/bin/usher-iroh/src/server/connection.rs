use anyhow::Result;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use iroh::endpoint::Incoming;
use key::Key;

use crate::{
    config::UsherIrohConfig,
    server::{appstate::AppState, process::process_stream},
};

pub async fn handle_connection(
    config: &UsherIrohConfig,
    appstate: &mut AppState,
    incoming: Incoming,
) -> Result<()> {
    let accepting = incoming.accept()?;
    let connection = accepting.await?;
    let node_id = connection.remote_id();
    let base64 = URL_SAFE_NO_PAD.encode(node_id.as_bytes());
    println!(
        "Accepted connection from: 💠{} ({})",
        Key::sigil_id(node_id.as_bytes()),
        base64
    );

    while let Ok((mut send_stream, mut recv_stream)) = connection.accept_bi().await {
        if let Err(e) = process_stream(&mut send_stream, &mut recv_stream, appstate, &config).await
        {
            eprintln!("Stream error: {:?}", e);
        }
    }
    Ok(())
}
