use risc0_zkvm::{default_executor, ExecutorEnv};
use toy_example_core::Account;
use transfer_methods::TRANSFER_ELF;

pub fn run_public_execution_of_transfer_program() {
    let sender_private_key = [0; 8];
    let mut sender = Account::new_from_private_key(sender_private_key, [1; 8]);
    sender.balance = 150;

    let receiver_private_key = [99; 8];
    let mut receiver = Account::new_from_private_key(receiver_private_key, [1; 8]);
    receiver.balance = 900;

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