use core::{
    account::Account,
    types::{Address, Key, Nullifier, ProgramId},
};
use std::collections::{HashMap, HashSet};

use nssa;
use program_methods::{PINATA_ID, TRANSFER_ID, TRANSFER_MULTIPLE_ID};
use sparse_merkle_tree::SparseMerkleTree;

pub struct MockedSequencer {
    accounts: HashMap<Address, Account>,
    commitment_tree: SparseMerkleTree,
    nullifier_set: HashSet<Nullifier>,
    deployed_program_ids: HashSet<ProgramId>,
}

const ACCOUNTS_PRIVATE_KEYS: [Key; 5] = [[1; 8], [2; 8], [3; 8], [4; 8], [5; 8]];
const ACCOUNTS_INITIAL_BALANCES: [u128; 5] = [100, 3, 900, 44, 0];
const DEPLOYED_PROGRAM_IDS: [ProgramId; 3] = [TRANSFER_ID, TRANSFER_MULTIPLE_ID, PINATA_ID];

impl MockedSequencer {
    pub fn new() -> Self {
        let mut accounts: HashMap<Address, Account> = ACCOUNTS_PRIVATE_KEYS
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
            deployed_program_ids: DEPLOYED_PROGRAM_IDS.iter().collect(),
        }
    }

    pub fn get_account(&self, address: &Address) -> Option<Account> {
        self.accounts.get(address).cloned()
    }

    pub fn invoke_program_public<P: nssa::Program>(
        &mut self,
        program_id: ProgramId,
        input_account_addresses: &[Address],
        instruction_data: &P::InstructionData,
    ) -> Result<(), ()> {
        let input_accounts: Vec<Account> = input_account_addresses
            .iter()
            .map(|address| self.get_account(address).ok_or(|_| ())?)
            .collect();
        let inputs_outputs = nssa::execute(input_accounts, instruction_data)?;

        self.inputs_outputs_are_consistent(&input_accounts, &inputs_outputs)?
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
        }
        let accounts_pre_total_balance = input_accounts.iter().map(|account| account.balance).sum();
        let accounts_post_total_balance = accounts_post.iter().map(|account| account.balance).sum();
        if accounts_pre_total_balance != accounts_post_total_balance {
            return Err(());
        }
        return Ok(());
    }
}
