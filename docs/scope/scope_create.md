# scope:create

Creates a new sub-scope, with an assigned creator key (the one that submitted the `scope:request`), and an array of usher endpoints, whom are automatically added to the Actor role.

## Sample

```rust
RtScopeCreate {
    scope: "some.scope.name".to_string(),
    creator: PublicKey::from_bytes(&[0u8; 32])?,
    ushers: vec![
        PublicKey::from_bytes(&[1u8; 32])?,
        PublicKey::from_bytes(&[2u8; 32])?,
        PublicKey::from_bytes(&[3u8; 32])?,
        PublicKey::from_bytes(&[4u8; 32])?,
    ]
}