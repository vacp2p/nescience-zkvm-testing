use core::account::Account;
use core::input::InputVisibiility;
use core::types::{Address, Commitment, Key, Nullifier};

use nssa::program::TransferProgram;

use super::{MockedClient, MockedSequencer};

impl MockedClient {
    /// A shielded execution of the Transfer program
    pub fn transfer_shielded(
        from_address: &Address,
        to_address: &Address,
        balance_to_move: u128,
        sequencer: &mut MockedSequencer,
    ) -> Account {
        // All of this is executed locally by the sender
        let sender_account = sequencer.get_account(&from_address).unwrap();
        let commitment_tree_root = sequencer.get_commitment_tree_root();
        let receiver_addr = to_address;
        let mut receiver_account = Self::fresh_account_for_mint(*receiver_addr);
        let visibilities = [InputVisibiility::Public, InputVisibiility::Private(None)];

        let private_outputs = Self::prove_and_send_to_sequencer::<TransferProgram>(
            &[sender_account, receiver_account],
            balance_to_move,
            &visibilities,
            commitment_tree_root,
            sequencer,
        );
        let [receiver_private_account] = private_outputs.try_into().unwrap();
        receiver_private_account
    }

}
