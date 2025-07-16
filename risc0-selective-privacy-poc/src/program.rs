use risc0_zkvm::{default_executor, default_prover, ExecutorEnv, ExecutorEnvBuilder, Receipt};
use serde::{Deserialize, Serialize};
use toy_example_core::account::Account;

pub(crate) trait Program {
    const PROGRAM_ID: [u32; 8];
    const PROGRAM_ELF: &[u8];
    type InstructionData: Serialize + for<'de> Deserialize<'de>;

    fn write_inputs(
        input_accounts: &[Account],
        instruction_data: &Self::InstructionData,
        env_builder: &mut ExecutorEnvBuilder,
    ) -> Result<(), ()> {
        for account in input_accounts {
            env_builder.write(&account).map_err(|_| ())?;
        }
        env_builder.write(&instruction_data).map_err(|_| ())?;
        Ok(())
    }

    fn execute_and_prove(
        input_accounts: &[Account],
        instruction_data: &Self::InstructionData,
    ) -> Result<(Receipt, Vec<Account>), ()> {
        // Write inputs to the program
        let mut env_builder = ExecutorEnv::builder();
        Self::write_inputs(input_accounts, instruction_data, &mut env_builder)?;
        let env = env_builder.build().unwrap();

        // Prove the program
        let prover = default_prover();
        let prove_info = prover.prove(env, Self::PROGRAM_ELF).map_err(|_| ())?;
        let receipt = prove_info.receipt;

        // Get proof and (inputs and) outputs
        let inputs_outputs: Vec<Account> = receipt.journal.decode().map_err(|_| ())?;

        Ok((receipt, inputs_outputs))
    }

    fn execute(
        input_accounts: &[Account],
        instruction_data: &Self::InstructionData,
    ) -> Result<Vec<Account>, ()> {
        // Write inputs to the program
        let mut env_builder = ExecutorEnv::builder();
        Self::write_inputs(input_accounts, instruction_data, &mut env_builder)?;
        let env = env_builder.build().unwrap();

        // Execute the program (without proving)
        let executor = default_executor();
        let session_info = executor.execute(env, Self::PROGRAM_ELF).map_err(|_| ())?;

        // Get proof and (inputs and) outputs
        let inputs_outputs: Vec<Account> = session_info.journal.decode().map_err(|_| ())?;

        Ok(inputs_outputs)
    }
}
