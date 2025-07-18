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
    println!("1️⃣ 🚀 Initial balances");
    sequencer.print();

    // A public execution of the Transfer Program
    let sender_addr = addresses[1];
    let receiver_addr = addresses[2];
    sequencer
        .invoke_public::<TransferProgram>(&[sender_addr, receiver_addr], 10)
        .unwrap();
    println!("2️⃣ 🚀 Balances after transfer");
    sequencer.print();

    // A shielded execution of the Transfer Program
    let private_account_2 =
        MockedClient::send_shielded(&addresses[1], &addresses[2], 15, &mut sequencer);
    println!("Balances after shielded execution");
    sequencer.print();

    // A private execution of the Transfer Program
    let private_account_1 = MockedClient::send_private(
        &private_account_2,
        &ACCOUNTS_PRIVATE_KEYS[1], // <-- this is shifted 🫠
        &addresses[3],
        8,
        &mut sequencer,
    );
    println!("3️⃣ 🚀 Balances after shielded execution");
    sequencer.print();

    // A deshielded execution of the Transfer Program
    MockedClient::send_deshielded(
        &private_account_1,
        &ACCOUNTS_PRIVATE_KEYS[0],
        &addresses[0],
        1,
        &mut sequencer,
    );
    println!("4️⃣ 🚀 Balances after deshielded execution");
    sequencer.print();

    // A public execution of the Pinata program
    let preimage = bytes_to_words(b"NSSA Selective privacy is great!").to_vec();
    sequencer
        .invoke_public::<PinataProgram>(&[addresses[0], addresses[3]], preimage)
        .unwrap();
    println!("5️⃣ 🚀 Balances after public piñata execution");
    sequencer.print();
}
