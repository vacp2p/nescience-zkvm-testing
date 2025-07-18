use core::account::Account;
use core::input::InputVisibiility;
use core::types::{Address, Commitment, Key, Nullifier};

use nssa::program::TransferProgram;

use super::{MockedSequencer, MockedClient};

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
        let visibilities = vec![InputVisibiility::Public, InputVisibiility::Private(None)];
        let (receipt, nonces) = nssa::invoke_privacy_execution::<TransferProgram>(
            &[sender_account, receiver_account.clone()],
            balance_to_move,
            &visibilities,
            commitment_tree_root,
        )
        .unwrap();

        // Assemble the private account
        receiver_account.nonce = nonces[1];
        receiver_account.balance = balance_to_move;
        let output: (Vec<Account>, Vec<Nullifier>, Vec<Commitment>, [u32; 8]) =
            receipt.journal.decode().unwrap();

        // Send to te sequencer
        sequencer
            .invoke_privacy_execution(receipt, &output.0, &output.1, &output.2)
            .unwrap();
        receiver_account
    }
}
