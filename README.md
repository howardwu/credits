# Credits SDK

[![Crates.io](https://img.shields.io/crates/v/credits.svg?color=neon)](https://crates.io/crates/credits)
[![Authors](https://img.shields.io/badge/authors-Aleo-orange.svg)](https://aleo.org)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE.md)

## Example

#### `bond_public`
```rust
use credits::Credits;
use anyhow::Result;

fn main() -> Result<()> {
    let rng = &mut rand::thread_rng();

    let transaction = Credits::bond_public(
        "APrivateKey1zkpBdGzC71T2A3D4bfyuPnz5NyJNLhSx3VQxWRMcha3JYtp", // Staker's Private Key
        "aleo1r8ak4sfzpljs65lu0cgu6x4pvvq6atsdx268auu7nf6wvsv5fgqq6v5p0a", // Validator's Address
        1_000_000, // Amount (in microcredits)
        10_000, // Fee (in microcredits)
        false, // Broadcast
        rng,
    )?.execute()?;
}
```

#### `unbond_public`
```rust
use credits::Credits;
use anyhow::Result;

fn main() -> Result<()> {
    let rng = &mut rand::thread_rng();

    let transaction = Credits::unbond_public(
        "APrivateKey1zkpBdGzC71T2A3D4bfyuPnz5NyJNLhSx3VQxWRMcha3JYtp", // Staker's Private Key
        1_000_000, // Amount (in microcredits)
        10_000, // Fee (in microcredits)
        false, // Broadcast
        rng,
    )?.execute()?;
}
```

#### `unbond_delegator_as_validator`
```rust
use credits::Credits;
use anyhow::Result;

fn main() -> Result<()> {
    let rng = &mut rand::thread_rng();

    let transaction = Credits::unbond_delegator_as_validator(
        "APrivateKey1zkpBdGzC71T2A3D4bfyuPnz5NyJNLhSx3VQxWRMcha3JYtp", // Validator's Private Key
        "aleo1r8ak4sfzpljs65lu0cgu6x4pvvq6atsdx268auu7nf6wvsv5fgqq6v5p0a", // Delegator's Address
        10_000, // Fee (in microcredits)
        false, // Broadcast
        rng,
    )?.execute()?;
}
```
#### `claim_unbond_public`
```rust
use credits::Credits;
use anyhow::Result;

fn main() -> Result<()> {
    let rng = &mut rand::thread_rng();

    let transaction = Credits::claim_unbond_public(
        "APrivateKey1zkpBdGzC71T2A3D4bfyuPnz5NyJNLhSx3VQxWRMcha3JYtp", // Staker's Private Key
        10_000, // Fee (in microcredits)
        false, // Broadcast
        rng,
    )?.execute()?;
}
```

#### `set_validator_state`
```rust
use credits::Credits;
use anyhow::Result;

fn main() -> Result<()> {
    let rng = &mut rand::thread_rng();

    let transaction = Credits::set_validator_state(
        "APrivateKey1zkpBdGzC71T2A3D4bfyuPnz5NyJNLhSx3VQxWRMcha3JYtp", // Validator's Private Key
        true, // is_open
        10_000, // Fee (in microcredits)
        false, // Broadcast
        rng,
    )?.execute()?;
}
```

#### `transfer_public`
```rust
use credits::Credits;
use anyhow::Result;

fn main() -> Result<()> {
    let rng = &mut rand::thread_rng();
    
    let transaction = Credits::transfer_public(
        "APrivateKey1zkpBdGzC71T2A3D4bfyuPnz5NyJNLhSx3VQxWRMcha3JYtp", // Sender's Private Key
        "aleo1r8ak4sfzpljs65lu0cgu6x4pvvq6atsdx268auu7nf6wvsv5fgqq6v5p0a", // Recipient's Address
        1_000_000, // Amount (in microcredits)
        10_000, // Fee (in microcredits)
        false, // Broadcast
        rng,
    )?.execute()?;
}
```
