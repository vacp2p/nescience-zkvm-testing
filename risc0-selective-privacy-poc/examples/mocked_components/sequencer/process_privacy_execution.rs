use core::types::PrivacyExecutionOutput;

use risc0_zkvm::Receipt;

use super::error::Error;
use super::MockedSequencer;

impl MockedSequencer {
    /// Processes a privacy execution request.
    /// Verifies the proof of the privacy execution and updates the state of the chain.
    pub fn process_privacy_execution(&mut self, receipt: Receipt) -> Result<(), Error> {
        // Parse the output of the "outer" program
        let output: PrivacyExecutionOutput = receipt.journal.decode().unwrap();

        // Reject in case the root used in the privacy execution is not the current root.
        if output.commitment_tree_root != self.get_commitment_tree_root() {
            return Err(Error::BadInput);
        }

        // Reject in case the number of accounts pre states is different from the post states
        if output.public_accounts_pre.len() != output.public_accounts_post.len() {
            return Err(Error::BadInput);
        }

        // Reject if the states of the public input accounts used in the inner execution do not
        // coincide with the on-chain state.
        for account in output.public_accounts_pre.iter() {
            let current_account = self.get_account(&account.address).ok_or(Error::NotFound)?;
            if &current_account != account {
                return Err(Error::BadInput);
            }
        }

        // Reject if the nullifiers of this privacy execution have already been published.
        if output
            .nullifiers
            .iter()
            .any(|nullifier| self.nullifier_set.contains(nullifier))
        {
            return Err(Error::BadInput);
        }

        // Reject if the commitments have already been seen.
        if output
            .private_output_commitments
            .iter()
            .any(|commitment| self.commitment_tree.values().contains(commitment))
        {
            return Err(Error::BadInput);
        }

        // Verify the proof of the privacy execution.
        // This includes a proof of the following statements
        // - Public inputs, public outputs, commitments and nullifiers are consistent with the
        //   execution of some program.
        // - The given nullifiers correctly correspond to commitments that currently belong to
        //   the commitment tree.
        // - The given commitments are correctly computed from valid accounts.
        nssa::verify_privacy_execution(receipt).map_err(|_| Error::BadInput)?;

        // At this point the privacy execution is considered valid.
        //
        // Update the state of the public accounts with the post-state of this privacy execution
        output.public_accounts_post.into_iter().for_each(|account_post_state| {
            self.accounts.insert(account_post_state.address, account_post_state);
        });

        // Add all nullifiers to the nullifier set.
        self.nullifier_set.extend(output.nullifiers);

        // Add commitments to the commitment tree.
        for commitment in output.private_output_commitments.iter() {
            self.commitment_tree.add_value(*commitment);
        }

        Ok(())
    }
}
