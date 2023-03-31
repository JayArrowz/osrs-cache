# rs2-cache

[![Build](https://github.com/osrs-rs/rs2-cache/workflows/build/badge.svg)](https://github.com/osrs-rs/rs2-cache)
[![API](https://docs.rs/rs2-cache/badge.svg)](https://docs.rs/rs2-cache)
[![Crate](https://img.shields.io/crates/v/rs2-cache)](https://crates.io/crates/rs2-cache)
[![dependency status](https://deps.rs/repo/github/osrs-rs/rs2-cache/status.svg)](https://deps.rs/repo/github/osrs-rs/rs2-cache)
[![OSRS Version](https://img.shields.io/badge/OSRS-208-blue)](https://img.shields.io/badge/OSRS-208-blue)
[![Discord](https://img.shields.io/discord/926860365873184768?color=5865F2)](https://discord.gg/CcTa7TZfSc)

A RS2 cache library written in Rust, based on the OpenRS2 implementation.

## Installation

Add this to your `Cargo.toml` file:

```toml
[dependencies]
rs2-cache = "0.1.0"
```

## Example

```rust
use rs2cache::Cache;

fn main() -> Result<(), osrscache::Error> {
    let cache = Cache::open("./cache")?;

    let index_id = 2; // Config index
    let archive_id = 10; // Item definitions archive
    let file_id = 1042; // Blue Partyhat file

    let buffer = cache.read(index_id, archive_id, file_id)?;

    Ok(())
}
```

## Contributing

If you have suggestions for features, or want to add for example a new loader for the cache, feel free to make a pull request. For bigger features it is advised to [open an issue](https://github.com/osrs-rs/rs2-cache/issues/new) in order to discuss it beforehand.

Examples can be found in the [examples](examples/) directory which include the osrs update protocol.

## Credits

Graham for his work on [OpenRS2](https://github.com/openrs2/openrs2). If you are using Kotlin and working on a RS2 rev server, it is highly recommended to utilize it. 
