use core::types::Address;

use nssa::program::TransferProgram;

use crate::mocked_components::{client::MockedClient, sequencer::MockedSequencer};

impl MockedClient {
    pub fn transfer_public(
        &self,
        to_address: &Address,
        amount_to_transfer: u128,
        sequencer: &mut MockedSequencer,
    ) -> Result<(), ()> {
        // Submit a public (on-chain) execution of the Transfer program to the sequencer
        sequencer.process_public_execution::<TransferProgram>(&[self.user_address(), *to_address], amount_to_transfer)
    }
}
