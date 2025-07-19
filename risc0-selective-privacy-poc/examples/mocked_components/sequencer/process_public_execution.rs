use core::{
    account::Account,
    types::{Address, ProgramOutput},
};

use super::MockedSequencer;

impl MockedSequencer {
    /// Processes a public execution request of the program `P`.
    pub fn process_public_execution<P: nssa::Program>(
        &mut self,
        input_account_addresses: &[Address],
        instruction_data: P::InstructionData,
    ) -> Result<(), ()> {
        // Fetch the current state of the input accounts.
        let input_accounts: Vec<Account> = input_account_addresses
            .iter()
            .map(|address| self.get_account(address).ok_or(()))
            .collect::<Result<_, _>>()?;

        // Execute the program
        let program_output = nssa::execute_onchain::<P>(&input_accounts, instruction_data)?;

        // Perform consistency checks
        if !self.program_output_is_valid(&input_accounts, &program_output.accounts_post) {
            return Err(());
        }

        // Update the accounts states
        program_output.accounts_post.into_iter().for_each(|account_post_state| {
            self.accounts.insert(account_post_state.address, account_post_state);
        });
        Ok(())
    }

    /// Verifies that a program public execution didn't break the chain's rules.
    /// `input_accounts` are the accounts provided as inputs to the program.
    /// `output_accounts` are the accounts post states after execution of the program
    fn program_output_is_valid(&self, input_accounts: &[Account], output_accounts: &[Account]) -> bool {
        // Fail if the number of input and output accounts differ
        if input_accounts.len() != output_accounts.len() {
            return false;
        }

        for (account_pre, account_post) in input_accounts.iter().zip(output_accounts) {
            // Fail if the program modified the addresses of the input accounts
            if account_pre.address != account_post.address {
                return false;
            }
            // Fail if the program modified the nonces of the input accounts
            if account_pre.nonce != account_post.nonce {
                return false;
            }
            // Fail if any of the output accounts is not yet registered.
            // (redundant with previous checks, but better make it explicit)
            if !self.accounts.contains_key(&account_post.address) {
                return false;
            }
        }

        // Fail if the execution didn't preserve the total supply.
        let total_balance_pre: u128 = input_accounts.iter().map(|account| account.balance).sum();
        let total_balance_post: u128 = output_accounts.iter().map(|account| account.balance).sum();
        if total_balance_pre != total_balance_post {
            return false;
        }

        true
    }
}
