use core::{
    account::Account,
    compute_nullifier, hash, is_in_tree,
    types::{Nonce, ProgramId},
    visibility::InputVisibiility,
};
use risc0_zkvm::{guest::env, serde::to_vec};

/// Privacy execution logic.
/// This is the circuit for proving correct off-chain executions of programs.
/// It also verifies that the chain's invariants are not violated.
///
/// Inputs:
/// - Vec<Account>: The output of the inner program. This is assumed to include the accounts pre and
///     post-states of the execution of the inner program.
///
/// - Vec<InputVisibility>: A vector indicating which accounts are private and which are public.
///
/// - Vec<Nonce>: The vector of nonces to be used for the output accounts. This is assumed to be
///     sampled at random by the host program.
///
/// - [u32; 8]: The root of the commitment tree. Commitments of used private accounts will be
///     checked against this to prove that they belong to the tree.
/// - ProgamId: The ID of the inner program.
///
/// Public outputs:
/// - The vector of accounts' pre and post states for the public accounts.
/// - The nullifiers of the used private accounts.
/// - The commitments for the ouput private accounts.
/// - The commitment tree root used for the authentication path verifications.
fn main() {
    let num_inputs: u32 = env::read();
    // Read inputs and outputs
    let mut inputs_outputs: Vec<Account> = env::read();
    assert_eq!(inputs_outputs.len() as u32, num_inputs * 2);

    // Read visibilities
    let input_visibilities: Vec<InputVisibiility> = env::read();
    assert_eq!(input_visibilities.len() as u32, num_inputs);

    // Read nonces for outputs
    let output_nonces: Vec<Nonce> = env::read();
    assert_eq!(output_nonces.len() as u32, num_inputs);

    // Read root and program id.
    let commitment_tree_root: [u32; 8] = env::read();
    let program_id: ProgramId = env::read();

    // Verify pre states and post states of accounts are consistent
    // with the execution of the `program_id` program
    env::verify(program_id, &to_vec(&inputs_outputs).unwrap()).unwrap();

    // Split inputs_outputs into two separate vectors
    let (inputs, mut outputs) = {
        let outputs = inputs_outputs.split_off(num_inputs as usize);
        (inputs_outputs, outputs)
    };

    let mut nullifiers = Vec::new();
    for (visibility, input_account) in input_visibilities.iter().zip(inputs.iter()) {
        match visibility {
            InputVisibiility::Private(Some((private_key, auth_path))) => {
                // Prove ownership of input accounts by proving knowledge of the pre-image of their addresses.
                assert_eq!(hash(private_key), input_account.address);
                // Check the input account was created by a previous transaction by checking it belongs to the commitments tree.
                let commitment = input_account.commitment();
                assert!(is_in_tree(commitment, auth_path, commitment_tree_root));
                // Compute the nullifier to nullify this private input account.
                let nullifier = compute_nullifier(&commitment, private_key);
                nullifiers.push(nullifier);
            }
            InputVisibiility::Private(None) => {
                // Private accounts without a companion private key are enforced to have default values
                // Used for executions that need to create a new private account.
                assert_eq!(input_account.balance, 0);
                assert_eq!(input_account.nonce, [0; 8]);
            }
            // No checks on public accounts
            InputVisibiility::Public => continue,
        }
    }

    // Assert `program_id` program didn't modify address fields or nonces
    for (account_pre, account_post) in inputs.iter().zip(outputs.iter()) {
        assert_eq!(account_pre.address, account_post.address);
        assert_eq!(account_pre.nonce, account_post.nonce);
    }

    // Check that the program preserved the total supply
    let total_balance_pre: u128 = inputs.iter().map(|account| account.balance).sum();
    let total_balance_post: u128 = outputs.iter().map(|account| account.balance).sum();
    assert_eq!(total_balance_pre, total_balance_post);

    // Insert new nonces in outputs (including public ones (?!))
    outputs
        .iter_mut()
        .zip(output_nonces)
        .for_each(|(account, new_nonce)| account.nonce = new_nonce);

    // Compute private outputs commitments
    let mut private_outputs = Vec::new();
    for (output, visibility) in outputs.iter().zip(input_visibilities.iter()) {
        match visibility {
            InputVisibiility::Public => continue,
            InputVisibiility::Private(_) => private_outputs.push(output),
        }
    }

    // Get the list of public inputs pre states and their post states
    let mut public_inputs_outputs = Vec::new();
    for (account, visibility) in inputs
        .iter()
        .chain(outputs.iter())
        .zip(input_visibilities.iter().chain(input_visibilities.iter()))
    {
        match visibility {
            InputVisibiility::Public => {
                public_inputs_outputs.push(account);
            }
            InputVisibiility::Private(_) => continue,
        }
    }

    // Compute commitments for every private output
    let private_output_commitments: Vec<_> = private_outputs.iter().map(|account| account.commitment()).collect();

    // Output nullifier of consumed input accounts and commitments of new output private accounts
    env::commit(&(
        public_inputs_outputs,
        nullifiers,
        private_output_commitments,
        commitment_tree_root,
    ));
}
