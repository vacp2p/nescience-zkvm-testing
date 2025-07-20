use core::{
    account::Account,
    types::{Nonce, ProgramOutput},
    visibility::AccountVisibility,
};
use program_methods::{OUTER_ELF, OUTER_ID};
use rand::{rngs::OsRng, Rng};
use risc0_zkvm::{default_executor, default_prover, ExecutorEnv, ExecutorEnvBuilder, Receipt};

pub mod error;
pub mod program;
pub use error::Error;

pub use program::Program;

pub fn new_random_nonce() -> Nonce {
    let mut rng = OsRng;
    std::array::from_fn(|_| rng.gen())
}

/// Writes inputs to `env_builder` in the order expected by the programs
fn write_inputs<P: Program>(
    input_accounts: &[Account],
    instruction_data: P::InstructionData,
    env_builder: &mut ExecutorEnvBuilder,
) -> Result<(), Error> {
    let input_accounts = input_accounts.to_vec();
    env_builder.write(&input_accounts).map_err(|_| Error::BadInput)?;
    env_builder.write(&instruction_data).map_err(|_| Error::BadInput)?;
    Ok(())
}

/// Executes and proves the program `P`.
/// Returns the proof and the list of accounts pre and post states
fn execute_and_prove_inner<P: Program>(
    input_accounts: &[Account],
    instruction_data: P::InstructionData,
) -> Result<Receipt, Error> {
    // Write inputs to the program
    let mut env_builder = ExecutorEnv::builder();
    write_inputs::<P>(input_accounts, instruction_data, &mut env_builder)?;
    let env = env_builder.build().unwrap();

    // Prove the program
    let prover = default_prover();
    let prove_info = prover
        .prove(env, P::PROGRAM_ELF)
        .map_err(|e| Error::Risc0(e.to_string()))?;
    Ok(prove_info.receipt)
}

/// Builds the private outputs from the results of the execution of an inner program.
/// Filters private outputs and populates the nonces with the ones provided.
fn build_private_outputs_from_inner_results(
    inner_program_output: &ProgramOutput,
    visibilities: &[AccountVisibility],
    nonces: &[Nonce],
) -> Vec<Account> {
    inner_program_output
        .accounts_post
        .iter()
        .zip(visibilities)
        .zip(nonces)
        .filter(|((_, visibility), _)| matches!(visibility, AccountVisibility::Private(_)))
        .map(|((account, _), nonce)| {
            let mut this = account.clone();
            this.nonce = *nonce;
            this
        })
        .collect()
}

/// Executes the program `P` without generating a proof.
/// Returns the list of accounts pre and post states.
pub fn execute_onchain<P: Program>(
    input_accounts: &[Account],
    instruction_data: P::InstructionData,
) -> Result<ProgramOutput, Error> {
    // Write inputs to the program
    let mut env_builder = ExecutorEnv::builder();
    write_inputs::<P>(input_accounts, instruction_data, &mut env_builder)?;
    let env = env_builder.build().unwrap();

    // Execute the program (without proving)
    let executor = default_executor();
    let session_info = executor
        .execute(env, P::PROGRAM_ELF)
        .map_err(|e| Error::Risc0(e.to_string()))?;

    // Get (inputs and) outputs
    session_info.journal.decode().map_err(|e| Error::Risc0(e.to_string()))
}

/// Executes and proves the inner program `P` and executes and proves the outer program on top of it.
/// Returns the proof of execution of the outer program and the list of new private accounts
/// resulted from this execution.
pub fn execute_offchain<P: Program>(
    inputs: &[Account],
    instruction_data: P::InstructionData,
    visibilities: &[AccountVisibility],
    commitment_tree_root: [u32; 8],
) -> Result<(Receipt, Vec<Account>), Error> {
    // Prove inner program and get post state of the accounts
    let inner_receipt = execute_and_prove_inner::<P>(inputs, instruction_data)?;
    let inner_program_output: ProgramOutput = inner_receipt
        .journal
        .decode()
        .map_err(|e| Error::Risc0(e.to_string()))?;

    // Sample fresh random nonces for the outputs of this execution
    let output_nonces: Vec<_> = (0..inputs.len()).map(|_| new_random_nonce()).collect();

    // Prove outer program.
    let mut env_builder = ExecutorEnv::builder();
    env_builder.add_assumption(inner_receipt);
    env_builder.write(&inner_program_output).unwrap();
    env_builder.write(&visibilities).unwrap();
    env_builder.write(&output_nonces).unwrap();
    env_builder.write(&commitment_tree_root).unwrap();
    env_builder.write(&P::PROGRAM_ID).unwrap();
    let env = env_builder.build().unwrap();
    let prover = default_prover();
    let prove_info = prover.prove(env, OUTER_ELF).unwrap();

    // Build private accounts.
    let private_outputs = build_private_outputs_from_inner_results(&inner_program_output, visibilities, &output_nonces);

    Ok((prove_info.receipt, private_outputs))
}

/// Verifies a proof of the outer program for the given parameters.
pub fn verify_privacy_execution(receipt: Receipt) -> Result<(), Error> {
    receipt.verify(OUTER_ID).map_err(|e| Error::Risc0(e.to_string()))
}
