use iroh::EndpointId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UsherLocation {
    Iroh { endpoint: EndpointId },
    Local { port: u16 },
    Internet { ip: String, port: u16 },
    Tor { onion: String },
    Unknown,
}
