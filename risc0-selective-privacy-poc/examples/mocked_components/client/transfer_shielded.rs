use core::account::Account;
use core::input::InputVisibiility;
use core::types::{Address, Commitment, Key, Nullifier};

use nssa::program::TransferProgram;

use super::{MockedClient, MockedSequencer};

impl MockedClient {
    pub fn transfer_deshielded(
        from_account: &Account,
        from_account_pk: &Key,
        to_address: &Address,
        balance_to_move: u128,
        sequencer: &mut MockedSequencer,
    ) {
        // All of this is executed locally by the sender
        let commitment_tree_root = sequencer.get_commitment_tree_root();
        let receiver_addr = to_address;
        let sender_commitment_auth_path =
            sequencer.get_authentication_path_for(&from_account.commitment());
        let to_account = sequencer.get_account(&to_address).unwrap();
        let visibilities = vec![
            InputVisibiility::Private(Some((from_account_pk.clone(), sender_commitment_auth_path))),
            InputVisibiility::Public,
        ];
        let (receipt, nonces) = nssa::invoke_privacy_execution::<TransferProgram>(
            &[from_account.clone(), to_account],
            balance_to_move,
            &visibilities,
            commitment_tree_root,
        )
        .unwrap();

        let output: (Vec<Account>, Vec<Nullifier>, Vec<Commitment>, [u32; 8]) =
            receipt.journal.decode().unwrap();

        // Send to te sequencer
        sequencer
            .invoke_privacy_execution(receipt, &output.0, &output.1, &output.2)
            .unwrap();
    }
}
