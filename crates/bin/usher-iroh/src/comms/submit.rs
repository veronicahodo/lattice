use anyhow::Result;
use iroh::{Endpoint, endpoint::presets};

use crate::{
    comms::chatter::{ChatterRequest, ChatterResponse},
    server::PROTOCOL_ALPN,
};

pub async fn submit(requests: Vec<ChatterRequest>) -> Result<()> {
    let endpoint = Endpoint::bind(presets::N0).await?;

    submit_requests(&endpoint, requests).await?;

    endpoint.close().await;
    Ok(())
}

async fn submit_requests(endpoint: &Endpoint, requests: Vec<ChatterRequest>) -> Result<()> {
    let max_size = 50 * 1024 * 1024;
    for req in requests {
        let connection = match endpoint.connect(req.address, PROTOCOL_ALPN).await {
            Ok(conn) => conn,
            Err(e) => {
                eprintln!("Failed to connec to {}: {e}", req.address);
                continue;
            }
        };

        let (mut send_stream, mut recv_stream) = connection.open_bi().await?;

        send_stream.write_all(&req.to_vec()?).await?;
        send_stream.finish()?;

        let response_bytes = recv_stream.read_to_end(max_size).await?;
        let response: ChatterResponse = cbor2::from_slice(&response_bytes)?;

        println!("Response from {}: {:?}", req.address, response);
    }
    Ok(())
}
