mod private_execution;
mod program;
mod public_execution;

use program::Program;
use transfer_methods::{TRANSFER_ELF, TRANSFER_ID};

struct TransferProgram;

impl Program for TransferProgram {
    const PROGRAM_ID: [u32; 8] = TRANSFER_ID;

    const PROGRAM_ELF: &[u8] = TRANSFER_ELF;

    // Amount to transfer
    type InstructionData = u128;
}
