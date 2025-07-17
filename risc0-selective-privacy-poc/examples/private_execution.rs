use nssa::program::TransferProgram;
use outer_methods::OUTER_ID;
use sparse_merkle_tree::SparseMerkleTree;
use toy_example_core::{
    account::Account,
    bytes_to_words,
    input::InputVisibiility,
    types::{Address, AuthenticationPath, Commitment, Nullifier},
};

fn mint_fresh_account(address: Address) -> Account {
    let nonce = [0; 8];
    Account::new(address, nonce)
}

/// A private execution of the transfer function.
/// This actually "burns" a sender private account and "mints" two new private accounts:
/// one for the recipient with the transferred balance, and another owned by the sender with the remaining balance.
fn main() {
    // This is supposed to be an existing private account (UTXO) with balance equal to 150.
    // And it is supposed to be a private account of the user running this private execution (hence the access to the private key)
    let sender_private_key = [1, 2, 3, 4, 4, 3, 2, 1];
    let sender = {
        // Creating it now but it's supposed to be already created by other previous transactions.
        let mut account = Account::new_from_private_key(sender_private_key, [1; 8]);
        account.balance = 150;
        account
    };

    let commitment_tree = SparseMerkleTree::new([sender.commitment()].into_iter().collect());
    let root = bytes_to_words(&commitment_tree.root());
    let auth_path: Vec<[u32; 8]> = commitment_tree
        .get_authentication_path_for_value(sender.commitment())
        .iter()
        .map(bytes_to_words)
        .collect();
    let auth_path: AuthenticationPath = auth_path.try_into().unwrap();

    let balance_to_move: u128 = 3;

    // This is the new private account (UTXO) being minted by this private execution. (The `receiver_address` would be <Npk> in UTXO's terminology)
    let receiver_address = [99; 8];
    let receiver = mint_fresh_account(receiver_address);

    let visibilities = vec![
        InputVisibiility::Private(Some((sender_private_key, auth_path))),
        InputVisibiility::Private(None),
    ];

    let receipt = nssa::execute_and_prove_privacy_execution::<TransferProgram>(
        &[sender, receiver],
        &balance_to_move,
        &visibilities,
        root,
    )
    .unwrap();

    let output: (Vec<Account>, Vec<Nullifier>, Vec<Commitment>, [u32; 8]) =
        receipt.journal.decode().unwrap();
    println!("public_outputs: {:?}", output.0);
    println!("nullifiers: {:?}", output.1);
    println!("commitments: {:?}", output.2);
    println!("commitment_tree_root: {:?}", output.3);

    assert!(
        nssa::verify_privacy_execution(receipt, &output.0, &output.1, &output.2, &output.3).is_ok()
    );
}
