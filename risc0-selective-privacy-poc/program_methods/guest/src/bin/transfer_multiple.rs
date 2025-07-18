use core::account::Account;
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
    let mut receivers_post = recipients.clone();

    // Transfer balances
    sender_post.balance -= total_balance_to_move;
    for (receiver, balance_for_receiver) in receivers_post.iter_mut().zip(target_balances) {
        receiver.balance += balance_for_receiver;
    }

    // Flatten pre and post states for output
    let inputs_outputs: Vec<Account> = vec![sender]
        .into_iter()
        .chain(recipients)
        .chain(vec![sender_post])
        .chain(receivers_post)
        .collect();

    env::commit(&inputs_outputs);
}
