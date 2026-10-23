use std::collections::HashMap;

use iroh::PublicKey;

use crate::Usher;

pub type UsherMap = HashMap<PublicKey, Usher>;

pub fn disk_from(path: &str) -> UsherMap {
    let data = std::fs::read(path).unwrap();
    let ushers: Vec<Usher> = cbor2::from_slice(&data).unwrap();
    let mut ushers_map = HashMap::new();
    for usher in ushers {
        ushers_map.insert(usher.pk, usher);
    }
    ushers_map
}

pub fn disk_to(path: &str, ushers: UsherMap) {
    let mut ushers_vec = Vec::new();
    for (_, usher) in ushers {
        ushers_vec.push(usher);
    }
    let data = cbor2::to_canonical_vec(&ushers_vec);
    std::fs::write(path, &data.unwrap()).unwrap();
}
