use core::account::Account;


use nssa::program::TransferMultipleProgram;

/// A public execution of the TransferMultipleProgram.
/// This would be executed by the runtime after checking that
/// the initiating transaction includes the sender's signature.
pub fn main() {
    // Account fetched from the chain state with 150 in its balance.
    let sender = Account::new([5; 8], 150);

    // Account fetched from the chain state with 900 in its balance.
    let receiver_1 = Account::new([6; 8], 900);

    // Account fetched from the chain state with 500 in its balance.
    let receiver_2 = Account::new([6; 8], 500);

    let balance_to_move = vec![10, 20];

    let inputs_outputs =
        nssa::execute_onchain::<TransferMultipleProgram>(&[sender, receiver_1, receiver_2], balance_to_move).unwrap();

    println!("OK!");
}
