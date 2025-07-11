use risc0_zkvm::{default_executor, ExecutorEnv};
use toy_example_core::Account;
use transfer_methods::TRANSFER_ELF;

/// A public execution.
/// This would be executed by the runtime after checking that
/// the initiating transaction includes the sender's signature.
pub fn run_public_execution_of_transfer_program() {
    // Account fetched from the chain state with 150 in its balance.
    let sender = {
        let mut account = Account::new([5; 8], [98; 8]);
        account.balance = 150;
        account
    };

    // Account fetched from the chain state with 900 in its balance.
    let receiver = {
        let mut account = Account::new([6; 8], [99; 8]);
        account.balance = 900;
        account
    };

    let balance_to_move: u128 = 3;

    let mut env_builder = ExecutorEnv::builder();
    env_builder.write(&sender).unwrap();
    env_builder.write(&receiver).unwrap();
    env_builder.write(&balance_to_move).unwrap();
    let env = env_builder.build().unwrap();

    let executor = default_executor();
    let result: [Account; 4] = executor.execute(env, TRANSFER_ELF).unwrap().journal.decode().unwrap();
    let [_, _, sender_post, receiver_post] = result;

    println!("sender_before: {:?}, sender_after: {:?}", sender, sender_post);
    println!("receiver_before: {:?}, receiver_after: {:?}", receiver, receiver_post);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public() {
        run_public_execution_of_transfer_program();
    }
}