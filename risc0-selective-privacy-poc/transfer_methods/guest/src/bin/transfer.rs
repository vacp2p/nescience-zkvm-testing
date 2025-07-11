use risc0_zkvm::{guest::env, sha::{Impl, Sha256}, serde::to_vec};
use toy_example_core::Account;


/// A transfer of balance program.
/// To be used both in public and private contexts.
fn main() {
    let sender: Account = env::read();
    let receiver: Account = env::read();
    let balance_to_move: u128 = env::read();

    // Check sender has enough balance
    assert!(sender.balance >= balance_to_move);

    // Create accounts post states, with updated balances
    let mut sender_post = sender.clone();
    let mut receiver_post = receiver.clone();
    sender_post.balance -= balance_to_move;
    receiver_post.balance += balance_to_move;

    env::commit(&(sender, receiver, sender_post, receiver_post));
}
