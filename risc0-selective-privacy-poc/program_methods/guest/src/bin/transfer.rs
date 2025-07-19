use core::{account::Account, types::ProgramOutput};
use risc0_zkvm::guest::env;

/// A transfer of balance program.
/// To be used both in public and private contexts.
fn main() {
    // Read input accounts.
    // It is expected to receive only two accounts: [sender_account, receiver_account]
    let input_accounts: Vec<Account> = env::read();
    let balance_to_move: u128 = env::read();

    // Unpack sender and receiver
    assert_eq!(input_accounts.len(), 2);
    let [sender, receiver] = input_accounts.try_into().unwrap();

    // Check sender has enough balance
    assert!(sender.balance >= balance_to_move);

    // Create accounts post states, with updated balances
    let mut sender_post = sender.clone();
    let mut receiver_post = receiver.clone();
    sender_post.balance -= balance_to_move;
    receiver_post.balance += balance_to_move;

    let output = ProgramOutput {
        accounts_pre: vec![sender, receiver],
        accounts_post: vec![sender_post, receiver_post],
    };

    env::commit(&output);
}
