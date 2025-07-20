use core::{
    compute_nullifier, hash, is_in_tree,
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
    assert_eq!(inner_program_output.accounts_post.len(), num_inputs);

    // Read visibilities
    let account_visibilities: Vec<AccountVisibility> = env::read();
    assert_eq!(account_visibilities.len(), num_inputs);

    // Read nonces for outputs
    let output_nonces: Vec<Nonce> = env::read();
    assert_eq!(output_nonces.len(), num_inputs);

    // Read root and program id.
    let commitment_tree_root: [u32; 8] = env::read();
    let program_id: ProgramId = env::read();

    // Verify pre states and post states of accounts are consistent
    // with the execution of the `program_id` program
    env::verify(program_id, &to_vec(&inner_program_output).unwrap()).unwrap();

    let inputs = inner_program_output.accounts_pre;
    let mut outputs = inner_program_output.accounts_post;
    let mut nullifiers = Vec::new();
    for (visibility, input_account) in account_visibilities.iter().zip(inputs.iter()) {
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

    // Assert that the inner program didn't modify address fields or nonces
    for (account_pre, account_post) in inputs.iter().zip(outputs.iter()) {
        assert_eq!(account_pre.address, account_post.address);
        assert_eq!(account_pre.nonce, account_post.nonce);
    }

    // Check that the program preserved the total supply
    let total_balance_pre: u128 = inputs.iter().map(|account| account.balance).sum();
    let total_balance_post: u128 = outputs.iter().map(|account| account.balance).sum();
    assert_eq!(total_balance_pre, total_balance_post);

    // From this point on the execution is considered valid
    //
    // Insert new nonces in outputs (including public ones)
    outputs
        .iter_mut()
        .zip(output_nonces)
        .for_each(|(account, new_nonce)| account.nonce = new_nonce);

    // Compute commitments for every private output
    let mut private_outputs = Vec::new();
    for (output, visibility) in outputs.iter().zip(account_visibilities.iter()) {
        match visibility {
            AccountVisibility::Public => continue,
            AccountVisibility::Private(_) => private_outputs.push(output),
        }
    }
    let private_output_commitments: Vec<_> = private_outputs.iter().map(|account| account.commitment()).collect();

    // Get the list of public accounts pre and post states
    let mut public_accounts_pre = Vec::new();
    let mut public_accounts_post = Vec::new();
    for ((account_pre, account_post), visibility) in
        inputs.into_iter().zip(outputs.into_iter()).zip(account_visibilities)
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

    env::commit(&output);
}
