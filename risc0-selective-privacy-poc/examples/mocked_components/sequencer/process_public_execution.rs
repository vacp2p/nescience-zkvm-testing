use core::{account::Account, inputs_outputs_preserve_invariants, types::Address};

use crate::mocked_components::sequencer::error::Error;

use super::MockedSequencer;

impl MockedSequencer {
    /// Processes a public execution request of the program `P`.
    pub fn process_public_execution<P: nssa::Program>(
        &mut self,
        input_account_addresses: &[Address],
        instruction_data: P::InstructionData,
    ) -> Result<(), Error> {
        // Fetch the current state of the input accounts.
        let input_accounts: Vec<Account> = input_account_addresses
            .iter()
            .map(|address| self.get_account(address).ok_or(Error::NotFound))
            .collect::<Result<_, _>>()?;

        // Execute the program
        let program_output =
            nssa::execute_onchain::<P>(&input_accounts, instruction_data).map_err(|_| Error::BadInput)?;

        // Assert accounts pre- and post-states preserve chains invariants
        if !inputs_outputs_preserve_invariants(&input_accounts, &program_output.accounts_post) {
            return Err(Error::BadInput);
        }

        // Fail if any of the output accounts is not yet registered.
        // This is redundant with previous checks, but better make it explicit.
        if !program_output
            .accounts_post
            .iter()
            .all(|account| self.accounts.contains_key(&account.address))
        {
            return Err(Error::BadInput);
        }

        // Update the accounts states
        program_output.accounts_post.into_iter().for_each(|account_post_state| {
            self.accounts.insert(account_post_state.address, account_post_state);
        });
        Ok(())
    }
}
