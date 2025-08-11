#[cfg(not(rust_analyzer))]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/methods.rs"));
}
#[cfg(not(rust_analyzer))]
pub use generated::{GUEST_ELF, GUEST_ID};


#[cfg(rust_analyzer)]
pub const GUEST_ELF: &[u8] = &[];
#[cfg(rust_analyzer)]
pub const GUEST_ID: [u32; 8] = [0; 8];

use anyhow::Result;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};


pub fn prove_encrypt(
    key: [u8; 32],
    nonce: [u8; 12],
    plaintext: &[u8],
) -> Result<(Receipt, Vec<u8>)> {
    let env = ExecutorEnv::builder()
        .write(&key)?
        .write(&nonce)?
        .write(&plaintext.to_vec())?
        .build()?;

    let prover = default_prover();
    let prove_info = prover.prove(env, GUEST_ELF)?;
    let receipt = prove_info.receipt;
    receipt.verify(GUEST_ID)?;

    let ciphertext: Vec<u8> = receipt.journal.bytes.clone(); // if this errors, use .to_vec()
    
    Ok((receipt, ciphertext))
}
