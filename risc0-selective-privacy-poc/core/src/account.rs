use crate::{
    hash,
    types::{Address, Commitment, Key, Nonce},
};
use risc0_zkvm::{serde::to_vec, sha::Impl};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Account {
    pub address: Address,
    pub balance: u128,
    pub nonce: Nonce,
}

impl Account {
    /// Creates a new account with address = hash(private_key) and balance = 0
    pub fn new_from_private_key(private_key: Key, nonce: Nonce) -> Self {
        let address = Self::address_for_key(&private_key);
        Self::new(address, nonce)
    }

    pub fn address_for_key(private_key: &Key) -> Address {
        hash(private_key)
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
