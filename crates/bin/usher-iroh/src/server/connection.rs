use anyhow::Result;
use iroh::endpoint::Incoming;

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
    println!("Accepted connection from: {}", node_id);

    while let Ok((mut send_stream, mut recv_stream)) = connection.accept_bi().await {
        if let Err(e) = process_stream(&mut send_stream, &mut recv_stream, appstate, &config).await
        {
            eprintln!("Stream error: {:?}", e);
        }
    }
    Ok(())
}
