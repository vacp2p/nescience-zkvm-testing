use risc0_zkvm::{
    guest::env,
    serde::to_vec,
    sha::{Impl, Sha256},
};
use toy_example_core::{
    account::{compute_nullifier, hash, is_in_commitment_tree, Account},
    input::InputVisibiility,
};

/// Private execution logic.
/// Circuit for proving correct execution of some program with program id
/// equal to `program_id` (last input).
///
/// Currently only supports private execution of a program with two input accounts, one
/// of which must be a fresh new account (`account_2`) (for example a private transfer function).
///
/// This circuit checks:
/// - That accounts pre states and post states are consistent with the execution of the given `program_id`.
/// - That `account_2` is fresh (meaning, for this toy example, that it has 0 balance).
/// - That `program_id` execution didn't change addresses of the accounts.
///
/// Outputs:
/// - The nullifier for the only existing input account (account_1)
/// - The commitments for the private accounts post states.
fn main() {
    let num_inputs: u32 = env::read();
    // Read inputs and outputs
    let mut inputs_outputs = Vec::new();
    for _ in 0..(2 * num_inputs) {
        let account: Account = env::read();
        inputs_outputs.push(account);
    }

    // Read visibilities
    let mut input_visibilities = Vec::new();
    for _ in 0..num_inputs {
        let input_visibility: InputVisibiility = env::read();
        input_visibilities.push(input_visibility);
    }

    let commitment_tree_root: [u32; 8] = env::read();
    let program_id: [u32; 8] = env::read();

    let inputs = inputs_outputs.iter().take(num_inputs as usize);
    let mut nullifiers = Vec::new();
    for (visibility, input_account) in input_visibilities.iter().zip(inputs) {
        match visibility {
            InputVisibiility::Private(Some(private_key)) => {
                // Prove ownership of input accounts by proving
                // knowledge of the pre-image of their addresses.
                assert_eq!(hash(private_key), input_account.address);
                // Check the input account was created by a previous transaction
                // by checking it belongs to the commitments tree.
                let commitment = input_account.commitment();
                assert!(is_in_commitment_tree(commitment, commitment_tree_root));
                // Compute nullifier to nullify this private input account.
                let nullifier = compute_nullifier(&commitment, &private_key);
                nullifiers.push(nullifier);
            }
            InputVisibiility::Private(None) => {
                // Private accounts without a companion private key are
                // enforced to have default values
                assert_eq!(input_account.balance, 0);
            }
            // No checks on public accounts
            InputVisibiility::Public => continue,
        }
    }
    let outputs = inputs_outputs.iter().skip(num_inputs as usize);
    let output_commitments: Vec<_> = outputs.map(|account| account.commitment()).collect();

    // Assert `program_id` program didn't modify address fields
    for (account_pre, account_post) in inputs_outputs
        .iter()
        .take(num_inputs as usize)
        .zip(inputs_outputs.iter().skip(num_inputs as usize))
    {
        assert_eq!(account_pre.address, account_post.address);
    }

    // Verify pre states and post states of accounts are consistent
    // with the execution of the `program_id`` program
    env::verify(program_id, &to_vec(&inputs_outputs).unwrap()).unwrap();

    // Output nullifier of consumed input accounts and commitments of new output private accounts
    env::commit(&(nullifiers, output_commitments));
}
