#![cfg_attr(not(test), no_std)]

use serde::{Serialize, Deserialize};
use risc0_zkvm::{sha::{Impl, Sha256}, serde::to_vec};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Account {
    pub address: [u32; 8],
    pub balance: u128,
    pub nonce: [u32; 8]
}

impl Account {
    /// Creates a new account with address = hash(private_key) and balance = 0
    pub fn new_from_private_key(private_key: [u32; 8], nonce: [u32; 8]) -> Self {
        let address = hash(&private_key);
        Self { address, balance: 0, nonce }
    }

    /// Returns Hash(Account)
    pub fn commitment(&self) -> [u32; 8] {
        hash(&to_vec(&self).unwrap())
    }

}


pub fn hash(bytes: &[u32]) -> [u32; 8] {
    Impl::hash_words(&bytes).as_words().try_into().unwrap()
}

pub fn is_in_commitment_tree(_commitment: [u32; 8], _tree_root: [u32; 8]) -> bool {
    // Dummy implementation
    true
}

/// Returns Hash(Commitment || private_key)
pub fn compute_nullifier(commitment: [u32; 8], private_key: [u32; 8]) -> [u32; 8] {
    let mut bytes_to_hash = [0; 16];
    bytes_to_hash[..8].copy_from_slice(&commitment);
    bytes_to_hash[8..].copy_from_slice(&private_key);
    hash(&bytes_to_hash)
}

