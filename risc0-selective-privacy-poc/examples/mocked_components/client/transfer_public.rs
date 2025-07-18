use core::types::Address;

use nssa::program::TransferProgram;

use crate::mocked_components::{client::MockedClient, sequencer::MockedSequencer};

impl MockedClient {
    pub fn transfer_public(
        sender_address: &Address,
        receiver_address: &Address,
        amount_to_transfer: u128,
        sequencer: &mut MockedSequencer,
    ) -> Result<(), ()> {
        sequencer.process_public_execution::<TransferProgram>(
            &[*sender_address, *receiver_address],
            amount_to_transfer,
        )
    }
}
