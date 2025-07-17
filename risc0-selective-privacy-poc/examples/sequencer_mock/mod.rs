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

pub struct MockedSequencer {
    accounts: BTreeMap<Address, Account>,
    commitment_tree: SparseMerkleTree,
    nullifier_set: HashSet<Nullifier>,
    deployed_program_ids: HashSet<ProgramId>,
}

pub const ACCOUNTS_PRIVATE_KEYS: [Key; 3] = [[1; 8], [2; 8], [3; 8]];
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

    pub fn invoke_privacy_execution(
        &mut self,
        receipt: Receipt,
        public_inputs_outputs: &[Account],
        nullifiers: &[Nullifier],
        commitments: &[Commitment],
    ) -> Result<(), ()> {
        let commitments_tree_root = self.get_commitment_tree_root();
        if public_inputs_outputs.len() % 2 != 0 {
            return Err(());
        }

        let num_input_public = public_inputs_outputs.len() >> 1;
        for account in public_inputs_outputs.iter().take(num_input_public) {
            let current_account = self.get_account(&account.address).ok_or(())?;
            if &current_account != account {
                return Err(());
            }
        }

        // Check that nullifiers have not been added before
        if nullifiers
            .iter()
            .any(|nullifier| self.nullifier_set.contains(nullifier))
        {
            return Err(());
        }

        // Check that commitments are new too
        if commitments
            .iter()
            .any(|commitment| self.commitment_tree.values().contains(commitment))
        {
            return Err(());
        }

        // Verify consistency between public accounts, nullifiers and commitments
        nssa::verify_privacy_execution(
            receipt,
            public_inputs_outputs,
            nullifiers,
            commitments,
            &commitments_tree_root,
        )?;

        // Update accounts
        public_inputs_outputs
            .iter()
            .cloned()
            .skip(num_input_public)
            .for_each(|account_post_state| {
                self.accounts
                    .insert(account_post_state.address, account_post_state);
            });

        // Add nullifiers
        self.nullifier_set.extend(nullifiers);

        // Add commitments
        for commitment in commitments.iter() {
            self.commitment_tree.add_value(*commitment);
        }

        Ok(())
    }

    pub fn invoke_public<P: nssa::Program>(
        &mut self,
        input_account_addresses: &[Address],
        instruction_data: P::InstructionData,
    ) -> Result<(), ()> {
        // Fetch accounts
        let input_accounts: Vec<Account> = input_account_addresses
            .iter()
            .map(|address| self.get_account(address).ok_or(()))
            .collect::<Result<_, _>>()?;

        // Execute
        let inputs_outputs = nssa::execute::<P>(&input_accounts, instruction_data)?;

        // Consistency checks
        self.inputs_outputs_are_consistent(&input_accounts, &inputs_outputs)?;

        // Update accounts
        inputs_outputs
            .into_iter()
            .skip(input_accounts.len())
            .for_each(|account_post_state| {
                self.accounts
                    .insert(account_post_state.address, account_post_state);
            });
        Ok(())
    }

    fn inputs_outputs_are_consistent(
        &self,
        input_accounts: &[Account],
        inputs_outputs: &[Account],
    ) -> Result<(), ()> {
        let num_inputs = input_accounts.len();
        if inputs_outputs.len() != num_inputs * 2 {
            return Err(());
        }

        let (claimed_accounts_pre, accounts_post) = inputs_outputs.split_at(num_inputs);
        if claimed_accounts_pre != input_accounts {
            return Err(());
        }

        for (account_pre, account_post) in input_accounts.iter().zip(accounts_post) {
            if account_pre.address != account_post.address {
                return Err(());
            }
            if account_pre.nonce != account_post.nonce {
                return Err(());
            }
            // Redundant with previous checks, but better make it explicit.
            if !self.accounts.contains_key(&account_post.address) {
                return Err(());
            }
        }
        let accounts_pre_total_balance: u128 =
            input_accounts.iter().map(|account| account.balance).sum();
        let accounts_post_total_balance: u128 =
            accounts_post.iter().map(|account| account.balance).sum();
        if accounts_pre_total_balance != accounts_post_total_balance {
            return Err(());
        }
        return Ok(());
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
        let formatted: Vec<String> = self.nullifier_set
            .iter()
            .map(|arr| format!("0x{:x}", arr[0]))
            .collect();
        println!("Nullifiers: [{}]", formatted.join(", "));
        println!("");
        println!("");
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
}
