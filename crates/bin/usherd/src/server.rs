use std::sync::Arc;

use anyhow::Result;
use futures::{SinkExt, StreamExt};
use iam::IAm;
use key::enclave::Enclave;
use lattice::{
    Lattice, Rhex,
    usher::{self, UsherMap, map},
};
use tokio::sync::RwLock;
use transform::registry::TransformRegistry;

use crate::{
    config::UsherdConfig,
    rebuild,
    receive::{ReceiveStatus, receive},
};

pub async fn run(config: UsherdConfig) -> Result<()> {
    // Set up the connection settings
    let addr = format!("{}:{}", config.bind, config.port);

    // Load the transform registry from file
    if config.verbose {
        print!(
            "Loading transform registry from {}...",
            config.transform_registry
        );
    }

    // If the registry file doesn't exist, create it.
    if !std::fs::exists(&config.transform_registry)? {
        if config.verbose {
            print!("rebuilding...");
        }
        let tr = TransformRegistry::new();
        tr.to_file(&config.transform_registry)?;
    }

    let trans_registry =
        TransformRegistry::from_file(&config.transform_registry, &config.transform_store)?;
    if config.verbose {
        println!(" done!");
    }

    // If rebuild=true we fire off the rebuilt bootstrap procedure,
    // otherwise we build from our existing cache
    let lattice = if config.rebuild {
        println!("Starting rebuild from bootstrap at {}...", config.bootstrap);
        rebuild::rebuild(&config).unwrap()
    } else {
        let mut building_lattice = lattice::Lattice::new();
        if !config.verbose {
            print!("Loading scopes")
        }
        building_lattice.build_from_disk(&config.scopes, &trans_registry, config.verbose)?;
        building_lattice
    };
    println!("🧬 Lattice is live! {} scopes loaded", lattice.scopes.len());

    // Load the cached Usher Map
    if config.verbose {
        print!("Loading usher map...");
    }

    if !std::fs::exists(&config.usher_map)? {
        if config.verbose {
            print!("rebuilding...");
        }
        let um = UsherMap::new();
        map::disk_to(&config.usher_map, um);
    }
    let usher_map = usher::map::disk_from(&config.usher_map);
    if config.verbose {
        println!(" done!");
    }

    // Load I Am entries
    if config.verbose {
        print!("Loading I Am...");
    }
    if !std::fs::exists(&config.i_am)? {
        if config.verbose {
            print!("rebuilding...");
        }
        let i = IAm::new();
        i.disk_to(&config.i_am)?;
    }
    let i_am = IAm::disk_from(&config.i_am)?;
    if config.verbose {
        println!(" done!");
    }

    // Load enclave and populate it
    if config.verbose {
        print!("Enclave loading");
    }
    let mut enclave = Enclave::new(Some(config.enclave.clone()));
    enclave.populate()?;
    if config.verbose {
        println!(" done! Loaded {} keys", enclave.keys.len());
    }

    // Check I Am against Enclave to make sure we have all the local
    // keys we need
    if config.verbose {
        print!("Checking I Am against Enclave...")
    }
    enclave.check_map(i_am.get_local()?)?;
    if config.verbose {
        println!(" done!");
    }

    // RwLock-ed items
    let lattice = Arc::new(RwLock::new(lattice));
    let trans_registry = Arc::new(RwLock::new(trans_registry));
    let i_am = Arc::new(RwLock::new(i_am));
    let enclave = Arc::new(RwLock::new(enclave));
    let usher_map = Arc::new(RwLock::new(usher_map));

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("🟢 Server listening on {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let c = config.clone();
        let lattice_clone = Arc::clone(&lattice);
        let trans_reg_clone = Arc::clone(&trans_registry);
        let iam_clone = Arc::clone(&i_am);
        let enclave_clone = Arc::clone(&enclave);
        let usher_map_clone = Arc::clone(&usher_map);
        tokio::spawn(async move {
            if let Err(e) = handle_connection(
                stream,
                c,
                lattice_clone,
                trans_reg_clone,
                iam_clone,
                enclave_clone,
                usher_map_clone,
            )
            .await
            {
                eprintln!("Connection error: {:?}", e);
            }
        });
    }
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    config: UsherdConfig,
    lattice: Arc<RwLock<Lattice>>,
    trans_registry: Arc<RwLock<TransformRegistry>>,
    iam: Arc<RwLock<IAm>>,
    enclave: Arc<RwLock<Enclave>>,
    usher_map: Arc<RwLock<UsherMap>>,
) -> Result<()> {
    // We use LengthDelimitedCodec so we don't have to worry about
    // TCP fragmenting our CBOR blobs.
    let mut framed =
        tokio_util::codec::Framed::new(stream, tokio_util::codec::LengthDelimitedCodec::new());

    while let Some(request_result) = framed.next().await {
        let bytes = request_result?;
        let mut output = Vec::new();

        // 2. Decode using minicbor
        let rhex_list: Vec<Rhex> = minicbor::decode(&bytes)?;
        println!("Received {} items", rhex_list.len());

        {
            let mut lattice_guard = lattice.write().await;
            let mut trans_reg_guard = trans_registry.write().await;
            let mut iam_guard = iam.write().await;
            let mut enclave_guard = enclave.write().await;
            let mut usher_map_guard = usher_map.write().await;

            for rhex in &rhex_list {
                // Append rhex here
                let receive_output = receive(
                    &config,
                    rhex,
                    &mut trans_reg_guard,
                    &mut lattice_guard,
                    &mut iam_guard,
                    &mut enclave_guard,
                    &mut usher_map_guard,
                )?;
                match receive_output.0 {
                    ReceiveStatus::FailedValidation(_) => {
                        // TODO: build failed validation R⬢ to return to client
                    }
                    ReceiveStatus::MissingSignature(_) => {
                        // TODO: build failed validation R⬢ to return to client
                    }
                    // success means we have output from the append process
                    // already to go, so we just have to attach it to the
                    // output.
                    ReceiveStatus::Success => {
                        let mut out = receive_output.1.unwrap();
                        output.append(&mut out);
                    }
                }
            }
        }
        let mut response = Vec::new();
        minicbor::encode(&output, &mut response)?;

        // 4. Send back through the frame
        framed.send(response.into()).await?;
    }

    Ok(())
}
