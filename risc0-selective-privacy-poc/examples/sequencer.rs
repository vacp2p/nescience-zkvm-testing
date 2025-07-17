use nssa::program::TransferProgram;

use crate::sequencer_mock::MockedSequencer;

mod sequencer_mock;

fn main() {
    let mut sequencer = MockedSequencer::new();
    let addresses = sequencer.addresses();
    println!("Initial balances");
    sequencer.print();

    // A public execution of the Transfer Program
    let sender_addr = addresses[0];
    let receiver_addr = addresses[1];
    sequencer
        .invoke_public::<TransferProgram>(&[sender_addr, receiver_addr], 10)
        .unwrap();

    println!("Balances after transfer");
    sequencer.print();


    // A private execution of the Transfer Program

}
