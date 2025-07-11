use risc0_zkvm::{guest::env, sha::{Impl, Sha256}, serde::to_vec};
use toy_example_core::{Account, hash, compute_nullifier, is_in_commitment_tree};

fn main() {
    // Read inputs
    let account_1_private_key: [u32; 8] = env::read();
    let account_1: Account = env::read();
    let account_2: Account = env::read();
    let balance_to_move: u128 = env::read();
    let account_1_post: Account = env::read();
    let account_2_post: Account = env::read();
    let commitment_tree_root: [u32; 8] = env::read();
    let program_id: [u32; 8] = env::read();

    // Assert account_2 account is fresh
    assert_eq!(account_2.balance, 0);

    // Prove ownership of account_1 account by proving
    // knowledge of the pre-image of its address
    assert_eq!(hash(&account_1_private_key), account_1.address);

    // Compute account_1 account commitment and prove it belongs to commitments tree
    let account_1_commitment = account_1.commitment();
    assert!(is_in_commitment_tree(account_1_commitment, commitment_tree_root));

    // Compute nullifier of account_1 account
    let account_1_nullifier = compute_nullifier(account_1_commitment, account_1_private_key);

    // Compute accounts post states commitments
    let account_1_post_commitment = account_1_post.commitment();
    let account_2_post_commitment = account_2_post.commitment();

    // Verify pre states and post states of accounts are consistent
    // with the execution of the `program_id`` program
    env::verify(program_id, &to_vec(&(account_1.clone(), account_2.clone(), account_1_post.clone(), account_2_post.clone())).unwrap()).unwrap();

    // Assert `program_id` program didn't modify address fields
    assert_eq!(account_1.address, account_1_post.address);
    assert_eq!(account_2.address, account_2_post.address);

    // Output nullifier and commitments of new private accounts
    env::commit(&(account_1_nullifier, account_1_post_commitment, account_2_post_commitment));
}
