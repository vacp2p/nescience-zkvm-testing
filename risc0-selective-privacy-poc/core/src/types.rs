/// For this POC we consider 32-bit commitments
pub type Commitment = u32;
pub type Nullifier = [u32; 8];
pub type Address = [u32; 8];
pub type Nonce = [u32; 8];
pub type Key = [u32; 8];
pub type AuthenticationPath = [[u32; 8]; 32];
pub type ProgramId = [u32; 8];
