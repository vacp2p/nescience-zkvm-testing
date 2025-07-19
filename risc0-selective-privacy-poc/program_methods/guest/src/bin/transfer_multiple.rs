use core::{account::Account, types::ProgramOutput};
use risc0_zkvm::guest::env;

/// A transfer of balance program with one sender and multiple recipients.
/// To be used both in public and private contexts.
fn main() {
    // Read input accounts.
    // First account is the sender.
    let mut input_accounts: Vec<Account> = env::read();

    // Read the balances to be send to each recipient
    let target_balances: Vec<u128> = env::read();

    // Check that there is at least one recipient
    assert!(input_accounts.len() > 1);

    // Check that there's one target balance for each recipient.
    assert_eq!(target_balances.len() + 1, input_accounts.len());

    // Unpack sender and recipients
    let recipients = input_accounts.split_off(1);
    let sender = input_accounts.pop().unwrap();

    // Check that the sender has enough balance to pay to all recipients
    let total_balance_to_move = target_balances.iter().sum();
    assert!(sender.balance >= total_balance_to_move);

    // Create accounts post states, with updated balances
    let mut sender_post = sender.clone();
    let mut recipients_post = recipients.clone();

    // Transfer balances
    sender_post.balance -= total_balance_to_move;
    for (receiver, balance_for_receiver) in recipients_post.iter_mut().zip(target_balances) {
        receiver.balance += balance_for_receiver;
    }

    let output = ProgramOutput {
        accounts_pre: vec![sender].into_iter().chain(recipients).collect(),
        accounts_post: vec![sender_post].into_iter().chain(recipients_post).collect(),
    };

    env::commit(&output);
}
