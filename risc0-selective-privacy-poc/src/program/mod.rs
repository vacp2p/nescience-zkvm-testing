use serde::{Deserialize, Serialize};
use transfer_methods::{PINATA_ELF, PINATA_ID, TRANSFER_ELF, TRANSFER_ID};
use transfer_multiple_methods::{TRANSFER_MULTIPLE_ELF, TRANSFER_MULTIPLE_ID};

pub trait Program {
    const PROGRAM_ID: [u32; 8];
    const PROGRAM_ELF: &[u8];
    type InstructionData: Serialize + for<'de> Deserialize<'de>;
}

pub struct TransferProgram;
impl Program for TransferProgram {
    const PROGRAM_ID: [u32; 8] = TRANSFER_ID;
    const PROGRAM_ELF: &[u8] = TRANSFER_ELF;
    /// Amount to transfer
    type InstructionData = u128;
}

pub struct TransferMultipleProgram;
impl Program for TransferMultipleProgram {
    const PROGRAM_ID: [u32; 8] = TRANSFER_MULTIPLE_ID;
    const PROGRAM_ELF: &[u8] = TRANSFER_MULTIPLE_ELF;
    /// Amounts to transfer
    type InstructionData = Vec<u128>;
}

pub struct PinataProgram;
impl Program for PinataProgram {
    const PROGRAM_ID: [u32; 8] = TRANSFER_MULTIPLE_ID;
    const PROGRAM_ELF: &[u8] = TRANSFER_MULTIPLE_ELF;
    /// Preimage of target hash to win price
    type InstructionData = [u32; 8];
}
