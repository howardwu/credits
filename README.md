# Credits SDK

[![Crates.io](https://img.shields.io/crates/v/credits.svg?color=neon)](https://crates.io/crates/credits)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE.md)

The `credits` crate provides an interface for managing and handling operations related to credit transactions on the Aleo network. With this crate, you can effortlessly generate, authorize, and execute credit-related transactions like transferring credits, bonding, unbonding, and more.

## Features

- **Simple Transactions**: Easily transfer credits between two accounts.
- **Staking & Validators**: Bond or unbond microcredits to validators, control validator states, and manage unbonded credits.

## Usage

First, add `credits` to your `Cargo.toml` dependencies:

```toml
[dependencies]
credits = "0.1.3"
```

Then, incorporate it in your Rust code:

```rust
use credits::Credits;
```


## Notice

This repository is under active development and is subject to change.

Notably, this repository does not yet support the following:
- [ ] Private transfers
- [ ] Private join/splits
- [ ] **Checking the fee is sufficient for a transaction before executing it**

## Examples

#### `bond_public`
```rust
use credits::Credits;
use anyhow::Result;

fn main() -> Result<()> {
    let rng = &mut rand::thread_rng();

    let transaction = Credits::bond_public(
        "APrivateKey1zkpBdGzC71T2A3D4bfyuPnz5NyJNLhSx3VQxWRMcha3JYtp", // Staker's Private Key
        "aleo1r8ak4sfzpljs65lu0cgu6x4pvvq6atsdx268auu7nf6wvsv5fgqq6v5p0a", // Validator's Address
        10_000_000, // Amount (in microcredits)
        10_000, // Priority Fee (in microcredits)
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
        10_000_000, // Amount (in microcredits)
        10_000, // Priority Fee (in microcredits)
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
        10_000, // Priority Fee (in microcredits)
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
        10_000, // Priority Fee (in microcredits)
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
        10_000, // Priority Fee (in microcredits)
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
        10_000_000, // Amount (in microcredits)
        10_000, // Priority Fee (in microcredits)
        false, // Broadcast
        rng,
    )?.execute()?;
}
```

#### `transfer_public_to_private`
```rust
use credits::Credits;
use anyhow::Result;

fn main() -> Result<()> {
    let rng = &mut rand::thread_rng();
    
    let transaction = Credits::transfer_public(
        "APrivateKey1zkpBdGzC71T2A3D4bfyuPnz5NyJNLhSx3VQxWRMcha3JYtp", // Sender's Private Key
        "aleo1r8ak4sfzpljs65lu0cgu6x4pvvq6atsdx268auu7nf6wvsv5fgqq6v5p0a", // Recipient's Address
        10_000_000, // Amount (in microcredits)
        10_000, // Priority Fee (in microcredits)
        false, // Broadcast
        rng,
    )?.execute()?;
}
```

## Testing

This crate provides a comprehensive set of tests for every function in the credits program.

Use the following to run tests:

```bash
cargo test
```

## Contributing

Pull requests are welcome. For significant changes, please open an issue first to discuss the intended change.
