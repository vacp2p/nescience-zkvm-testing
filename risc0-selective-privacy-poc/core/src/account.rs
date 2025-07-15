use risc0_zkvm::{
    serde::to_vec,
    sha::{Impl, Sha256},
};
use serde::{Deserialize, Serialize};

pub type Commitment = u32;
pub type Nullifier = [u32; 8];
pub type Address = [u32; 8];
pub type Nonce = [u32; 8];
pub type Key = [u32; 8];
pub type AuthenticationPath = [[u32; 8]; 32];

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Account {
    pub address: Address,
    pub balance: u128,
    pub nonce: Nonce,
}

impl Account {
    /// Creates a new account with address = hash(private_key) and balance = 0
    pub fn new_from_private_key(private_key: Address, nonce: Nonce) -> Self {
        let address = hash(&private_key);
        Self {
            address,
            balance: 0,
            nonce,
        }
    }

    pub fn new(address: Address, nonce: Nonce) -> Self {
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

pub fn is_in_tree(commitment: Commitment, path: &AuthenticationPath, root: [u32; 8]) -> bool {
    const HASH_ONE: [u32; 8] = [
        789771595, 3310634292, 3140410939, 3820475020, 3591004369, 2777006897, 1021496535,
        2588247415,
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
