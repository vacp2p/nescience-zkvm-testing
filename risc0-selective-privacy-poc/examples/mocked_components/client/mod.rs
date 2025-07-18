use core::{
    account::Account,
    types::{Address, Commitment, Key, Nullifier},
    visibility::InputVisibiility,
};

use nssa::program::TransferProgram;

use crate::mocked_components::sequencer::MockedSequencer;

pub mod transfer_deshielded;
pub mod transfer_private;
pub mod transfer_public;
pub mod transfer_shielded;

pub struct MockedClient {
    user_private_key: Key,
    private_accounts: Vec<Account>,
}

impl MockedClient {
    pub const fn new(user_private_key: Key) -> Self {
        Self {
            user_private_key,
            private_accounts: Vec::new(),
        }
    }

    pub fn user_address(&self) -> Address {
        Account::address_for_key(&self.user_private_key)
    }

    pub fn prove_and_send_to_sequencer<P: nssa::Program>(
        input_accounts: &[Account],
        instruction_data: P::InstructionData,
        visibilities: &[InputVisibiility],
        commitment_tree_root: [u32; 8],
        sequencer: &mut MockedSequencer,
    ) -> Result<Vec<Account>, ()> {
        let (receipt, private_outputs) = nssa::invoke_privacy_execution::<P>(
            input_accounts,
            instruction_data,
            visibilities,
            commitment_tree_root,
        )
        .unwrap();
        // Send to te sequencer
        sequencer.process_privacy_execution(receipt)?;

        Ok(private_outputs)
    }

    pub fn fresh_account_for_mint(address: Address) -> Account {
        Account::new(address, 0)
    }
}
