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

mod test_bond_public;
mod test_claim_unbond_public;
mod test_set_validator_state;
mod test_transfer_public;
mod test_transfer_public_to_private;
mod test_unbond_delegator_as_validator;
mod test_unbond_public;

use snarkvm::{
    console::account::{Address, PrivateKey},
    prelude::TestRng,
};

type CurrentNetwork = snarkvm::console::network::Testnet3;
type CurrentAleo = snarkvm::circuit::AleoV0;

/// Samples a random private key and address.
fn sample_account(rng: &mut TestRng) -> (PrivateKey<CurrentNetwork>, Address<CurrentNetwork>) {
    let private_key = PrivateKey::<CurrentNetwork>::new(rng).unwrap();
    let address = Address::<CurrentNetwork>::try_from(&private_key).unwrap();
    (private_key, address)
}
