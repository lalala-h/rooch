// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use crate::address::{RoochAddress, RoochSupportedAddress};
use crate::transaction::authenticator::Authenticator;
use crate::transaction::rooch::{RoochTransaction, RoochTransactionData};
use rand::{thread_rng, Rng};

pub use moveos_types::test_utils::*;

pub fn random_rooch_transaction() -> RoochTransaction {
    let move_action_type = random_move_action_type();
    random_rooch_transaction_with_move_action(move_action_type)
}

pub fn random_rooch_transaction_with_move_action(move_action: MoveActionType) -> RoochTransaction {
    let mut rng = thread_rng();
    let sequence_number = rng.gen_range(1..=100);
    let tx_data = RoochTransactionData::new_for_test(
        RoochAddress::random(),
        sequence_number,
        random_move_action_with_action_type(move_action.action_type()),
    );

    let mut rng = thread_rng();
    let auth_validator_id = rng.gen_range(1..=100);
    let authenticator = Authenticator::new(auth_validator_id, random_bytes());

    RoochTransaction::new(tx_data, authenticator)
}
