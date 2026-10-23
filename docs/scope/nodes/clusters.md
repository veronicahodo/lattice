# Node Clusters

## What are they?

Node Clusters are typically a pool of devices owned by an entity. This could be a person, an organization, or even something more complex. The point of them is to be an anchor in the Lattice, by providing availablity and execution. While thanks to mirrors most data survives a scope's entire cluster dropping off, it is still obviously ideal to have a live "hot" data store.

## How to set one up

Thanks to the backend that we get with integration into Iroh, we can set up machines all over the globe to work in unison. Your phones, tablets, laptops, and desktops all proved a symmetrical view of the data you have provided to the Lattice.

**TODO** actual integration, obviously. I'm thinking a script of some sorts that does the bulk of the work.

Pseudo-interface for cli clients:

`cluster create <id>` - This creates a new group, either providing a new key to be shared or using the one provided by `<id>`

This creates a new group, with the local node being the first node. This is how it all begins. Obviously mobile clients will differ, and it may be a minute before we get to being able to host a cluster on a phone by itself (availability is the main issue).

## Adding nodes

`cluster add endpoint_id` - This begins the key exchange necessary for securely tying a device into the cluster. As long as both systems have the same master key it should automatically set everything up, with options for parameters being located in the JSON config for desktop clients and in the mobile app settings on Android/iPhone
