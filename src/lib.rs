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
    types::{Boolean, U64},
};

use anyhow::Result;
use core::str::FromStr;
use rand::{CryptoRng, Rng};

pub struct Credits;

impl Credits {
    /// Returns a transaction that allows any staker to bond their microcredits to a validator.
    pub fn bond_public(
        private_key: &str,
        validator: &str,
        amount_in_microcredits: u64,
        fee_in_microcredits: u64,
        broadcast: bool,
        rng: &mut (impl Rng + CryptoRng),
    ) -> Result<Authorized<N>> {
        // Initialize the private key.
        let private_key = PrivateKey::<N>::from_str(private_key)?;
        // Initialize the validator's address.
        let validator = Address::<N>::from_str(validator)?;
        // Initialize the amount in microcredits.
        let amount_in_microcredits = U64::<N>::new(amount_in_microcredits);

        // Construct the program ID and function name.
        let (program_id, function_name) = ("credits.aleo", "bond_public");
        // Construct the inputs.
        let inputs =
            vec![Value::<N>::from(Literal::Address(validator)), Value::from(Literal::U64(amount_in_microcredits))];
        // Construct the authorization.
        Self::authorize(&private_key, program_id, function_name, inputs, fee_in_microcredits, broadcast, rng)
    }

    /// Returns a transaction that any staker to unbond their microcredits from a validator.
    pub fn unbond_public(
        private_key: &str,
        amount_in_microcredits: u64,
        fee_in_microcredits: u64,
        broadcast: bool,
        rng: &mut (impl Rng + CryptoRng),
    ) -> Result<Authorized<N>> {
        // Initialize the private key.
        let private_key = PrivateKey::<N>::from_str(private_key)?;
        // Initialize the amount in microcredits.
        let amount_in_microcredits = U64::<N>::new(amount_in_microcredits);

        // Construct the program ID and function name.
        let (program_id, function_name) = ("credits.aleo", "unbond_public");
        // Construct the inputs.
        let inputs = vec![Value::from(Literal::U64(amount_in_microcredits))];
        // Construct the authorization.
        Self::authorize(&private_key, program_id, function_name, inputs, fee_in_microcredits, broadcast, rng)
    }

    /// Returns a transaction that allows a validator to unbond any delegator that is bonded to them.
    pub fn unbond_delegator_as_validator(
        private_key: &str,
        delegator: &str,
        fee_in_microcredits: u64,
        broadcast: bool,
        rng: &mut (impl Rng + CryptoRng),
    ) -> Result<Authorized<N>> {
        // Initialize the private key.
        let private_key = PrivateKey::<N>::from_str(private_key)?;
        // Initialize the delegator's address.
        let delegator = Address::<N>::from_str(delegator)?;

        // Construct the program ID and function name.
        let (program_id, function_name) = ("credits.aleo", "unbond_delegator_as_validator");
        // Construct the inputs.
        let inputs = vec![Value::<N>::from(Literal::Address(delegator))];
        // Construct the authorization.
        Self::authorize(&private_key, program_id, function_name, inputs, fee_in_microcredits, broadcast, rng)
    }

    /// Returns a transaction that allows any staker to claim their microcredits after the unbonding period.
    pub fn claim_unbond_public(
        private_key: &str,
        fee_in_microcredits: u64,
        broadcast: bool,
        rng: &mut (impl Rng + CryptoRng),
    ) -> Result<Authorized<N>> {
        // Initialize the private key.
        let private_key = PrivateKey::<N>::from_str(private_key)?;

        // Construct the program ID and function name.
        let (program_id, function_name) = ("credits.aleo", "claim_unbond_public");
        // Construct the authorization.
        Self::authorize(&private_key, program_id, function_name, vec![], fee_in_microcredits, broadcast, rng)
    }

    /// Returns a transaction that allows a validator to set their state to be either opened or closed to stakers.
    pub fn set_validator_state(
        private_key: &str,
        is_open: bool,
        fee_in_microcredits: u64,
        broadcast: bool,
        rng: &mut (impl Rng + CryptoRng),
    ) -> Result<Authorized<N>> {
        // Initialize the private key.
        let private_key = PrivateKey::<N>::from_str(private_key)?;
        // Initialize the 'is_open' boolean flag.
        let is_open = Boolean::<N>::new(is_open);

        // Construct the program ID and function name.
        let (program_id, function_name) = ("credits.aleo", "set_validator_state");
        // Construct the inputs.
        let inputs = vec![Value::<N>::from(Literal::Boolean(is_open))];
        // Construct the authorization.
        Self::authorize(&private_key, program_id, function_name, inputs, fee_in_microcredits, broadcast, rng)
    }

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

        // Construct the program ID and function name.
        let (program_id, function_name) = ("credits.aleo", "transfer_public");
        // Construct the inputs.
        let inputs =
            vec![Value::<N>::from(Literal::Address(recipient)), Value::from(Literal::U64(amount_in_microcredits))];
        // Construct the authorization.
        Self::authorize(&private_key, program_id, function_name, inputs, fee_in_microcredits, broadcast, rng)
    }
}

impl Credits {
    /// An internal method that authorizes a function call with a corresponding fee.
    fn authorize(
        private_key: &PrivateKey<N>,
        program_id: &str,
        function_name: &str,
        inputs: Vec<Value<N>>,
        fee_in_microcredits: u64,
        broadcast: bool,
        rng: &mut (impl Rng + CryptoRng),
    ) -> Result<Authorized<N>> {
        // Authorize the main function.
        let function = PROCESS.authorize::<A, _>(&private_key, program_id, function_name, inputs.into_iter(), rng)?;
        // Retrieve the execution ID.
        let execution_id = function.to_execution_id()?;
        // Authorize the fee.
        let fee = match fee_in_microcredits == 0 {
            true => None,
            false => {
                Some(PROCESS.authorize_fee_public::<A, _>(&private_key, fee_in_microcredits, execution_id, rng)?)
            }
        };
        // Construct the authorization.
        Ok(Authorized::<N>::new(function, fee, broadcast))
    }
}
