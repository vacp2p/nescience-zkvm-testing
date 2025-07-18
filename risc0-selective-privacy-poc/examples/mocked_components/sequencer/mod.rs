use core::{
    account::Account,
    bytes_to_words,
    types::{Address, AuthenticationPath, Commitment, Key, Nullifier, ProgramId},
};
use std::collections::{BTreeMap, HashSet};

use nssa;
use program_methods::{PINATA_ID, TRANSFER_ID, TRANSFER_MULTIPLE_ID};
use risc0_zkvm::Receipt;
use sparse_merkle_tree::SparseMerkleTree;

use super::ACCOUNTS_PRIVATE_KEYS;

pub mod invoke_privacy_execution;
pub mod invoke_public_execution;

pub struct MockedSequencer {
    accounts: BTreeMap<Address, Account>,
    commitment_tree: SparseMerkleTree,
    nullifier_set: HashSet<Nullifier>,
    deployed_program_ids: HashSet<ProgramId>,
}

const ACCOUNTS_INITIAL_BALANCES: [u128; 3] = [100, 1337, 37];
const DEPLOYED_PROGRAM_IDS: [ProgramId; 3] = [TRANSFER_ID, TRANSFER_MULTIPLE_ID, PINATA_ID];

impl MockedSequencer {
    pub fn new() -> Self {
        let mut accounts: BTreeMap<Address, Account> = ACCOUNTS_PRIVATE_KEYS
            .iter()
            .cloned()
            .zip(ACCOUNTS_INITIAL_BALANCES)
            .map(|(key, initial_balance)| {
                let mut this = Account::new_from_private_key(key, [0; 8]);
                this.balance = initial_balance;
                this
            })
            .map(|account| (account.address, account))
            .collect();

        let pinata_account = {
            let mut this = Account::new([0xcafe; 8], [0; 8]);
            this.balance = 100;
            this
        };
        accounts.insert(pinata_account.address, pinata_account);

        let commitment_tree = SparseMerkleTree::new_empty();
        let nullifier_set = HashSet::new();
        Self {
            accounts,
            commitment_tree,
            nullifier_set,
            deployed_program_ids: DEPLOYED_PROGRAM_IDS.iter().cloned().collect(),
        }
    }

    pub fn get_account(&self, address: &Address) -> Option<Account> {
        self.accounts.get(address).cloned()
    }

    pub fn get_commitment_tree_root(&self) -> [u32; 8] {
        bytes_to_words(&self.commitment_tree.root())
    }

    pub fn get_authentication_path_for(&self, commitment: &Commitment) -> AuthenticationPath {
        self.commitment_tree
            .get_authentication_path_for_value(*commitment)
            .iter()
            .map(bytes_to_words)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    pub fn addresses(&self) -> Vec<Address> {
        self.accounts.keys().cloned().collect()
    }

    pub fn print(&self) {
        println!("{:<20} | {:>10}", "Address (first u32)", "Balance");
        println!("{:-<20}-+-{:-<10}", "", "");

        for account in self.accounts.values() {
            println!("{:<20x} | {:>10}", account.address[0], account.balance);
        }
        println!("{:-<20}-+-{:-<10}", "", "");
        println!("Commitments: {:?}", self.commitment_tree.values());
        let formatted: Vec<String> = self
            .nullifier_set
            .iter()
            .map(|arr| format!("0x{:x}", arr[0]))
            .collect();
        println!("Nullifiers: [{}]", formatted.join(", "));
        println!("");
        println!("");
    }
}
