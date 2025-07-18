use core::account::Account;
use core::types::{Address, Commitment, Key, Nullifier};
use core::visibility::InputVisibiility;

use nssa::program::TransferProgram;

use super::{MockedClient, MockedSequencer};

impl MockedClient {
    /// A private execution of the Transfer program
    // All of this is executed locally by the sender
    pub fn transfer_private(
        &self,
        owned_private_account: Account,
        to_address: &Address,
        balance_to_move: u128,
        sequencer: &mut MockedSequencer,
    ) -> Result<[Account; 2], ()> {
        // Fetch commitment tree root from the sequencer
        let commitment_tree_root = sequencer.get_commitment_tree_root();
        // Compute authenticaton path for the input private account
        let sender_commitment_auth_path = sequencer.get_authentication_path_for(&owned_private_account.commitment());

        // Create a new default private account for the recipient
        let mut receiver_account = Self::fresh_account_for_mint(*to_address);

        // Set visibilities. Both private accounts.
        let visibilities = vec![
            InputVisibiility::Private(Some((self.user_private_key, sender_commitment_auth_path))),
            InputVisibiility::Private(None),
        ];

        // Execute privately (off-chain) and submit it to the sequencer
        let private_outputs = Self::prove_and_send_to_sequencer::<TransferProgram>(
            &[owned_private_account, receiver_account],
            balance_to_move,
            &visibilities,
            commitment_tree_root,
            sequencer,
        )?;

        // There are two output private accounts of this execution.
        // The first corresponds to the sender, with the remaining balance.
        // The second corresponds to the newly minted private account for the recipient.
        let [sender_private_account, receiver_private_account] = private_outputs.try_into().unwrap();
        Ok([sender_private_account, receiver_private_account])
    }
}
