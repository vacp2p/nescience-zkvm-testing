use core::{
    account::Account,
    input::InputVisibiility,
    types::{Address, Commitment, Key, Nullifier},
};

use nssa::program::TransferProgram;

use crate::mocked_components::sequencer::MockedSequencer;

pub mod transfer_deshielded;
pub mod transfer_private;
pub mod transfer_public;
pub mod transfer_shielded;

pub struct MockedClient;

impl MockedClient {
    pub fn fresh_account_for_mint(address: Address) -> Account {
        let nonce = [0; 8];
        Account::new(address, nonce)
    }
}
