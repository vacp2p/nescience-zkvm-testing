use core::{
    account::Account,
    bytes_to_words,
    types::{Address, AuthenticationPath, Commitment, Key, Nullifier, ProgramId},
};
use std::collections::{BTreeMap, HashSet};

use program_methods::{PINATA_ID, TRANSFER_ID, TRANSFER_MULTIPLE_ID};
use sparse_merkle_tree::SparseMerkleTree;

use crate::mocked_components::USER_CLIENTS;

pub mod process_privacy_execution;
pub mod process_public_execution;

pub struct MockedSequencer {
    accounts: BTreeMap<Address, Account>,
    commitment_tree: SparseMerkleTree,
    nullifier_set: HashSet<Nullifier>,
    deployed_program_ids: HashSet<ProgramId>,
}

/// List of deployed programs
const DEPLOYED_PROGRAM_IDS: [ProgramId; 3] = [TRANSFER_ID, TRANSFER_MULTIPLE_ID, PINATA_ID];
/// The initial balance of the genesis accounts
const INITIAL_BALANCE: u128 = 150;
/// The address of the piñata program account
const PINATA_ADDRESS: Address = [0xcafe; 8];

impl MockedSequencer {
    pub fn new() -> Self {
        let mut accounts: BTreeMap<Address, Account> = USER_CLIENTS
            .iter()
            .map(|client| client.user_address())
            .map(|address| Account::new(address, INITIAL_BALANCE))
            .map(|account| (account.address, account))
            .collect();

        let pinata_account = Account::new(PINATA_ADDRESS, INITIAL_BALANCE);
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

    /// Returns the current state of the account for the given address
    pub fn get_account(&self, address: &Address) -> Option<Account> {
        self.accounts.get(address).cloned()
    }

    /// Returns the root of the commitment tree
    pub fn get_commitment_tree_root(&self) -> [u32; 8] {
        bytes_to_words(&self.commitment_tree.root())
    }

    /// Computes the authentication path for the given commitment
    pub fn get_authentication_path_for(&self, commitment: &Commitment) -> AuthenticationPath {
        self.commitment_tree
            .get_authentication_path_for_value(*commitment)
            .iter()
            .map(bytes_to_words)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    /// Returns the list of all registered addresses
    pub fn addresses(&self) -> Vec<Address> {
        self.accounts.keys().cloned().collect()
    }
}

/// Pretty prints the chain's state
pub fn print_accounts(sequencer: &MockedSequencer, private_accounts: &[&Account]) {
    println!("\n====================== ACCOUNT SNAPSHOT ======================\n");

    println!(">> Public Accounts:");
    println!("{:<20} | {:>10} |", "Address (first u32)", "Balance");
    println!("{:-<20}-+-{:-<10}", "", "");

    for account in sequencer.accounts.values() {
        println!("0x{:<20x} | {:>10} |", account.address[0], account.balance);
    }

    println!("{:-<20}-+-{:-<10}\n", "", "");

    println!(">> Commitments:");
    println!("{:-<20}", "");

    for commitment in sequencer.commitment_tree.values().iter() {
        println!("{:<20x}", commitment);
    }

    println!("{:-<20}\n", "");

    let formatted: Vec<String> = sequencer
        .nullifier_set
        .iter()
        .map(|nullifier| format!("0x{:x}", nullifier[0]))
        .collect();

    println!(">> Nullifiers (first u32):");
    println!("{:-<20}", "");

    for entry in formatted {
        println!("{:<20}", entry);
    }

    println!("{:-<20}\n", "");

    println!(">> Private Accounts:");
    println!("{:<20} | {:>10} | {:>10}", "Address (first u32)", "Nonce", "Balance");
    println!("{:-<20}-+-{:-<10}-+-{:-<10}", "", "", "");

    for account in private_accounts.iter() {
        println!(
            "{:<20x} | {:>10x}| {:>10} | ",
            account.address[0], account.nonce[0], account.balance,
        );
    }

    println!("{:-<20}-+-{:-<10}-+-{:-<10}", "", "", "");

    println!("\n=============================================================\n");
}
