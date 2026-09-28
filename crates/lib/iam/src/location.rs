use minicbor::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode, PartialEq)]
pub enum IAmLocation {
    #[n(0)]
    None,
    #[n(1)]
    Local,
    #[n(2)]
    Internet {
        #[n(0)]
        address: String,
        #[n(1)]
        port: u16,
    },
    #[n(3)]
    Tor {
        #[n(0)]
        onion: String,
        #[n(1)]
        port: u16,
    },
    #[n(4)]
    Unknown,
}
