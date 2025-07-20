use core::account::Account;
use core::types::Address;
use core::visibility::AccountVisibility;

use nssa::program::TransferProgram;

use super::{MockedClient, MockedSequencer};

impl MockedClient {
    /// A shielded execution of the Transfer program
    // All of this is executed locally by the sender
    pub fn transfer_shielded(
        &self,
        to_address: &Address,
        balance_to_move: u128,
        sequencer: &mut MockedSequencer,
    ) -> Result<Account, nssa::Error> {
        // Fetch commitment tree root from the sequencer
        let commitment_tree_root = sequencer.get_commitment_tree_root();

        // Fetch sender account from the sequencer
        let from_account = sequencer
            .get_account(&self.user_address())
            .ok_or(nssa::Error::Generic)?;

        // Create a new default private account for the receiver
        let to_account = Self::fresh_account_for_mint(*to_address);

        // Set account visibilities
        // First is the public account of the sender. Second is the private account minted in this
        // execution
        let visibilities = [AccountVisibility::Public, AccountVisibility::Private(None)];

        // Execute privately (off-chain) and submit it to the sequencer
        let private_outputs = Self::prove_and_send_to_sequencer::<TransferProgram>(
            &[from_account, to_account],
            balance_to_move,
            &visibilities,
            commitment_tree_root,
            sequencer,
        )?;

        // There is only one private account as the output of this execution. It corresponds to the
        // receiver.
        let [receiver_private_account] = private_outputs.try_into().unwrap();
        Ok(receiver_private_account)
    }
}
