// Copyright (C) 2019-2023 Howard Wu
// This file is part of the credits library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// The network.
pub type N = snarkvm::console::network::Testnet3;
/// The network circuit.
pub type A = snarkvm::circuit::AleoV0;
/// The API URL for the network.
pub const API_URL: &str = "https://api.explorer.aleo.org/v1/testnet3/explorer";
// pub const API_URL: &str = "http://127.0.0.1:6130/v1/testnet3/explorer";

use snarkvm::synthesizer::Process;

use anyhow::{bail, Result};

lazy_static! {
    /// The main process.
    pub(crate) static ref PROCESS: Process<N> = Process::<N>::load().unwrap();
}

/// Retrieves the base fee in microcredits for a given program and function.
pub fn get_base_fee_in_microcredits(program_id: &str, function_name: &str) -> Result<u64> {
    match (program_id, function_name) {
        ("credits.aleo", "bond_public") => Ok(843880),
        ("credits.aleo", "claim_unbond_public") => Ok(167230),
        ("credits.aleo", "set_validator_state") => Ok(128275),
        ("credits.aleo", "transfer_public") => Ok(263388),
        ("credits.aleo", "transfer_public_to_private") => Ok(136587),
        ("credits.aleo", "unbond_delegator_as_validator") => Ok(324857),
        ("credits.aleo", "unbond_public") => Ok(1233777),
        _ => bail!("Unknown program ID '{program_id}' and function name '{function_name}'"),
    }
}
