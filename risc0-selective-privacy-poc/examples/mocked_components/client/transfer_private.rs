use core::account::Account;
use core::visibility::InputVisibiility;
use core::types::{Address, Commitment, Key, Nullifier};

use nssa::program::TransferProgram;

use super::{MockedClient, MockedSequencer};

impl MockedClient {
    /// A private execution of the Transfer program
    pub fn transfer_private(
        &self,
        owned_private_account: Account,
        to_address: &Address,
        balance_to_move: u128,
        sequencer: &mut MockedSequencer,
    ) -> Result<[Account; 2], ()> {
        // All of this is executed locally by the sender
        let commitment_tree_root = sequencer.get_commitment_tree_root();
        let receiver_addr = to_address;
        let sender_commitment_auth_path =
            sequencer.get_authentication_path_for(&owned_private_account.commitment());
        let mut receiver_account = Self::fresh_account_for_mint(*receiver_addr);
        let visibilities = vec![
            InputVisibiility::Private(Some((self.user_private_key, sender_commitment_auth_path))),
            InputVisibiility::Private(None),
        ];

        let private_outputs = Self::prove_and_send_to_sequencer::<TransferProgram>(
            &[owned_private_account, receiver_account],
            balance_to_move,
            &visibilities,
            commitment_tree_root,
            sequencer,
        )?;

        Ok(private_outputs.try_into().unwrap())
    }
}
