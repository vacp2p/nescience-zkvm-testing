use core::{
    account::Account,
    input::InputVisibiility,
    types::{Commitment, Nonce, Nullifier},
};
use program::Program;
use rand::{rngs::OsRng, Rng};
use risc0_zkvm::{default_executor, default_prover, ExecutorEnv, ExecutorEnvBuilder, Receipt};
use program_methods::{OUTER_ELF, OUTER_ID};

pub mod program;

pub fn new_random_nonce() -> Nonce {
    let mut rng = OsRng;
    std::array::from_fn(|_| rng.gen())
}

fn write_inputs<P: Program>(
    input_accounts: &[Account],
    instruction_data: &P::InstructionData,
    env_builder: &mut ExecutorEnvBuilder,
) -> Result<(), ()> {
    let input_accounts = input_accounts.to_vec();
    env_builder.write(&input_accounts).map_err(|_| ())?;
    env_builder.write(&instruction_data).map_err(|_| ())?;
    Ok(())
}

fn execute_and_prove_inner<P: Program>(
    input_accounts: &[Account],
    instruction_data: &P::InstructionData,
) -> Result<(Receipt, Vec<Account>), ()> {
    // Write inputs to the program
    let mut env_builder = ExecutorEnv::builder();
    write_inputs::<P>(input_accounts, instruction_data, &mut env_builder)?;
    let env = env_builder.build().unwrap();

    // Prove the program
    let prover = default_prover();
    let prove_info = prover.prove(env, P::PROGRAM_ELF).map_err(|_| ())?;
    let receipt = prove_info.receipt;

    // Get proof and (inputs and) outputs
    let inputs_outputs: Vec<Account> = receipt.journal.decode().map_err(|_| ())?;

    Ok((receipt, inputs_outputs))
}

pub fn execute<P: Program>(
    input_accounts: &[Account],
    instruction_data: &P::InstructionData,
) -> Result<Vec<Account>, ()> {
    // Write inputs to the program
    let mut env_builder = ExecutorEnv::builder();
    write_inputs::<P>(input_accounts, instruction_data, &mut env_builder)?;
    let env = env_builder.build().unwrap();

    // Execute the program (without proving)
    let executor = default_executor();
    let session_info = executor.execute(env, P::PROGRAM_ELF).map_err(|_| ())?;

    // Get proof and (inputs and) outputs
    let inputs_outputs: Vec<Account> = session_info.journal.decode().map_err(|_| ())?;

    Ok(inputs_outputs)
}

pub fn execute_and_prove_privacy_execution<P: Program>(
    inputs: &[Account],
    instruction_data: &P::InstructionData,
    visibilities: &[InputVisibiility],
    commitment_tree_root: [u32; 8],
) -> Result<Receipt, ()> {
    // Prove inner program and get post state of the accounts
    let num_inputs = inputs.len();
    let (inner_receipt, inputs_outputs) = execute_and_prove_inner::<P>(inputs, instruction_data)?;

    // Sample fresh random nonces for the outputs of this execution
    let output_nonces: Vec<_> = (0..num_inputs).map(|_| new_random_nonce()).collect();

    // Prove outer program.
    // This computes the nullifiers for the input accounts and commitments for the output accounts.
    let mut env_builder = ExecutorEnv::builder();
    env_builder.add_assumption(inner_receipt);
    env_builder.write(&(num_inputs as u32)).unwrap();
    env_builder.write(&inputs_outputs).unwrap();
    env_builder.write(&visibilities).unwrap();
    env_builder.write(&output_nonces).unwrap();
    env_builder.write(&commitment_tree_root).unwrap();
    env_builder.write(&P::PROGRAM_ID).unwrap();
    let env = env_builder.build().unwrap();

    let prover = default_prover();
    let prove_info = prover.prove(env, OUTER_ELF).unwrap();
    Ok(prove_info.receipt)
}

pub fn verify_privacy_execution(
    receipt: Receipt,
    public_accounts_inputs_outputs: &[Account],
    nullifiers: &[Nullifier],
    private_output_commitments: &[Commitment],
    commitment_tree_root: &[u32; 8],
) -> Result<(), ()> {
    let output: (Vec<Account>, Vec<Nullifier>, Vec<Commitment>, [u32; 8]) =
        receipt.journal.decode().unwrap();
    let expected_output = (
        public_accounts_inputs_outputs.to_vec(),
        nullifiers.to_vec(),
        private_output_commitments.to_vec(),
        commitment_tree_root.to_owned(),
    );
    if output != expected_output {
        return Err(());
    } else {
        receipt.verify(OUTER_ID).map_err(|_| ())
    }
}
