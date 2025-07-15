use risc0_zkvm::{
    serde::to_vec,
    sha::{Impl, Sha256},
};
use serde::{Deserialize, Serialize};

pub type Commitment = u32;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Account {
    pub address: [u32; 8],
    pub balance: u128,
    pub nonce: [u32; 8],
}

impl Account {
    /// Creates a new account with address = hash(private_key) and balance = 0
    pub fn new_from_private_key(private_key: [u32; 8], nonce: [u32; 8]) -> Self {
        let address = hash(&private_key);
        Self {
            address,
            balance: 0,
            nonce,
        }
    }

    pub fn new(address: [u32; 8], nonce: [u32; 8]) -> Self {
        Self {
            address,
            balance: 0,
            nonce,
        }
    }

    /// Returns Hash(Account)[0] (only first word for this POC)
    pub fn commitment(&self) -> Commitment {
        hash(&to_vec(&self).unwrap())[0]
    }
}

pub fn hash(bytes: &[u32]) -> [u32; 8] {
    Impl::hash_words(bytes).as_words().try_into().unwrap()
}

/// Dummy implementation
pub fn is_in_commitment_tree(_commitment: Commitment, _tree_root: [u32; 8]) -> bool {
    true
}

/// Returns Hash(Commitment || private_key)
pub fn compute_nullifier(commitment: &Commitment, private_key: &[u32; 8]) -> [u32; 8] {
    let mut bytes_to_hash = [0; 9]; // <- 1 word for the commitment, 8 words for the private key
    bytes_to_hash[..1].copy_from_slice(&[*commitment]);
    bytes_to_hash[1..].copy_from_slice(private_key);
    hash(&bytes_to_hash)
}
