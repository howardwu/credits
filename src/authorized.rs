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

use crate::API_URL;
use snarkvm::{
    console::network::Network,
    ledger::block::Transaction,
    prelude::{de, Deserialize, DeserializeExt, Deserializer, Serialize, SerializeStruct, Serializer},
    synthesizer::Authorization,
};

use anyhow::{bail, Result};

#[cfg(test)]
use rand::{CryptoRng, Rng};
#[cfg(test)]
use snarkvm::circuit::Aleo;

pub struct Authorized<N: Network> {
    /// The authorization for the main function execution.
    function: Authorization<N>,
    /// The authorization for the fee execution.
    fee: Option<Authorization<N>>,
    /// Whether to broadcast the transaction.
    broadcast: bool,
}

impl<N: Network> Authorized<N> {
    /// Initializes a new authorization.
    pub const fn new(function: Authorization<N>, fee: Option<Authorization<N>>, broadcast: bool) -> Self {
        Self { function, fee, broadcast }
    }

    /// Executes the authorization, returning the resulting transaction.
    pub fn execute(self) -> Result<Transaction<N>> {
        // Execute the authorization.
        let response = reqwest::blocking::Client::new()
            .post(format!("{API_URL}/execute"))
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&self)?)
            .send()?;

        // Ensure the response is successful.
        match response.status().is_success() {
            // Return the transaction.
            true => Ok(response.json()?),
            // Return the error.
            false => bail!(response.text()?),
        }
    }

    /// Executes the authorization locally, returning the resulting transaction.
    ///
    /// Note: Due to the lack of ledger state, this method should *only* be used for *testing*.
    #[cfg(test)]
    pub fn execute_local<A: Aleo<Network = N>, R: Rng + CryptoRng>(self, rng: &mut R) -> Result<Transaction<N>> {
        use snarkvm::{
            ledger::store::{helpers::memory::ConsensusMemory, ConsensusStore},
            synthesizer::VM,
        };

        // Initialize the VM.
        let vm = VM::from(ConsensusStore::<_, ConsensusMemory<_>>::open(None)?)?;
        // Execute the transaction.
        vm.execute_authorization(self.function, self.fee, None, rng)
    }
}

impl<N: Network> Serialize for Authorized<N> {
    /// Serializes the authorization into string or bytes.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut authorization = serializer.serialize_struct("Authorized", 3)?;
        authorization.serialize_field("function", &self.function)?;
        if let Some(fee) = &self.fee {
            authorization.serialize_field("fee", fee)?;
        }
        authorization.serialize_field("broadcast", &self.broadcast)?;
        authorization.end()
    }
}

impl<'de, N: Network> Deserialize<'de> for Authorized<N> {
    /// Deserializes the authorization from a string or bytes.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // Parse the authorization from a string into a value.
        let mut authorization = serde_json::Value::deserialize(deserializer)?;
        // Retrieve the function authorization.
        let function: Authorization<_> = DeserializeExt::take_from_value::<D>(&mut authorization, "function")?;
        // Retrieve the fee authorization, if it exists.
        let fee = serde_json::from_value(authorization.get_mut("fee").unwrap_or(&mut serde_json::Value::Null).take())
            .map_err(de::Error::custom)?;
        // Retrieve the broadcast flag.
        let broadcast = DeserializeExt::take_from_value::<D>(&mut authorization, "broadcast")?;
        // Recover the authorization.
        Ok(Self { function, fee, broadcast })
    }
}
