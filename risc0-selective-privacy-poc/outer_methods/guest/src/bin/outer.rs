use risc0_zkvm::{guest::env, sha::{Impl, Sha256}, serde::to_vec};
use toy_example_core::account::{Account, hash, compute_nullifier, is_in_commitment_tree};

/// Private execution logic.
/// Circuit for proving correct execution of some program with program id
/// equal to `program_id` (last input).
/// 
/// Currently only supports private execution of a program with two input accounts, one
/// of which must be a fresh new account (`account_2`) (for example a private transfer function).
/// 
/// This circuit checks:
/// - That accounts pre states and post states are consistent with the execution of the given `program_id`.
/// - That `account_2` is fresh (meaning, for this toy example, that it has 0 balance).
/// - That `program_id` execution didn't change addresses of the accounts.
/// 
/// Outputs:
/// - The nullifier for the only existing input account (account_1)
/// - The commitments for the private accounts post states.
fn main() {
    // Read inputs
    let account_1_private_key: [u32; 8] = env::read();
    let account_1: Account = env::read();
    let account_2: Account = env::read();
    let account_1_post: Account = env::read();
    let account_2_post: Account = env::read();
    let commitment_tree_root: [u32; 8] = env::read();
    let program_id: [u32; 8] = env::read();

    // Assert account_2 is a fresh account
    assert_eq!(account_2.balance, 0);

    // Prove ownership of account_1 account by proving
    // knowledge of the pre-image of its address
    assert_eq!(hash(&account_1_private_key), account_1.address);

    // Compute account_1 account commitment and prove it belongs to commitments tree
    let account_1_commitment = account_1.commitment();
    assert!(is_in_commitment_tree(account_1_commitment, commitment_tree_root)); // <- Dummy implementation

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
