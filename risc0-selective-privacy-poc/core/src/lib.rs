pub mod account;
pub mod types;
pub mod visibility;

use crate::{account::Account, types::{AuthenticationPath, Commitment, Key, Nullifier}};
use risc0_zkvm::sha::{Impl, Sha256};

pub fn hash(bytes: &[u32]) -> [u32; 8] {
    Impl::hash_words(bytes).as_words().try_into().unwrap()
}

pub fn is_in_tree(commitment: Commitment, path: &AuthenticationPath, root: [u32; 8]) -> bool {
    const HASH_ONE: [u32; 8] = [
        789771595, 3310634292, 3140410939, 3820475020, 3591004369, 2777006897, 1021496535, 2588247415,
    ];

    let mut hash = HASH_ONE;
    let mut current_index = commitment;
    for path_value in path.iter() {
        if current_index & 1 == 0 {
            hash = hash_two(&hash, path_value);
        } else {
            hash = hash_two(path_value, &hash);
        }
        current_index >>= 1;
    }
    root == hash
}

/// Returns Hash(Commitment || private_key)
pub fn compute_nullifier(commitment: &Commitment, private_key: &Key) -> Nullifier {
    let mut bytes_to_hash = [0; 9]; // <- 1 word for the commitment, 8 words for the private key
    bytes_to_hash[..1].copy_from_slice(&[*commitment]);
    bytes_to_hash[1..].copy_from_slice(private_key);
    hash(&bytes_to_hash)
}

fn hash_two(left: &[u32; 8], right: &[u32; 8]) -> [u32; 8] {
    let mut bytes_to_hash = [0; 16];
    bytes_to_hash[..8].copy_from_slice(left);
    bytes_to_hash[8..].copy_from_slice(right);
    hash(&bytes_to_hash)
}

pub fn bytes_to_words(bytes: &[u8; 32]) -> [u32; 8] {
    let mut words = [0; 8];
    for (i, chunk) in bytes.chunks(4).enumerate() {
        words[i] = u32::from_le_bytes(chunk.try_into().unwrap());
    }
    words
}

/// Verifies that a program public execution didn't break the chain's rules.
/// `input_accounts` are the accounts provided as inputs to the program.
/// `output_accounts` are the accounts post states after execution of the program
pub fn inputs_outputs_preserve_invariants(input_accounts: &[Account], output_accounts: &[Account]) -> bool {
    // Fail if the number of input and output accounts differ
    if input_accounts.len() != output_accounts.len() {
        return false;
    }

    for (account_pre, account_post) in input_accounts.iter().zip(output_accounts) {
        // Fail if the program modified the addresses of the input accounts
        if account_pre.address != account_post.address {
            return false;
        }
        // Fail if the program modified the nonces of the input accounts
        if account_pre.nonce != account_post.nonce {
            return false;
        }
    }

    // Fail if the execution didn't preserve the total supply.
    let total_balance_pre: u128 = input_accounts.iter().map(|account| account.balance).sum();
    let total_balance_post: u128 = output_accounts.iter().map(|account| account.balance).sum();
    if total_balance_pre != total_balance_post {
        return false;
    }

    true
}
