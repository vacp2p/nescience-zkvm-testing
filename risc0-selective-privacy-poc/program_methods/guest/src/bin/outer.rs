use core::{
    compute_nullifier, hash, inputs_outputs_preserve_invariants, is_in_tree,
    types::{Nonce, PrivacyExecutionOutput, ProgramId, ProgramOutput},
    visibility::AccountVisibility,
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
/// - Vec<AccountVisibility>: A vector indicating which accounts are private and which are public.
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
    // Read inner program output
    let inner_program_output: ProgramOutput = env::read();
    let num_inputs = inner_program_output.accounts_pre.len();

    // Read visibilities
    let visibilities: Vec<AccountVisibility> = env::read();
    assert_eq!(visibilities.len(), num_inputs);

    // Read nonces for outputs
    let output_nonces: Vec<Nonce> = env::read();
    assert_eq!(output_nonces.len(), num_inputs);

    // Read root and program id.
    let commitment_tree_root: [u32; 8] = env::read();
    let program_id: ProgramId = env::read();

    // Authentication step:
    let nullifiers = verify_and_nullify_private_inputs(&inner_program_output, &visibilities, commitment_tree_root);

    // Verify pre states and post states of accounts are consistent
    // with the execution of the `program_id` program
    env::verify(program_id, &to_vec(&inner_program_output).unwrap()).unwrap();

    // Assert accounts pre- and post-states preserve chains invariants
    assert!(inputs_outputs_preserve_invariants(
        &inner_program_output.accounts_pre,
        &inner_program_output.accounts_post
    ));

    // From this point on the execution is considered valid

    let output = assemble_privacy_execution_output(
        inner_program_output,
        visibilities,
        output_nonces,
        commitment_tree_root,
        nullifiers,
    );

    env::commit(&output);
}

fn assemble_privacy_execution_output(
    inner_program_output: ProgramOutput,
    visibilities: Vec<AccountVisibility>,
    output_nonces: Vec<[u32; 8]>,
    commitment_tree_root: [u32; 8],
    nullifiers: Vec<[u32; 8]>,
) -> PrivacyExecutionOutput {
    // Insert new nonces in outputs (including public ones)
    let accounts_pre = inner_program_output.accounts_pre;
    let mut accounts_post = inner_program_output.accounts_post;
    accounts_post
        .iter_mut()
        .zip(output_nonces)
        .for_each(|(account, new_nonce)| account.nonce = new_nonce);

    // Compute commitments for every private output
    let mut private_outputs = Vec::new();
    for (output, visibility) in accounts_post.iter().zip(visibilities.iter()) {
        match visibility {
            AccountVisibility::Public => continue,
            AccountVisibility::Private(_) => private_outputs.push(output),
        }
    }
    let private_output_commitments: Vec<_> = private_outputs.iter().map(|account| account.commitment()).collect();

    // Get the list of public accounts pre and post states
    let mut public_accounts_pre = Vec::new();
    let mut public_accounts_post = Vec::new();
    for ((account_pre, account_post), visibility) in accounts_pre
        .into_iter()
        .zip(accounts_post.into_iter())
        .zip(visibilities)
    {
        match visibility {
            AccountVisibility::Public => {
                public_accounts_pre.push(account_pre);
                public_accounts_post.push(account_post);
            }
            AccountVisibility::Private(_) => continue,
        }
    }

    let output = PrivacyExecutionOutput {
        public_accounts_pre,
        public_accounts_post,
        private_output_commitments,
        nullifiers,
        commitment_tree_root,
    };
    output
}

/// Compute nullifiers of private accounts pre states and check that their commitments belong to the commitments tree
fn verify_and_nullify_private_inputs(
    inner_program_output: &ProgramOutput,
    account_visibilities: &Vec<AccountVisibility>,
    commitment_tree_root: [u32; 8],
) -> Vec<[u32; 8]> {
    let mut nullifiers = Vec::new();
    for (visibility, input_account) in account_visibilities
        .iter()
        .zip(inner_program_output.accounts_pre.iter())
    {
        match visibility {
            AccountVisibility::Private(Some((private_key, auth_path))) => {
                // Prove ownership of input accounts by proving knowledge of the pre-image of their addresses.
                assert_eq!(hash(private_key), input_account.address);
                // Check the input account was created by a previous transaction by checking it belongs to the commitments tree.
                let commitment = input_account.commitment();
                assert!(is_in_tree(commitment, auth_path, commitment_tree_root));
                // Compute the nullifier to nullify this private input account.
                let nullifier = compute_nullifier(&commitment, private_key);
                nullifiers.push(nullifier);
            }
            AccountVisibility::Private(None) => {
                // Private accounts without a companion private key are enforced to have default values
                // Used for executions that need to create a new private account.
                assert_eq!(input_account.balance, 0);
                assert_eq!(input_account.nonce, [0; 8]);
            }
            // No checks on public accounts
            AccountVisibility::Public => continue,
        }
    }
    nullifiers
}
