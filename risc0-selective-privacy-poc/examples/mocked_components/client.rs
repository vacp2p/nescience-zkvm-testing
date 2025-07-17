use core::{
    account::Account,
    input::InputVisibiility,
    types::{Address, Commitment, Key, Nullifier},
};

use nssa::program::TransferProgram;

use crate::mocked_components::sequencer::MockedSequencer;

pub struct MockedClient;

impl MockedClient {
    pub fn fresh_account_for_mint(address: Address) -> Account {
        let nonce = [0; 8];
        Account::new(address, nonce)
    }

    /// A shielded execution of the Transfer program
    pub fn send_shielded(
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

    /// A private execution of the Transfer program
    pub fn send_private(
        from_account: &Account,
        from_account_pk: &Key,
        to_address: &Address,
        balance_to_move: u128,
        sequencer: &mut MockedSequencer,
    ) -> Account {
        // All of this is executed locally by the sender
        let commitment_tree_root = sequencer.get_commitment_tree_root();
        let receiver_addr = to_address;
        let sender_commitment_auth_path =
            sequencer.get_authentication_path_for(&from_account.commitment());
        let mut receiver_account = Self::fresh_account_for_mint(*receiver_addr);
        let visibilities = vec![
            InputVisibiility::Private(Some((from_account_pk.clone(), sender_commitment_auth_path))),
            InputVisibiility::Private(None),
        ];
        let (receipt, nonces) = nssa::invoke_privacy_execution::<TransferProgram>(
            &[from_account.clone(), receiver_account.clone()],
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

    pub fn send_deshielded(
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
