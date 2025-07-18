use core::{
    account::Account,
    types::{Commitment, Nullifier},
};

use risc0_zkvm::Receipt;

use super::MockedSequencer;

impl MockedSequencer {
    pub fn invoke_privacy_execution(
        &mut self,
        receipt: Receipt,
        public_inputs_outputs: &[Account],
        nullifiers: &[Nullifier],
        commitments: &[Commitment],
    ) -> Result<(), ()> {
        let commitments_tree_root = self.get_commitment_tree_root();
        if public_inputs_outputs.len() % 2 != 0 {
            return Err(());
        }

        let num_input_public = public_inputs_outputs.len() >> 1;
        for account in public_inputs_outputs.iter().take(num_input_public) {
            let current_account = self.get_account(&account.address).ok_or(())?;
            if &current_account != account {
                return Err(());
            }
        }

        // Check that nullifiers have not been added before
        if nullifiers
            .iter()
            .any(|nullifier| self.nullifier_set.contains(nullifier))
        {
            return Err(());
        }

        // Check that commitments are new too
        if commitments
            .iter()
            .any(|commitment| self.commitment_tree.values().contains(commitment))
        {
            return Err(());
        }

        // Verify consistency between public accounts, nullifiers and commitments
        nssa::verify_privacy_execution(
            receipt,
            public_inputs_outputs,
            nullifiers,
            commitments,
            &commitments_tree_root,
        )?;

        // Update accounts
        public_inputs_outputs
            .iter()
            .cloned()
            .skip(num_input_public)
            .for_each(|account_post_state| {
                self.accounts
                    .insert(account_post_state.address, account_post_state);
            });

        // Add nullifiers
        self.nullifier_set.extend(nullifiers);

        // Add commitments
        for commitment in commitments.iter() {
            self.commitment_tree.add_value(*commitment);
        }

        Ok(())
    }
}
