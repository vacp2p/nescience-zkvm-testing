use core::account::Account;
use risc0_zkvm::{default_executor, ExecutorEnv};

use nssa;

use nssa::program::TransferMultipleProgram;

/// A public execution.
/// This would be executed by the runtime after checking that
/// the initiating transaction includes the sender's signature.
pub fn main() {
    // Account fetched from the chain state with 150 in its balance.
    let sender = {
        let mut account = Account::new([5; 8], [98; 8]);
        account.balance = 150;
        account
    };

    // Account fetched from the chain state with 900 in its balance.
    let receiver_1 = {
        let mut account = Account::new([6; 8], [99; 8]);
        account.balance = 900;
        account
    };

    let receiver_2 = {
        let mut account = Account::new([6; 8], [99; 8]);
        account.balance = 500;
        account
    };

    let balance_to_move = vec![10, 20];

    let inputs_outputs = nssa::execute::<TransferMultipleProgram>(
        &[sender, receiver_1, receiver_2],
        balance_to_move,
    )
    .unwrap();

    println!(
        "sender_before: {:?}, sender_after: {:?}",
        inputs_outputs[0], inputs_outputs[3]
    );
    println!(
        "receiver_1_before: {:?}, receiver_1_after: {:?}",
        inputs_outputs[1], inputs_outputs[4],
    );
    println!(
        "receiver_2_before: {:?}, receiver_2_after: {:?}",
        inputs_outputs[2], inputs_outputs[5],
    );
}
