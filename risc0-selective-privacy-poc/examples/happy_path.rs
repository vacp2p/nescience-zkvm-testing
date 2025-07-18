use core::{
    account::Account,
    bytes_to_words, hash,
    input::InputVisibiility,
    types::{Address, Commitment, Key, Nullifier},
};

use nssa::program::{PinataProgram, TransferProgram};
use risc0_zkvm::Receipt;

use crate::mocked_components::sequencer::MockedSequencer;
use crate::mocked_components::{client::MockedClient, USER_CLIENTS};

mod mocked_components;

fn main() {
    let mut sequencer = MockedSequencer::new();
    let addresses: [Address; 3] = USER_CLIENTS.map(|client| client.user_address());

    println!("addresses: {:?}", addresses);
    println!("🚀 Initial balances");
    sequencer.print();

    // A public execution of the Transfer Program
    USER_CLIENTS[0]
        .transfer_public(&addresses[1], 10, &mut sequencer)
        .unwrap();
    println!("🚀 Balances after transfer");
    sequencer.print();

    // A shielded execution of the Transfer Program
    let private_account_1 = USER_CLIENTS[0]
        .transfer_shielded(&addresses[1], 15, &mut sequencer)
        .unwrap();
    println!("Balances after shielded execution");
    sequencer.print();

    // A private execution of the Transfer Program
    let [_, private_account_2] = USER_CLIENTS[1]
        .transfer_private(private_account_1, &addresses[2], 8, &mut sequencer)
        .unwrap();
    println!("🚀 Balances after shielded execution");
    sequencer.print();

    // A deshielded execution of the Transfer Program
    USER_CLIENTS[2]
        .transfer_deshielded(private_account_2, &addresses[0], 1, &mut sequencer)
        .unwrap();
    println!("🚀 Balances after deshielded execution");
    sequencer.print();

    // A public execution of the Pinata program
    let preimage = bytes_to_words(b"NSSA Selective privacy is great!").to_vec();
    sequencer
        .process_public_execution::<PinataProgram>(&[[0xcafe; 8], addresses[2]], preimage)
        .unwrap();
    println!("🚀 Balances after public piñata execution");
    sequencer.print();
}
