use crate::{
    hash,
    types::{Address, Commitment, Key, Nonce, ProgramId},
};
use risc0_zkvm::serde::to_vec;
use serde::{Deserialize, Serialize};

/// Account to be used both in public and private contexts
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Account {
    pub program_owner: Option<ProgramId>,
    pub address: Address,
    pub balance: u128,
    pub nonce: Nonce,
}

impl Account {
    pub fn new(address: Address, balance: u128) -> Self {
        Self {
            program_owner: None,
            address,
            balance,
            nonce: [0; 8],
        }
    }

    /// Creates a new account with address = hash(private_key) and balance = 0
    pub fn new_from_private_key(private_key: Key) -> Self {
        let address = Self::address_for_key(&private_key);
        Self::new(address, 0)
    }

    /// Computes the address corresponding to the given private key
    pub fn address_for_key(private_key: &Key) -> Address {
        hash(private_key)
    }

    /// Returns (first 8 bytes of) SHA256(Account)
    pub fn commitment(&self) -> Commitment {
        hash(&to_vec(&self).unwrap())[0]
    }
}
