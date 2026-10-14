use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UsherLocation {
    Local { port: u16 },
    Internet { ip: String, port: u16 },
    Tor { onion: String },
    Unknown,
}
