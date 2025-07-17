use core::account::Account;
use risc0_zkvm::guest::env;

/// A transfer of balance program with multiple recipients.
/// To be used both in public and private contexts.
fn main() {
    let mut input_accounts: Vec<Account> = env::read();
    let target_balances: Vec<u128> = env::read();

    assert!(input_accounts.len() > 1);
    assert_eq!(target_balances.len() + 1, input_accounts.len());

    let receivers = input_accounts.split_off(1);
    let sender = input_accounts.pop().unwrap();
    let total_balance_to_move = target_balances.iter().sum();

    // Check sender has enough balance
    assert!(sender.balance >= total_balance_to_move);

    // Create accounts post states, with updated balances
    let mut sender_post = sender.clone();
    let mut receivers_post = receivers.clone();

    sender_post.balance -= total_balance_to_move;
    for (receiver, balance_for_receiver) in receivers_post.iter_mut().zip(target_balances) {
        receiver.balance += balance_for_receiver;
    }

    // Flatten pre and post states for output
    let inputs_outputs: Vec<Account> = vec![sender]
        .into_iter()
        .chain(receivers)
        .chain(vec![sender_post])
        .chain(receivers_post)
        .collect();

    env::commit(&inputs_outputs);
}
