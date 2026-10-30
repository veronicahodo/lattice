use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use iam::{IAm, location::IAmLocation};
use iroh::{Endpoint, PublicKey, endpoint::presets::N0};
use lattice::usher::{self, UsherMap, map};
use tokio::sync::RwLock;
use transform::registry::TransformRegistry;

use crate::{
    config::UsherIrohConfig,
    server::{
        address::get_addresses,
        appstate::AppState,
        connection::handle_connection,
        init::{init_enclave, init_lattice, init_resource},
    },
};

pub mod address;
pub mod appstate;
pub mod connection;
pub mod init;
pub mod process;
pub mod rebuild;

pub const PROTOCOL_ALPN: &[u8] = b"rhex-exchange/v1";

pub async fn run(config: Arc<UsherIrohConfig>) -> Result<()> {
    // Initialize
    let trans_registry = init_resource(
        &config.trans_registry,
        "transform registry",
        config.verbose,
        || TransformRegistry::new().to_file(&config.trans_registry),
        || TransformRegistry::from_file(&config.trans_registry, &config.trans_store),
    )?;
    let lattice = init_lattice(&config, &trans_registry)?;
    let usher_map = init_resource(
        &config.usher_map,
        "usher map",
        config.verbose,
        || Ok(map::disk_to(&config.usher_map, UsherMap::new())),
        || Ok(usher::map::disk_from(&config.usher_map)),
    )?;
    let mut iam = init_resource(
        &config.i_am,
        "I Am",
        config.verbose,
        || IAm::new().disk_to(&config.i_am),
        || IAm::disk_from(&config.i_am),
    )?;
    let pk = PublicKey::from_bytes(&config.usher_key)?;
    iam.add(&pk, None, IAmLocation::Local);

    let enclave = init_enclave(&config, &iam)?;
    let secret = enclave.export(&pk)?;

    // Build shared state
    let appstate = AppState {
        lattice: Arc::new(RwLock::new(lattice)),
        trans_registry: Arc::new(RwLock::new(trans_registry)),
        iam: Arc::new(RwLock::new(iam)),
        enclave: Arc::new(RwLock::new(enclave)),
        usher_map: Arc::new(RwLock::new(usher_map)),
    };

    // Set up listener
    let endpoint = Endpoint::builder(N0)
        .secret_key(secret)
        .alpns(vec![PROTOCOL_ALPN.to_vec()])
        .bind()
        .await?;
    let addr = endpoint.addr();
    let mut ips = Vec::new();
    let addr_string = get_addresses(addr.ip_addrs().collect());
    for a in addr.ip_addrs() {
        ips.push(match a {
            SocketAddr::V4(add) => {
                let bytes = add.ip().octets();
                let port = add.port();
                let bytes = [
                    bytes[0], bytes[1], bytes[2], bytes[3], 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ];
                (bytes, port)
            }
            SocketAddr::V6(add) => {
                let bytes = add.ip().octets();
                let port = add.port();
                (bytes, port)
            }
        });
    }
    println!(
        "Endpoint: {}, ({})",
        URL_SAFE_NO_PAD.encode(addr.id.as_bytes()),
        addr_string
    );

    // Handle the connection
    while let Some(incoming) = endpoint.accept().await {
        let config = Arc::clone(&config);
        let mut appstate = appstate.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(&config, &mut appstate, incoming).await {
                eprintln!("Error handling connection: {:?}", e);
            }
        });
    }
    Ok(())
}
