use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IAmLocation {
    None,
    Local,
    Internet { address: String, port: u16 },
    Tor { onion: String, port: u16 },
    Unknown,
}
