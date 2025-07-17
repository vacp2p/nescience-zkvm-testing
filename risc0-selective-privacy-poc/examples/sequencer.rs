use core::{
    account::Account,
    bytes_to_words, hash,
    input::InputVisibiility,
    types::{Address, Commitment, Key, Nullifier},
};

use nssa::program::{PinataProgram, TransferProgram};
use risc0_zkvm::Receipt;

use crate::sequencer_mock::{MockedSequencer, ACCOUNTS_PRIVATE_KEYS};

mod sequencer_mock;

fn main() {
    let mut sequencer = MockedSequencer::new();
    let addresses = sequencer.addresses();
    println!("addresses: {:?}", addresses);
    println!("Initial balances");
    sequencer.print();

    // A public execution of the Transfer Program
    let sender_addr = addresses[1];
    let receiver_addr = addresses[2];
    sequencer
        .invoke_public::<TransferProgram>(&[sender_addr, receiver_addr], 10)
        .unwrap();
    println!("Balances after transfer");
    sequencer.print();

    // A shielded execution of the Transfer Program
    let private_account_2 = send_shielded(&addresses[1], &addresses[2], 15, &mut sequencer);
    println!("Balances after shielded execution");
    sequencer.print();

    // A private execution of the Transfer Program
    let private_account_1 = send_private(
        &private_account_2,
        &ACCOUNTS_PRIVATE_KEYS[1], // <-- this is shifted 🫠
        &addresses[3],
        8,
        &mut sequencer,
    );
    println!("Balances after shielded execution");
    sequencer.print();

    // A deshielded execution of the Transfer Program
    send_deshielded(
        &private_account_1,
        &ACCOUNTS_PRIVATE_KEYS[0],
        &addresses[0],
        1,
        &mut sequencer,
    );
    println!("Balances after deshielded execution");
    sequencer.print();

    // A public execution of the Pinata program
    let preimage = bytes_to_words(b"NSSA Selective privacy is great!").to_vec();
    sequencer
        .invoke_public::<PinataProgram>(&[addresses[0], addresses[3]], preimage)
        .unwrap();
    println!("Balances after public piñata execution");
    sequencer.print();
}

fn mint_fresh_account(address: Address) -> Account {
    let nonce = [0; 8];
    Account::new(address, nonce)
}

/// A shielded execution of the Transfer program
fn send_shielded(
    from_address: &Address,
    to_address: &Address,
    balance_to_move: u128,
    sequencer: &mut MockedSequencer,
) -> Account {
    // All of this is executed locally by the sender
    let sender_account = sequencer.get_account(&from_address).unwrap();
    let commitment_tree_root = sequencer.get_commitment_tree_root();
    let receiver_addr = to_address;
    let mut receiver_account = mint_fresh_account(*receiver_addr);
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
fn send_private(
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
    let mut receiver_account = mint_fresh_account(*receiver_addr);
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

fn send_deshielded(
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
