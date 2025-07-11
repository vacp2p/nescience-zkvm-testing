use risc0_zkvm::{guest::env, sha::{Impl, Sha256}, serde::to_vec};
use toy_example_core::Account;

fn main() {
    let sender: Account = env::read();
    let receiver: Account = env::read();
    let balance_to_move: u128 = env::read();

    assert!(sender.balance >= balance_to_move);

    let mut sender_post = sender.clone();
    let mut receiver_post = receiver.clone();

    sender_post.balance -= balance_to_move;
    receiver_post.balance += balance_to_move;

    env::commit(&(sender, receiver, sender_post, receiver_post));
}
