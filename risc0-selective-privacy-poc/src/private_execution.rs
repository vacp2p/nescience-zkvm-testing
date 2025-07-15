use outer_methods::{OUTER_ELF, OUTER_ID};
use rand::{rngs::OsRng, Rng};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use toy_example_core::{
    account::{Account, Commitment},
    input::InputVisibiility,
};
use transfer_methods::{TRANSFER_ELF, TRANSFER_ID};

const COMMITMENT_TREE_ROOT: [u32; 8] = [0xdd, 0xee, 0xaa, 0xdd, 0xbb, 0xee, 0xee, 0xff];

pub fn new_random_nonce() -> [u32; 8] {
    let mut rng = OsRng;
    std::array::from_fn(|_| rng.gen())
}

fn mint_fresh_account(address: [u32; 8]) -> Account {
    let nonce = new_random_nonce();
    Account::new(address, nonce)
}

/// A private execution of the transfer function.
/// This actually "burns" a sender private account and "mints" two new private accounts:
/// one for the recipient with the transferred balance, and another owned by the sender with the remaining balance.
fn run_private_execution_of_transfer_program() {
    // This is supposed to be an existing private account (UTXO) with balance equal to 150.
    // And it is supposed to be a private account of the user running this private execution (hence the access to the private key)
    let sender_private_key = [0; 8];
    let sender = {
        // Creating it now but it's supposed to be already created by other previous transactions.
        let mut account = Account::new_from_private_key(sender_private_key, [1; 8]);
        account.balance = 150;
        account
    };

    let balance_to_move: u128 = 3;

    // This is the new private account (UTXO) being minted by this private execution.
    // (The `receiver_address` would be <Npk> in UTXO's terminology)
    let receiver_address = [99; 8];
    let receiver = mint_fresh_account(receiver_address);

    // Prove inner program and get post state of the accounts
    let (inner_receipt, inputs_outputs) = prove_inner(&sender, &receiver, balance_to_move);

    let visibilities = vec![
        InputVisibiility::Private(Some(sender_private_key)),
        InputVisibiility::Private(None),
    ];

    let num_inputs: u32 = inputs_outputs.len() as u32 / 2;

    // Sample fresh random nonces for the outputs of this execution
    let output_nonces: Vec<_> = (0..num_inputs).map(|_| new_random_nonce()).collect();
    println!("output nonces {output_nonces:?}");

    // Prove outer program.
    // This computes the nullifier for the input account
    // and commitments for the accounts post states.
    let mut env_builder = ExecutorEnv::builder();
    env_builder.add_assumption(inner_receipt);
    env_builder.write(&num_inputs).unwrap();
    env_builder.write(&inputs_outputs).unwrap();
    env_builder.write(&visibilities).unwrap();
    env_builder.write(&output_nonces).unwrap();
    env_builder.write(&COMMITMENT_TREE_ROOT).unwrap();
    env_builder.write(&TRANSFER_ID).unwrap();
    let env = env_builder.build().unwrap();

    let prover = default_prover();
    let prove_info = prover.prove(env, OUTER_ELF).unwrap();

    let receipt = prove_info.receipt;

    // Sanity check
    receipt.verify(OUTER_ID).unwrap();

    let output: (Vec<Account>, Vec<[u32; 8]>, Vec<Commitment>) = receipt.journal.decode().unwrap();
    println!("public_outputs: {:?}", output.0);
    println!("nullifiers: {:?}", output.1);
    println!("commitments: {:?}", output.2);
}

fn prove_inner(
    sender: &Account,
    receiver: &Account,
    balance_to_move: u128,
) -> (Receipt, Vec<Account>) {
    let mut env_builder = ExecutorEnv::builder();
    env_builder.write(&sender).unwrap();
    env_builder.write(&receiver).unwrap();
    env_builder.write(&balance_to_move).unwrap();
    let env = env_builder.build().unwrap();

    let prover = default_prover();
    let prove_info = prover.prove(env, TRANSFER_ELF).unwrap();

    let receipt = prove_info.receipt;

    let inputs_outputs: Vec<Account> = receipt.journal.decode().unwrap();
    assert_eq!(inputs_outputs.len(), 4);

    println!(
        "sender_before: {:?}, sender_after: {:?}",
        inputs_outputs[0], inputs_outputs[2]
    );
    println!(
        "receiver_before: {:?}, receiver_after: {:?}",
        inputs_outputs[1], inputs_outputs[3]
    );

    // Sanity check
    receipt.verify(TRANSFER_ID).unwrap();

    (receipt, inputs_outputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_private() {
        run_private_execution_of_transfer_program();
    }
}
