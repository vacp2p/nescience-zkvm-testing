use core::{
    account::Account,
    bytes_to_words, hash,
    types::{Address, Commitment, Key, Nullifier},
    visibility::InputVisibiility,
};

use nssa::program::{PinataProgram, TransferProgram};
use risc0_zkvm::Receipt;

use crate::mocked_components::sequencer::{print_accounts, MockedSequencer};
use crate::mocked_components::{client::MockedClient, USER_CLIENTS};

mod mocked_components;

fn main() {
    let mut sequencer = MockedSequencer::new();
    let addresses: [Address; 3] = USER_CLIENTS.map(|client| client.user_address());
    println!("📝 Initial balances");
    print_accounts(&sequencer, &[]);

    // A public execution of the Transfer Program
    USER_CLIENTS[1]
        .transfer_public(&[0xcafe; 8], 51, &mut sequencer)
        .unwrap();
    println!("📝 Balances after transfer");
    print_accounts(&sequencer, &[]);

    // A shielded execution of the Transfer Program
    let private_account_user_1 = USER_CLIENTS[0]
        .transfer_shielded(&addresses[1], 15, &mut sequencer)
        .unwrap();
    println!("📝 Balances after shielded execution");
    print_accounts(&sequencer, &[&private_account_user_1]);

    // A private execution of the Transfer Program
    let [private_account_user_1, private_account_user_2] = USER_CLIENTS[1]
        .transfer_private(private_account_user_1, &addresses[2], 8, &mut sequencer)
        .unwrap();
    println!("📝 Balances after shielded execution");
    print_accounts(
        &sequencer,
        &[&private_account_user_1, &private_account_user_2],
    );

    // A deshielded execution of the Transfer Program
    let private_acount_user_2 = USER_CLIENTS[2]
        .transfer_deshielded(private_account_user_2, &addresses[0], 1, &mut sequencer)
        .unwrap();
    println!("📝 Balances after deshielded execution");
    print_accounts(
        &sequencer,
        &[&private_account_user_1, &private_acount_user_2],
    );

    // A public execution of the Piñata program
    let preimage = bytes_to_words(b"NSSA Selective privacy is great!").to_vec();
    sequencer
        .process_public_execution::<PinataProgram>(&[[0xcafe; 8], addresses[2]], preimage)
        .unwrap();
    println!("📝 Balances after public piñata execution");
    print_accounts(
        &sequencer,
        &[&private_account_user_1, &private_acount_user_2],
    );

    // A deshielded execution of the Piñata program
    let private_account_user_0 = {
        // All of this is executed locally by the sender
        let receiver_addr = USER_CLIENTS[1].user_address();
        let pinata_account = sequencer.get_account(&[0xcafe; 8]).unwrap();
        let mut receiver_account = MockedClient::fresh_account_for_mint(receiver_addr);
        let visibilities = [InputVisibiility::Public, InputVisibiility::Private(None)];
        let preimage = bytes_to_words(b"NSSA Selective privacy is great!").to_vec();

        let private_outputs = MockedClient::prove_and_send_to_sequencer::<PinataProgram>(
            &[pinata_account, receiver_account],
            preimage,
            &visibilities,
            sequencer.get_commitment_tree_root(),
            &mut sequencer,
        )
        .unwrap();
        let [private_account_user_0] = private_outputs.try_into().unwrap();
        private_account_user_0
    };
    println!("📝 Balances after private piñata execution");
    print_accounts(
        &sequencer,
        &[
            &private_account_user_1,
            &private_acount_user_2,
            &private_account_user_0,
        ],
    );
}
