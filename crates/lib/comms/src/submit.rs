use anyhow::Result;
use iroh::{Endpoint, endpoint::presets};
use key::Key;

use crate::{
    COMM_PROTOCOL,
    chatter::{ChatterRequest, ChatterResponse},
};

pub async fn submit(requests: Vec<ChatterRequest>) -> Result<Vec<ChatterResponse>> {
    let endpoint = Endpoint::bind(presets::N0).await?;

    let output = submit_requests(&endpoint, requests).await?;

    endpoint.close().await;
    Ok(output)
}

async fn submit_requests(
    endpoint: &Endpoint,
    requests: Vec<ChatterRequest>,
) -> Result<Vec<ChatterResponse>> {
    let max_size = 50 * 1024 * 1024;
    let mut responses = Vec::new();
    for req in requests {
        let connection = match endpoint.connect(req.address, COMM_PROTOCOL).await {
            Ok(conn) => conn,
            Err(e) => {
                eprintln!("Failed to connect to {}: {e}", req.address);
                continue;
            }
        };

        let (mut send_stream, mut recv_stream) = connection.open_bi().await?;
        let data = cbor2::to_canonical_vec(&vec![req.clone()])?;
        send_stream.write_all(&data).await?;
        send_stream.finish()?;

        let response_bytes = recv_stream.read_to_end(max_size).await?;
        let response: Vec<ChatterResponse> = cbor2::from_slice(&response_bytes)?;
        responses.extend(response.clone());

        println!(
            "Response from 💠{}: {:?}",
            Key::sigil_id(req.address.as_bytes()),
            response
        );
    }
    Ok(responses)
}
