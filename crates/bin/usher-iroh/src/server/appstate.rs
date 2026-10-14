use std::sync::Arc;
use tokio::sync::RwLock;

use iam::IAm;
use key::enclave::Enclave;
use lattice::{Lattice, usher::UsherMap};
use transform::registry::TransformRegistry;

#[derive(Clone)]
pub struct AppState {
    pub lattice: Arc<RwLock<Lattice>>,
    pub trans_registry: Arc<RwLock<TransformRegistry>>,
    pub iam: Arc<RwLock<IAm>>,
    pub enclave: Arc<RwLock<Enclave>>,
    pub usher_map: Arc<RwLock<UsherMap>>,
}
