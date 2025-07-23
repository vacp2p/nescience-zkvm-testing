use core::types::ProgramId;

use program_methods::{PINATA_ELF, PINATA_ID, TRANSFER_ELF, TRANSFER_ID, TRANSFER_MULTIPLE_ELF, TRANSFER_MULTIPLE_ID};
use serde::{Deserialize, Serialize};

/// A trait to be implemented by inner programs.
pub trait Program {
    const PROGRAM_ID: ProgramId;
    const PROGRAM_ELF: &[u8];
    type InstructionData: Serialize + for<'de> Deserialize<'de>;
}

pub struct TransferProgram;
impl Program for TransferProgram {
    const PROGRAM_ID: ProgramId = TRANSFER_ID;
    const PROGRAM_ELF: &[u8] = TRANSFER_ELF;
    /// Amount to transfer
    type InstructionData = u128;
}

pub struct TransferMultipleProgram;
impl Program for TransferMultipleProgram {
    const PROGRAM_ID: ProgramId = TRANSFER_MULTIPLE_ID;
    const PROGRAM_ELF: &[u8] = TRANSFER_MULTIPLE_ELF;
    /// Amounts to transfer
    type InstructionData = Vec<u128>;
}

pub struct PinataProgram;
impl Program for PinataProgram {
    const PROGRAM_ID: ProgramId = PINATA_ID;
    const PROGRAM_ELF: &[u8] = PINATA_ELF;
    /// Preimage of target hash to win prize
    type InstructionData = Vec<u32>;
}
