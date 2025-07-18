use core::{
    account::Account,
    types::{Commitment, Nullifier},
};

use risc0_zkvm::Receipt;

use super::MockedSequencer;

impl MockedSequencer {
    /// Processes a privacy execution request.
    /// Verifies the proof of the privacy execution and updates the state of the chain.
    pub fn process_privacy_execution(&mut self, receipt: Receipt) -> Result<(), ()> {
        // Parse the output of the proof.
        // This is the output of the "outer" program
        let output: (Vec<Account>, Vec<Nullifier>, Vec<Commitment>, [u32; 8]) = receipt.journal.decode().unwrap();
        let (public_inputs_outputs, nullifiers, commitments, commitment_tree_root) = output;

        // Reject in case the root used in the privacy execution is not the current root.
        if commitment_tree_root != self.get_commitment_tree_root() {
            return Err(());
        }

        // Reject in case the number of accounts in the public_inputs_outputs is not even.
        // This is because it is expected to contain the pre and post-states of public the accounts
        // of the inner execution.
        if public_inputs_outputs.len() % 2 != 0 {
            return Err(());
        }

        // Reject if the states of the public input accounts used in the inner execution do not
        // coincide with the on-chain state.
        let num_input_public = public_inputs_outputs.len() >> 1;
        for account in public_inputs_outputs.iter().take(num_input_public) {
            let current_account = self.get_account(&account.address).ok_or(())?;
            if &current_account != account {
                return Err(());
            }
        }

        // Reject if the nullifiers of this privacy execution have already been published.
        if nullifiers
            .iter()
            .any(|nullifier| self.nullifier_set.contains(nullifier))
        {
            return Err(());
        }

        // Reject if the commitments have already been seen.
        if commitments
            .iter()
            .any(|commitment| self.commitment_tree.values().contains(commitment))
        {
            return Err(());
        }

        // Verify the proof of the privacy execution.
        // This includes a proof of the following statements
        // - Public inputs, public outputs, commitments and nullifiers are consistent with the
        //   execution of some program.
        // - The given nullifiers correctly correspond to commitments that currently belong to
        //   the commitment tree.
        // - The given commitments are correctly computed from valid accounts.
        nssa::verify_privacy_execution(
            receipt,
            &public_inputs_outputs,
            &nullifiers,
            &commitments,
            &commitment_tree_root,
        )?;

        // At this point the privacy execution is considered valid.
        //
        // Update the state of the public accounts with the post-state of this privacy execution
        public_inputs_outputs
            .iter()
            .cloned()
            .skip(num_input_public)
            .for_each(|account_post_state| {
                self.accounts.insert(account_post_state.address, account_post_state);
            });

        // Add all nullifiers to the nullifier set.
        self.nullifier_set.extend(nullifiers);

        // Add commitments to the commitment tree.
        for commitment in commitments.iter() {
            self.commitment_tree.add_value(*commitment);
        }

        Ok(())
    }
}
