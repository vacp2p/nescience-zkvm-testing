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
use risc0_zkvm::ExecutorEnv;
use risc0_zkvm::prove::{Prover, default_prover};

fn main() -> Result<()> {
    // Example inputs
    let key       = [0x42u8; 32];
    let nonce     = [0x24u8; 12];
    let plaintext = b"Hello, RISC Zero ChaCha20 demo!";

    /*
    1) Create the prover with the embedded guest code
    let mut prover: ! = Prover::new(&GUEST_ELF, &GUEST_ID)?;
    2) Supply inputs
    prover.add_input_u8_slice(&key);
    prover.add_input_u8_slice(&nonce);
    prover.add_input_u8_slice(plaintext);
    */
    let env = ExecutorEnv::builder()
        .write(&key)?
        .write(&nonce)?
        .write_slice(plaintext)
        .build()?;
  
   
    let prover  = default_prover();
    let receipt: ! = prover.prove_elf(env, GUEST_ELF)?; 
    // 3) Run, getting a Receipt (proof + journal)
    //let receipt: Receipt = prover.run()?;

    // 4) (Optionally) verify the proof
    receipt.verify(&GUEST_ID)?;

    // 5) Extract and print the ciphertext
    let ct: &[u8] = receipt.get_journal_bytes();
    println!("Ciphertext: {}", encode(ct));

    Ok(())
}


