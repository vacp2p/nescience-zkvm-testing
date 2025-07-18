use core::account::Account;
use core::visibility::InputVisibiility;
use core::types::{Address, Commitment, Key, Nullifier};

use nssa::program::TransferProgram;

use super::{MockedClient, MockedSequencer};

impl MockedClient {
    pub fn transfer_deshielded(
        &self,
        from_account: Account,
        to_address: &Address,
        balance_to_move: u128,
        sequencer: &mut MockedSequencer,
    ) -> Result<Account, ()> {
        // All of this is executed locally by the sender
        let commitment_tree_root = sequencer.get_commitment_tree_root();
        let receiver_addr = to_address;
        // let from_account = sequencer.get_account(&self.user_address()).ok_or(())?;
        let sender_commitment_auth_path =
            sequencer.get_authentication_path_for(&from_account.commitment());
        let to_account = sequencer.get_account(&to_address).unwrap();
        let visibilities = vec![
            InputVisibiility::Private(Some((self.user_private_key, sender_commitment_auth_path))),
            InputVisibiility::Public,
        ];
        let private_outputs = Self::prove_and_send_to_sequencer::<TransferProgram>(
            &[from_account, to_account],
            balance_to_move,
            &visibilities,
            commitment_tree_root,
            sequencer,
        )?;
        let [sender_private_account] = private_outputs.try_into().unwrap();
        Ok(sender_private_account)
    }
}
