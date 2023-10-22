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

use super::*;
use crate::Authorized;

/// Samples the authorization for `bond_public`.
pub(crate) fn sample_all_authorizations(rng: &mut TestRng) -> Vec<Authorized<CurrentNetwork>> {
    vec![
        sample_bond_public(rng),
        sample_claim_unbond_public(rng),
        sample_set_validator_state(rng),
        sample_transfer_public(rng),
        sample_transfer_public_to_private(rng),
        sample_unbond_delegator_as_validator(rng),
        sample_unbond_public(rng),
    ]
}

#[test]
fn test_cost() {
    use snarkvm::{
        ledger::store::{helpers::memory::ConsensusMemory, ConsensusStore},
        synthesizer::VM,
    };

    let rng = &mut TestRng::default();

    // Initialize the VM.
    let vm = VM::from(ConsensusStore::<_, ConsensusMemory<_>>::open(None).unwrap()).unwrap();

    // Initialize the string.
    let mut string = String::new();

    for authorization in sample_all_authorizations(rng) {
        // Execute the authorization.
        let transaction = authorization.execute_local::<CurrentAleo, _>(rng).unwrap();
        assert!(transaction.execution().is_some());
        assert_eq!(transaction.execution().unwrap().transitions().len(), 1);

        // Retrieve the execution.
        let execution = transaction.execution().unwrap();
        // Retrieve the transition.
        let transition = execution.transitions().next().unwrap().clone();
        // Retrieve the program ID.
        let program_id = transition.program_id().to_string();
        // Retrieve the function name.
        let function_name = transition.function_name().to_string();

        // Compute the cost.
        let (cost, _) = snarkvm::synthesizer::execution_cost(&vm, execution).unwrap();
        string += &format!("\t(\"{program_id}\", \"{function_name}\") => Ok({cost}),\n");

        // Retrieve the base fee in microcredits.
        let found = crate::config::get_base_fee_in_microcredits(&program_id, &function_name).unwrap();
        assert_eq!(cost, found);
    }

    // Print the string.
    println!("\n{string}");
}
