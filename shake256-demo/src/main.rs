#[cfg(not(rust_analyzer))]
include!(concat!(env!("OUT_DIR"), "/methods.rs"));

#[cfg(rust_analyzer)]
mod methods {
    pub const GUEST_ELF: &[u8] = &[];
    pub const GUEST_ID: [u32; 8] = [0; 8];
}
#[cfg(rust_analyzer)]
use methods::*;


use anyhow::Result; 
use hex::encode;
use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() -> Result<()> {
    // Example inputs
    let key       = [0x42u8; 32];
    let nonce     = [0x24u8; 12];
    let plaintext = b"Hello, RISC Zero ChaCha20 demo!";

    let env = ExecutorEnv::builder()
        .write(&key)?
        .write(&nonce)?
        .write(&plaintext.to_vec())?
        .build()?;
  
   
    let prover  = default_prover();
    let prove_info = prover.prove(env, GUEST_ELF)?;
    let receipt = prove_info.receipt; 
   
    // (Optionally) verify the proof
    receipt.verify(GUEST_ID)?;

    //  Extract and print the ciphertext
    let ct: &[u8] = &receipt.journal.bytes;
    println!("Ciphertext: {}", encode(ct));

    Ok(())
}


