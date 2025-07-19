use crate::mocked_components::sequencer::MockedSequencer;
use core::{
    account::Account,
    types::{Address, Key},
    visibility::AccountVisibility,
};

pub mod transfer_deshielded;
pub mod transfer_private;
pub mod transfer_public;
pub mod transfer_shielded;

/// A client that creates and submits transfer transactions
pub struct MockedClient {
    user_private_key: Key,
}

impl MockedClient {
    pub const fn new(user_private_key: Key) -> Self {
        Self { user_private_key }
    }

    pub fn user_address(&self) -> Address {
        Account::address_for_key(&self.user_private_key)
    }

    /// Runs the outer program and submits the proof to the sequencer.
    /// Returns the output private accounts of the execution.
    pub fn prove_and_send_to_sequencer<P: nssa::Program>(
        input_accounts: &[Account],
        instruction_data: P::InstructionData,
        visibilities: &[AccountVisibility],
        commitment_tree_root: [u32; 8],
        sequencer: &mut MockedSequencer,
    ) -> Result<Vec<Account>, ()> {
        // Execute and generate proof of the outer program
        let (receipt, private_outputs) =
            nssa::execute_offchain::<P>(input_accounts, instruction_data, visibilities, commitment_tree_root).unwrap();

        // Send proof to the sequencer
        sequencer.process_privacy_execution(receipt)?;

        // Return private outputs
        Ok(private_outputs)
    }

    /// Returns a new account used in privacy executions that mint private accounts
    pub fn fresh_account_for_mint(address: Address) -> Account {
        Account::new(address, 0)
    }
}
