use transfer_methods::{
    TRANSFER_ELF, TRANSFER_ID
};
use outer_methods::{
    OUTER_ELF, OUTER_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use toy_example_core::account::Account;

/// A private execution of the transfer function.
/// This actually "burns" a sender private account and "mints" two new private accounts:
/// one for the recipient with the transferred balance, and another owned by the sender with the remaining balance.
fn run_private_execution_of_transfer_program() {
    let commitment_tree_root = [0xdd, 0xee, 0xaa, 0xdd, 0xbb, 0xee, 0xee, 0xff];
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
    let receiver = Account::new(receiver_address, [1; 8]);

    // Prove inner program and get post state of the accounts
    let (inner_receipt, sender_post, receiver_post) = prove_inner(sender.clone(), receiver.clone(), balance_to_move);

    // Prove outer program.
    // This computes the nullifier for the input account
    // and commitments for the accounts post states.
    let mut env_builder = ExecutorEnv::builder();
    env_builder.add_assumption(inner_receipt);
    env_builder.write(&sender_private_key).unwrap();
    env_builder.write(&sender).unwrap();
    env_builder.write(&receiver) .unwrap();
    env_builder.write(&sender_post).unwrap();
    env_builder.write(&receiver_post).unwrap();
    env_builder.write(&commitment_tree_root).unwrap();
    env_builder.write(&TRANSFER_ID).unwrap();
    let env = env_builder.build().unwrap();

    let prover = default_prover();
    let prove_info = prover
        .prove(env, OUTER_ELF)
        .unwrap();

    let receipt = prove_info.receipt;
    
    // Sanity check
    receipt.verify(OUTER_ID).unwrap();
    
    let output: [[u32; 8]; 3] = receipt.journal.decode().unwrap();
    println!("nullifier: {:?}", output[0]);
    println!("commitment_1: {:?}", output[1]);
    println!("commitment_2: {:?}", output[2]);
}

fn prove_inner(sender: Account, receiver: Account, balance_to_move: u128) -> (Receipt, Account, Account) {
    let mut env_builder = ExecutorEnv::builder();
    env_builder.write(&sender).unwrap();
    env_builder.write(&receiver).unwrap();
    env_builder.write(&balance_to_move).unwrap();
    let env = env_builder.build().unwrap();

    let prover = default_prover();
    let prove_info = prover
        .prove(env, TRANSFER_ELF)
        .unwrap();

    let receipt = prove_info.receipt;

    let output: [Account; 4] = receipt.journal.decode().unwrap();
    let [_, _, sender_post, receiver_post] = output;

    println!("sender_before: {:?}, sender_after: {:?}", sender, sender_post);
    println!("receiver_before: {:?}, receiver_after: {:?}", receiver, receiver_post);

    // Sanity check
    receipt
        .verify(TRANSFER_ID)
        .unwrap();

    (receipt, sender_post, receiver_post)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_private() {
        run_private_execution_of_transfer_program();
    }
}
