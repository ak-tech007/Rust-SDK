# fuels-rs

[![build](https://github.com/FuelLabs/fuels-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/FuelLabs/fuels-rs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/fuels?label=latest)](https://crates.io/crates/fuels)
[![docs](https://docs.rs/fuels/badge.svg)](https://docs.rs/fuels)
[![discord](https://img.shields.io/badge/chat%20on-discord-orange?&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/xfpK4Pe)

Rust SDK for Fuel. It can be used for a variety of things, including but not limited to:

- Compiling, deploying, and testing [Sway](https://github.com/FuelLabs/sway) contracts;
- Launching a local Fuel network;
- Crafting and signing transactions with hand-crafted scripts or contract calls;
- Generating type-safe Rust bindings of contract methods;
- And more, `fuels-rs` is still in active development.

## Documentation

See [the `fuels-rs` book](https://fuellabs.github.io/fuels-rs/latest/)

## Features

- [x] Launch Fuel nodes
- [x] Deploy contracts
- [x] Interact with deployed contracts
- [x] Type-safe Sway contracts bindings code generation
- [x] Run Sway scripts
- [x] CLI for common operations
- [x] Local test wallets
- [ ] Wallet integration
- [ ] Events querying/monitoring


## FAQ

### Why is the prefix `fuels` and not `fuel`?

In order to make the SDK for Fuel feel familiar with those coming from the [ethers.js](https://github.com/ethers-io/ethers.js) ecosystem, this project opted for an `s` at the end. The `fuels-*` family of SDKs is inspired by The Ethers Project.

