// Copyright (C) 2019-2023 Aleo Systems Inc.
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

#[macro_use]
extern crate lazy_static;

mod authorized;
pub use authorized::*;

mod config;
pub use config::*;

#[cfg(test)]
mod tests;

use snarkvm::console::{
    account::{Address, PrivateKey},
    program::{Literal, Value},
    types::U64,
};

use anyhow::Result;
use core::str::FromStr;
use rand::{CryptoRng, Rng};

pub struct Credits;

impl Credits {
    /// Returns a transaction that transfers public credits from the sender to the recipient.
    pub fn transfer_public(
        private_key: &str,
        recipient: &str,
        amount_in_microcredits: u64,
        fee_in_microcredits: u64,
        broadcast: bool,
        rng: &mut (impl Rng + CryptoRng),
    ) -> Result<Authorized<N>> {
        // Initialize the private key.
        let private_key = PrivateKey::<N>::from_str(private_key)?;
        // Initialize the recipient.
        let recipient = Address::<N>::from_str(recipient)?;
        // Initialize the amount in microcredits.
        let amount_in_microcredits = U64::<N>::new(amount_in_microcredits);

        // Construct the inputs.
        let inputs =
            [Value::<N>::from(Literal::Address(recipient)), Value::<N>::from(Literal::U64(amount_in_microcredits))]
                .into_iter();

        // Authorize the main function.
        let function = PROCESS.authorize::<A, _>(&private_key, "credits.aleo", "transfer_public", inputs, rng)?;
        // Retrieve the execution ID.
        let execution_id = function.to_execution_id()?;
        // Authorize the fee.
        let fee = PROCESS.authorize_fee_public::<A, _>(&private_key, fee_in_microcredits, execution_id, rng)?;

        // Construct the authorization.
        Ok(Authorized::<N>::new(function, Some(fee), broadcast))
    }
}
