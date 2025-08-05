//! Host-side helpers: prove encryption inside RISC Zero and verify receipts.

use risc0_zkvm::{
    default_prover, ExecutorEnv, Receipt,
};
use encryption_demo_methods::{
    CHACHA20_ELF, CHACHA20_ID
};

/// Encrypt `plaintext` with ChaCha20 inside the zkVM.
/// Returns `(ciphertext, receipt)`.
pub fn encrypt_chacha20(
    key: &[u8; 32],
    nonce: &[u8; 12],
    plaintext: &[u8],
)  {
    let env = ExecutorEnv::builder()
        .write(key).unwrap()
        .write(nonce).unwrap()
        .write(&(plaintext.len() as u32)).unwrap()
        .write(&plaintext).unwrap()
        .build().unwrap();

    let prover = default_prover();
    let prove_info = prover.prove(env, CHACHA20_ELF).unwrap();
    // receipt.verify(CHACHA20_ID)?;
    //
    // let ciphertext: Vec<u8> = receipt.journal.decode()?;
    // Ok((ciphertext, receipt))
}


