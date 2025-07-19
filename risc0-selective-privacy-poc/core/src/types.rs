use serde::{Deserialize, Serialize};

use crate::account::Account;

/// For this POC we consider 32-bit commitments
pub type Commitment = u32;
pub type Nullifier = [u32; 8];
pub type Address = [u32; 8];
pub type Nonce = [u32; 8];
pub type Key = [u32; 8];
pub type AuthenticationPath = [[u32; 8]; 32];
pub type ProgramId = [u32; 8];

#[derive(Serialize, Deserialize)]
pub struct ProgramOutput {
    pub accounts_pre: Vec<Account>,
    pub accounts_post: Vec<Account>,
}

#[derive(Serialize, Deserialize)]
pub struct PrivacyExecutionOutput {
    pub public_accounts_pre: Vec<Account>,
    pub public_accounts_post: Vec<Account>,
    pub private_output_commitments: Vec<Commitment>,
    pub nullifiers: Vec<Nullifier>,
    pub commitment_tree_root: [u32; 8],
}
