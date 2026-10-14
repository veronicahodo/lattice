# Rules of the Lattice

What does it take to exist on the lattice? The lattice is a mesh of iroh-blobs and iroh-docs, so really all that is needed is an ed25519 key.

This was the core behind the lattice and then we found that `iroh` is basically the network level we would have designed if we knew what the hell we were doing 😭

## Where to start?

The initial record, or Rhex (R⬢), of the lattice is the `lattice:genesis` record. It makes a few assumptions, one is access to the `iroh` network, and two is that we accept my key as the one to kick this all off.

## Reserved record types (`intent.rt`)

There are a handful of reserved record types. These are what make the lattice operate as a living machine rather than just static blobs in iroh.

- `policy:set` - Probably the most important record type in the whole lattice. This dictates what groups can do what when and how often. There's a whole document and some built in methods that will keep you from slamming the door on a scope, leaving it to never be written to again.
- `key:assign` - Easily the second most important record type, this sets what keys have access to what scopes. Keys get assigned to groups, and the `policy:set` decides what groups can do what. Most scopes have a bunch of these and a couple `policy:set`s in the initial dozen R⬢ of the scope.
- `key:revoke` - Nixes the groups specified for the keys specified. This can create a complex map of group interactions, so the scope ushers have to be diligent in maintaining an acccurate map across the scope chain.
- `usher:assign` - This makes an additional usher (outside the `scope:create` record), and assigns it a role (actor, mirror, cache, etc.). Now that we are using `iroh` the usher's public key is enough for navigation purposes.
- `usher:revoke` - Same as key revoking, we disavow an usher from the network. It may still serve up it's cache, but it is no longer considered an authority.
- `request:scope` - Pulls the selected records from a scope. `RhexData` is JSON, with "requested_rhex" = either `vec!["*".to_string()]` or an array of record types. e.g.:

```rust
vec![
    "policy:set".to_string(),
    "key:assign".to_string(),
    "key:revoke".to_string()
];
```

- `request:head` - Returns the latest hash for the scope head. (`[u8; 32]` currently)
- `request:hash` - Returns the current hash for the whole scope chain, in case someone wants to clone the whole thing. (Again, `[u8; 32]` currently)
