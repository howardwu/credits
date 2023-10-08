# Credits SDK

<!-- [![Crates.io](https://img.shields.io/crates/v/credits.svg?color=neon)](https://crates.io/crates/credits) -->
[![Authors](https://img.shields.io/badge/authors-Aleo-orange.svg)](https://aleo.org)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE.md)

## Example

#### `transfer_public`
```rust
use credits::Credits;
use anyhow::Result;

fn main() -> Result<()> {
    let rng = &mut rand::thread_rng();
    
    let transaction = Credits::transfer_public(
        "APrivateKey1zkpBdGzC71T2A3D4bfyuPnz5NyJNLhSx3VQxWRMcha3JYtp", // Sender
        "aleo1r8ak4sfzpljs65lu0cgu6x4pvvq6atsdx268auu7nf6wvsv5fgqq6v5p0a", // Recipient
        1_000_000, // Amount (in microcredits)
        10_000, // Fee (in microcredits)
        false, // Broadcast
        rng,
    )?.execute()?;
}
```
