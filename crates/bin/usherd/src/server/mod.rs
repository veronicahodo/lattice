use std::sync::Arc;
use tokio::sync::RwLock;

use anyhow::Result;
use futures::{SinkExt, StreamExt};
use iam::IAm;
use lattice::{
    Rhex,
    usher::{self, UsherMap, map},
};
use transform::registry::TransformRegistry;

use crate::{
    config::UsherdConfig,
    receive::{ReceiveStatus, receive},
    server::{
        appstate::AppState,
        init::{init_enclave, init_lattice, init_resource},
    },
};

pub mod appstate;
pub mod init;

pub async fn run(config: UsherdConfig) -> Result<()> {
    // Set up the connection settings
    let addr = format!("{}:{}", config.bind, config.port);

    // 1. Initialize Registries and Services
    let trans_registry = init_resource(
        &config.transform_registry,
        "transform registry",
        config.verbose,
        || TransformRegistry::new().to_file(&config.transform_registry),
        || TransformRegistry::from_file(&config.transform_registry, &config.transform_store),
    )?;

    let lattice = init_lattice(&config, &trans_registry)?;

    let usher_map = init_resource(
        &config.usher_map,
        "usher map",
        config.verbose,
        || Ok(map::disk_to(&config.usher_map, UsherMap::new())),
        || Ok(usher::map::disk_from(&config.usher_map)),
    )?;

    let i_am = init_resource(
        &config.i_am,
        "I Am",
        config.verbose,
        || IAm::new().disk_to(&config.i_am),
        || IAm::disk_from(&config.i_am),
    )?;

    let enclave = init_enclave(&config, &i_am)?;

    // 2. Bundle into Shared State
    let state = AppState {
        lattice: Arc::new(RwLock::new(lattice)),
        trans_registry: Arc::new(RwLock::new(trans_registry)),
        iam: Arc::new(RwLock::new(i_am)),
        enclave: Arc::new(RwLock::new(enclave)),
        usher_map: Arc::new(RwLock::new(usher_map)),
    };

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("🟢 Server listening on {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let config = config.clone();
        let state = state.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, config, state).await {
                eprintln!("Connection error: {:?}", e);
            }
        });
    }
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    config: UsherdConfig,
    state: AppState,
) -> Result<()> {
    // We use LengthDelimitedCodec so we don't have to worry about
    // TCP fragmenting our CBOR blobs.
    let mut framed =
        tokio_util::codec::Framed::new(stream, tokio_util::codec::LengthDelimitedCodec::new());

    while let Some(request_result) = framed.next().await {
        let bytes = request_result?;
        let mut output = Vec::new();

        // Decode using minicbor
        let rhex_list: Vec<Rhex> = minicbor::decode(&bytes)?;
        println!("Received {} items", rhex_list.len());

        {
            // Set the locks. Ideally we should kick through this fast, unless
            // theres a bunch of transforms/a few slow transforms.
            //
            // ...fuck. I just realized when querying for remote data we're
            // gonna be stalled out here. Bruh!
            let mut lattice_guard = state.lattice.write().await;
            let mut trans_registry_guard = state.trans_registry.write().await;
            let mut iam_guard = state.iam.write().await;
            let mut enclave_guard = state.enclave.write().await;
            let mut usher_map_guard = state.usher_map.write().await;

            for rhex in &rhex_list {
                // Append rhex here
                let receive_output = receive(
                    &config,
                    rhex,
                    &mut trans_registry_guard,
                    &mut lattice_guard,
                    &mut iam_guard,
                    &mut enclave_guard,
                    &mut usher_map_guard,
                )?;

                match receive_output.0 {
                    ReceiveStatus::FailedValidation(_) | ReceiveStatus::MissingSignature(_) => {
                        // TODO: build failed validation R⬢ to return to client
                    }
                    // success means we have output from the append process
                    // already to go, so we just have to attach it to the
                    // output.
                    ReceiveStatus::Success => {
                        if let Some(out) = receive_output.1 {
                            output.extend(out);
                        }
                    }
                }
            }
        } // Drop locks here

        let mut response = Vec::new();
        minicbor::encode(&output, &mut response)?;

        // 4. Send back through the frame
        framed.send(response.into()).await?;
    }

    Ok(())
}
