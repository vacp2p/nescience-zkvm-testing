use core::{
    account::Account,
    bytes_to_words, hash,
    input::InputVisibiility,
    types::{Address, Commitment, Key, Nullifier},
};

use nssa::program::{PinataProgram, TransferProgram};
use risc0_zkvm::Receipt;

use crate::mocked_components::sequencer::MockedSequencer;
use crate::mocked_components::{client::MockedClient, ACCOUNTS_PRIVATE_KEYS};

mod mocked_components;

fn main() {
    let mut sequencer = MockedSequencer::new();
    let addresses = sequencer.addresses();
    println!("addresses: {:?}", addresses);
    println!("🚀 Initial balances");
    sequencer.print();

    // A public execution of the Transfer Program
    MockedClient::transfer_public(&addresses[1], &addresses[2], 10, &mut sequencer).unwrap();
    println!("🚀 Balances after transfer");
    sequencer.print();

    // A shielded execution of the Transfer Program
    let private_account_2 =
        MockedClient::transfer_shielded(&addresses[1], &addresses[2], 15, &mut sequencer).unwrap();
    println!("Balances after shielded execution");
    sequencer.print();

    // A private execution of the Transfer Program
    let [_, private_account_1] = MockedClient::transfer_private(
        private_account_2,
        &ACCOUNTS_PRIVATE_KEYS[1], // <-- this is shifted 🫠
        &addresses[3],
        8,
        &mut sequencer,
    )
    .unwrap();
    println!("🚀 Balances after shielded execution");
    sequencer.print();

    // A deshielded execution of the Transfer Program
    MockedClient::transfer_deshielded(
        private_account_1,
        &ACCOUNTS_PRIVATE_KEYS[0],
        &addresses[0],
        1,
        &mut sequencer,
    )
    .unwrap();
    println!("🚀 Balances after deshielded execution");
    sequencer.print();

    // A public execution of the Pinata program
    let preimage = bytes_to_words(b"NSSA Selective privacy is great!").to_vec();
    sequencer
        .process_public_execution::<PinataProgram>(&[addresses[0], addresses[3]], preimage)
        .unwrap();
    println!("🚀 Balances after public piñata execution");
    sequencer.print();
}
