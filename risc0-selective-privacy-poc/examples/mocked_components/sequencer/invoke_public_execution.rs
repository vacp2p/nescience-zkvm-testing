use core::{account::Account, types::Address};

use super::MockedSequencer;

impl MockedSequencer {
    pub fn invoke_public_execution<P: nssa::Program>(
        &mut self,
        input_account_addresses: &[Address],
        instruction_data: P::InstructionData,
    ) -> Result<(), ()> {
        // Fetch accounts
        let input_accounts: Vec<Account> = input_account_addresses
            .iter()
            .map(|address| self.get_account(address).ok_or(()))
            .collect::<Result<_, _>>()?;

        // Execute
        let inputs_outputs = nssa::execute::<P>(&input_accounts, instruction_data)?;

        // Perform consistency checks
        if !self.inputs_outputs_are_consistent(&input_accounts, &inputs_outputs) {
            return Err(());
        }

        // Update accounts
        inputs_outputs
            .into_iter()
            .skip(input_accounts.len())
            .for_each(|account_post_state| {
                self.accounts
                    .insert(account_post_state.address, account_post_state);
            });
        Ok(())
    }

    fn inputs_outputs_are_consistent(
        &self,
        input_accounts: &[Account],
        inputs_outputs: &[Account],
    ) -> bool {
        let num_inputs = input_accounts.len();
        if inputs_outputs.len() != num_inputs * 2 {
            return false;
        }

        let (claimed_accounts_pre, accounts_post) = inputs_outputs.split_at(num_inputs);
        if claimed_accounts_pre != input_accounts {
            return false;
        }

        for (account_pre, account_post) in input_accounts.iter().zip(accounts_post) {
            if account_pre.address != account_post.address {
                return false;
            }
            if account_pre.nonce != account_post.nonce {
                return false;
            }
            // Redundant with previous checks, but better make it explicit.
            if !self.accounts.contains_key(&account_post.address) {
                return false;
            }
        }
        let accounts_pre_total_balance: u128 =
            input_accounts.iter().map(|account| account.balance).sum();
        let accounts_post_total_balance: u128 =
            accounts_post.iter().map(|account| account.balance).sum();
        if accounts_pre_total_balance != accounts_post_total_balance {
            return false;
        }
        return true;
    }
}
