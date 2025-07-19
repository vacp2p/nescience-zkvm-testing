use core::{
    account::Account,
    bytes_to_words,
    types::{Address, AuthenticationPath, Commitment, Nullifier},
    visibility::AccountVisibility,
};
use nssa::program::TransferMultipleProgram;
use sparse_merkle_tree::SparseMerkleTree;

/// A private execution of the TransferMultiple function.
/// This actually "burns" a sender private account and "mints" two new private accounts:
/// one for the recipient with the transferred balance, and another owned by the sender with the remaining balance.
fn main() {
    // Setup commitment tree, simulating a current chain state that has a private account already
    // committed to the commitment tree.
    let sender_private_key = [1, 2, 3, 4, 4, 3, 2, 1];
    let sender = {
        // Creating this here but it is supposed to be already in the private possesion of the user
        let mut account = Account::new_from_private_key(sender_private_key);
        account.balance = 150;
        account
    };
    let commitment_tree = SparseMerkleTree::new([sender.commitment()].into_iter().collect());

    // Get the root of the commitment tree and the authentication path of the commitment of the private account.
    let root = bytes_to_words(&commitment_tree.root());
    let auth_path: Vec<[u32; 8]> = commitment_tree
        .get_authentication_path_for_value(sender.commitment())
        .iter()
        .map(bytes_to_words)
        .collect();
    let auth_path: AuthenticationPath = auth_path.try_into().unwrap();

    // These are the new private account being minted by this private execution.
    // (the `receiver_address` would be <Npk> in UTXO's terminology)
    let receiver_address_1 = [99; 8];
    let receiver_1 = new_default_account(receiver_address_1);
    let receiver_address_2 = [100; 8];
    let receiver_2 = new_default_account(receiver_address_2);

    // Setup input account visibilites. All accounts are private for this execution.
    let visibilities = vec![
        AccountVisibility::Private(Some((sender_private_key, auth_path))),
        AccountVisibility::Private(None),
        AccountVisibility::Private(None),
    ];

    // Set the balances to be sent to the two receiver addresses.
    // This means, the execution will remove 70 tokens from the sender
    // and send 30 to the first receiver and 40 to the second.
    let balance_to_move = vec![30, 40];

    // Execute and prove the outer program for the TransferMultipleProgram.
    // This is executed off-chain by the sender.
    let (receipt, _) = nssa::execute_offchain::<TransferMultipleProgram>(
        &[sender, receiver_1, receiver_2],
        balance_to_move,
        &visibilities,
        root,
    )
    .unwrap();

    // Verify the proof
    assert!(nssa::verify_privacy_execution(receipt).is_ok());
    println!("OK!");
}

fn new_default_account(address: Address) -> Account {
    Account::new(address, 0)
}
