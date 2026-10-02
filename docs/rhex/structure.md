# Structure of R⬢

```rust
Rhex {
    magic: [u8; 6],
    intent: RhexIntent {
        prev: Option<[u8; 32]>,
        scope: String,
        author: [u8; 32],
        usher: [u8; 32],
        schema: Option<String>,
        rt: String,
        data_hash: [u8; 32]
    },
    data: RhexData {
        None,
        Json(Vec<u8>),
        Binary(Vec<u8>),
        Mixed { meta: Vec<u8>, data: Vec<u8> },
        Cid(Vec<u8>),
        Ejected(String)
    },
    context: RhexContext {
        at: u64,
        s: Option<ContextSpacial {
            s_ref: String,
            s_data: Vec<u8>
        }>
    },
    sigs: Vec<RhexSignature {
        pk: [u8; 32],
        sig: [u8; 64],
        t: RhexSignatureType {
            Author,
            Usher,
            Quorum(u64),
            Observer(u64),
            Other
        }
    }>,
    curr_hash: [u8; 32]
}
```
