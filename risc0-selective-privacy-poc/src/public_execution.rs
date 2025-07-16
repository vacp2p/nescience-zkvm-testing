use risc0_zkvm::{default_executor, ExecutorEnv};
use toy_example_core::account::Account;
use transfer_methods::TRANSFER_ELF;

use crate::{
    program::{execute, Program},
    TransferProgram,
};

/// A public execution.
/// This would be executed by the runtime after checking that
/// the initiating transaction includes the sender's signature.
pub fn run_public_execution_of_transfer_program() {
    // Account fetched from the chain state with 150 in its balance.
    let sender = {
        let mut account = Account::new([5; 8], [98; 8]);
        account.balance = 150;
        account
    };

    // Account fetched from the chain state with 900 in its balance.
    let receiver = {
        let mut account = Account::new([6; 8], [99; 8]);
        account.balance = 900;
        account
    };

    let balance_to_move: u128 = 3;

    let inputs_outputs = execute::<TransferProgram>(&[sender, receiver], &balance_to_move).unwrap();

    println!(
        "sender_before: {:?}, sender_after: {:?}",
        inputs_outputs[0], inputs_outputs[2]
    );
    println!(
        "receiver_before: {:?}, receiver_after: {:?}",
        inputs_outputs[1], inputs_outputs[3],
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public() {
        run_public_execution_of_transfer_program();
    }
}
