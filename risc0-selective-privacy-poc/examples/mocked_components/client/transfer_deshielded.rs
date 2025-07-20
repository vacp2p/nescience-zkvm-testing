use super::{MockedClient, MockedSequencer};
use crate::mocked_components::sequencer::error::Error;
use core::account::Account;
use core::types::Address;
use core::visibility::AccountVisibility;
use nssa::program::TransferProgram;

impl MockedClient {
    /// A deshielded transaction of the Transfer program.
    /// All of this is executed locally by the sender
    pub fn transfer_deshielded(
        &self,
        from_account: Account,
        to_address: &Address,
        balance_to_move: u128,
        sequencer: &mut MockedSequencer,
    ) -> Result<Account, Error> {
        // Fetch commitment tree root from the sequencer
        let commitment_tree_root = sequencer.get_commitment_tree_root();
        // Compute authenticaton path for the input private account
        let sender_commitment_auth_path = sequencer.get_authentication_path_for(&from_account.commitment());

        // Fetch public account to deshield to
        let to_account = sequencer.get_account(to_address).unwrap();

        // Set account visibilities
        // First entry is the private sender. Second entry is the public receiver
        let visibilities = vec![
            AccountVisibility::Private(Some((self.user_private_key, sender_commitment_auth_path))),
            AccountVisibility::Public,
        ];

        // Execute privately (off-chain) and submit it to the sequencer
        let private_outputs = Self::prove_and_send_to_sequencer::<TransferProgram>(
            &[from_account, to_account],
            balance_to_move,
            &visibilities,
            commitment_tree_root,
            sequencer,
        )?;

        // There's only one private output account corresponding to the new private account of
        // the sender, with the remaining balance.
        let [sender_private_account] = private_outputs.try_into().unwrap();
        Ok(sender_private_account)
    }
}
