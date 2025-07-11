use risc0_zkvm::{guest::env, sha::{Impl, Sha256}, serde::to_vec};
use toy_example_core::{Account, hash, compute_nullifier, is_in_commitment_tree};
use transfer_methods::TRANSFER_ID;

fn main() {
    // Read inputs
    let sender_private_key: [u32; 8] = env::read();
    let sender: Account = env::read();
    let receiver: Account = env::read();
    let balance_to_move: u128 = env::read();
    let sender_post: Account = env::read();
    let receiver_post: Account = env::read();
    let commitment_tree_root: [u32; 8] = env::read();

    // Assert receiver account is fresh
    assert_eq!(receiver.balance, 0);

    // Prove ownership of sender account by proving
    // knowledge of the pre-image of its address
    assert_eq!(hash(&sender_private_key), sender.address);

    // Compute sender account commitment and prove it belongs to commitments tree
    let sender_commitment = sender.commitment();
    assert!(is_in_commitment_tree(sender_commitment, commitment_tree_root));

    // Compute nullifier of sender account
    let sender_nullifier = compute_nullifier(sender_commitment, sender_private_key);

    // Compute receiver commitment
    let receiver_commitment = receiver_post.commitment();

    // Verify pre states and post states of accounts are consistent
    // with the execution of the TRANSFER_ELF program
    env::verify(TRANSFER_ID, &to_vec(&(sender.clone(), receiver.clone(), sender_post.clone(), receiver_post.clone())).unwrap()).unwrap();

    // Assert TRANSFER_ELF program didn't modify address fields
    assert_eq!(sender.address, sender_post.address);
    assert_eq!(receiver.address, receiver_post.address);

    // Output nullifier
    env::commit(&(sender_nullifier, receiver_commitment));
}
