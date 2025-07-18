use core::{
    account::Account,
    input::InputVisibiility,
    types::{Address, Commitment, Key, Nullifier},
};

use nssa::program::TransferProgram;

use crate::mocked_components::sequencer::MockedSequencer;

pub mod transfer_deshielded;
pub mod transfer_private;
pub mod transfer_public;
pub mod transfer_shielded;

pub struct MockedClient;

impl MockedClient {
    fn prove_and_send_to_sequencer<P: nssa::Program>(
        input_accounts: &[Account],
        instruction_data: P::InstructionData,
        visibilities: &[InputVisibiility],
        commitment_tree_root: [u32; 8],
        sequencer: &mut MockedSequencer,
    ) -> Vec<Account> {
        let (receipt, private_outputs) = nssa::invoke_privacy_execution::<P>(
            input_accounts,
            instruction_data,
            visibilities,
            commitment_tree_root,
        )
        .unwrap();
        // Send to te sequencer
        sequencer.process_privacy_execution(receipt).unwrap();

        private_outputs
    }

    pub fn fresh_account_for_mint(address: Address) -> Account {
        let nonce = [0; 8];
        Account::new(address, nonce)
    }
}
